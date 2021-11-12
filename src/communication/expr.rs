use super::CommunicationReplacer;
use crate::comm_repl_default_impl;
use crate::{ast::*, ClassOrInterfaceComponent};
use crate::{prophet::ModuleComponent, MethodComponent};

comm_repl_default_impl!(
    BinaryExpr,
    UnaryExpr,
    ParenExpr,
    DotExpr,
    IncDecExpr,
    LogExpr,
    Ident,
    Literal,
    IndexExpr,
    EndpointCallExpr
);

impl CommunicationReplacer for AssignExpr {
    fn replace_communication_call(
        &mut self,
        modules: &[ModuleComponent],
        module: &ModuleComponent,
        class: Option<&ClassOrInterfaceComponent>,
        method: &MethodComponent,
    ) -> Option<Node> {
        for expr in self.rhs.iter_mut() {
            if let Some(Node::Expr(replacement)) =
                expr.replace_communication_call(modules, module, class, method)
            {
                *expr = replacement;
            }
        }
        None
    }
}

impl CommunicationReplacer for CallExpr {
    fn replace_communication_call(
        &mut self,
        modules: &[ModuleComponent],
        module: &ModuleComponent,
        callee_class: Option<&ClassOrInterfaceComponent>,
        callee_method: &MethodComponent,
    ) -> Option<Node> {
        let mut endpoint_call = None;
        let map_endpoint_call = |endpoint_call: Option<EndpointCallExpr>| {
            endpoint_call.map_or_else(
                || None,
                |call| {
                    let call: Expr = call.into();
                    Some(call.into())
                },
            )
        };

        // Helper for placing an endpoint or RPC call into an Expr
        let replace_node = |arg: &mut Expr| {
            if let Some(Node::Expr(replacement)) =
                arg.replace_communication_call(modules, module, callee_class, callee_method)
            {
                *arg = replacement;
            }
        };

        // Client calls should be DotExpr's (e.g. compose_client->UploadUserId(...) in C++ or s.geoClient.Nearby(...) in Go)
        let client_call = match &*self.name {
            Expr::DotExpr(dot_expr) => Some(dot_expr),
            _ => {
                self.args.iter_mut().for_each(replace_node);
                None
            }
        }?;

        // Go a second level deep in the selector expression for Go's case
        let client_call = match &*client_call.selected {
            Expr::DotExpr(dot_expr) => Some(dot_expr),
            _ => Some(client_call),
        }?;

        // Deduce information about the service being called by the client type or alternatively
        // the client variable's naming convention
        let client_ident = match_ident_or(&*client_call.expr)?.name.to_lowercase();
        let method_ident = match_ident_or(&*client_call.selected)?;
        let client_name = get_rpc_service_name(callee_class, &client_ident);
        let client_name = match get_rest_service_name(callee_class, self.args.first()) {
            Some(rest_name) => Some(rest_name),
            None => client_name,
        };
        if client_name.is_none() {
            self.args.iter_mut().for_each(replace_node);
        }
        let client_name = client_name?;

        // Helper to see whether the endpoint has been found
        let found_rpc_endpoint =
            |class: &ClassOrInterfaceComponent, method: &MethodComponent, method_ident: &Ident| {
                // Same method name and parameter count
                let in_different_classes = match callee_class {
                    Some(callee_class) => {
                        callee_class.component.container_name != class.component.container_name
                    }
                    _ => true,
                };
                method.method_name == method_ident.name
                    && self.args.len() == method.parameters.len()
                    && callee_method.component.path != class.component.component.path
                    && in_different_classes
            };

        let found_rest_endpoint =
            |class: &ClassOrInterfaceComponent, method: &MethodComponent, client_name: &str| {
                // Same method name and parameter count
                let service_name_equiv = client_name.contains(
                    &class
                        .component
                        .container_name
                        .clone()
                        .replace("service", "")
                        .to_lowercase(),
                );

                // let base = class
                //     .annotations
                //     .iter()
                //     .find(|a| a.name.contains("RequestMapping"))
                //     .map(|a| a.value.clone());
                let tail = method
                    .annotations
                    .iter()
                    .find(|a| {
                        let name = a.name.to_lowercase();
                        name.contains("get")
                            || name.contains("post")
                            || name.contains("put")
                            || name.contains("delete")
                    })
                    .map(|a| a.value.clone());
                if tail.is_none() {
                    return false;
                }
                let tail = tail.unwrap();

                // let endpoint = format!("{}{}", base.unwrap_or("".into()), tail);

                let endpoint_config_equiv = client_name.contains(&tail.replace("/", ""));

                callee_method.component.path != class.component.component.path
                    && service_name_equiv
                    && endpoint_config_equiv
            };

        // Search through the modules pertaining to the client using the client name
        if is_communication_call(&client_ident, method_ident, &client_name) {
            for module in modules.iter() {
                for class in module.classes.iter() {
                    for method in class.component.methods.iter() {
                        if found_rpc_endpoint(class, method, method_ident)
                            || found_rest_endpoint(class, method, &client_name)
                        {
                            // Found the endpoint definition
                            endpoint_call = Some(EndpointCallExpr::new(
                                module.module_name.clone(),
                                Some(class.component.container_name.clone()),
                                method.method_name.clone(),
                                self.clone(),
                                method.component.language,
                            ));
                            break;
                        }
                    }
                    if class
                        .component
                        .container_name
                        .to_lowercase()
                        .contains(&client_name)
                        && endpoint_call.is_some()
                    {
                        // Prefer this method over any previously chosen ones because
                        // we found a closely named Service module and updated the endpoint call.
                        return map_endpoint_call(endpoint_call);
                    }
                }
            }
        }

        map_endpoint_call(endpoint_call)
    }
}

fn match_ident_or(expr: &Expr) -> Option<&Ident> {
    match expr {
        Expr::Ident(ident) => Some(ident),
        _ => None,
    }
}

fn get_rpc_service_name(
    class: Option<&ClassOrInterfaceComponent>,
    client_ident: &str,
) -> Option<String> {
    // Search through the modules pertaining to the client using the client name
    let mut client_name = client_ident
        .replace("client", "")
        .replace("pool", "")
        .replace("_", "");
    if let Some(class) = class {
        for field in class.field_components.iter() {
            // Go by type information for finding the service name. If not available
            // use naming convention.
            let field_name = if !field.r#type.is_empty() {
                field.r#type.to_lowercase()
            } else {
                field.field_name.to_lowercase()
            };

            if field_name.contains("client") {
                // Strip excess type information and unneeded naming like "client" and underscores
                let mut field_name = field_name.split("client").next()?;
                if field_name.contains('<') {
                    field_name = field_name.split('<').last()?;
                }
                client_name = field_name.replace("_", "").replace("service", "");
            }
        }
    }
    Some(client_name)
}

fn get_rest_service_name(
    class: Option<&ClassOrInterfaceComponent>,
    arg: Option<&Expr>,
) -> Option<String> {
    let config_name = get_rest_config_name(arg?)?;
    for field in class?.field_components.iter() {
        if field.field_name == config_name {
            for annotation in field.annotations.iter() {
                if annotation.name == "@Value" {
                    return Some(annotation.value.clone());
                }
            }
        }
    }
    None
}

fn get_rest_config_name(expr: &Expr) -> Option<String> {
    match expr {
        Expr::BinaryExpr(expr) => match expr.rhs.as_ref() {
            Expr::BinaryExpr(expr) => match expr.lhs.as_ref() {
                Expr::Ident(name) => Some(name.name.clone()),
                _ => None,
            },
            Expr::Ident(name) => Some(name.name.clone()),
            _ => None,
        },
        _ => None,
    }
}

fn is_communication_call(client_ident: &str, method_ident: &Ident, client_name: &str) -> bool {
    // Filter out common client utility methods that aren't endpoints
    let is_rpc_call = client_ident.contains("client")
        && !client_name.is_empty()
        && !matches!(
            &*method_ident.name.to_lowercase(),
            "push" | "pop" | "getclient"
        );

    let is_rest_call = client_ident.contains("rest") && !client_name.contains("rest");

    is_rpc_call || is_rest_call
}

impl CommunicationReplacer for InitListExpr {
    fn replace_communication_call(
        &mut self,
        modules: &[ModuleComponent],
        module: &ModuleComponent,
        class: Option<&ClassOrInterfaceComponent>,
        method: &MethodComponent,
    ) -> Option<Node> {
        for expr in self.exprs.iter_mut() {
            if let Some(Node::Expr(replacement)) =
                expr.replace_communication_call(modules, module, class, method)
            {
                *expr = replacement;
            }
        }
        None
    }
}

impl CommunicationReplacer for LambdaExpr {
    fn replace_communication_call(
        &mut self,
        modules: &[ModuleComponent],
        module: &ModuleComponent,
        class: Option<&ClassOrInterfaceComponent>,
        method: &MethodComponent,
    ) -> Option<Node> {
        self.body
            .replace_communication_call(modules, module, class, method)
    }
}

impl CommunicationReplacer for SwitchExpr {
    fn replace_communication_call(
        &mut self,
        modules: &[ModuleComponent],
        module: &ModuleComponent,
        class: Option<&ClassOrInterfaceComponent>,
        method: &MethodComponent,
    ) -> Option<Node> {
        for case in self.cases.iter_mut() {
            case.replace_communication_call(modules, module, class, method);
        }
        None
    }
}

impl CommunicationReplacer for CaseExpr {
    fn replace_communication_call(
        &mut self,
        modules: &[ModuleComponent],
        module: &ModuleComponent,
        class: Option<&ClassOrInterfaceComponent>,
        method: &MethodComponent,
    ) -> Option<Node> {
        self.body
            .replace_communication_call(modules, module, class, method)
    }
}
