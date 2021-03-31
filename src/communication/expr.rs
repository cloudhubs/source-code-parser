use super::CommunicationReplacer;
use crate::comm_repl_default_impl;
use crate::{ast::*, ClassOrInterfaceComponent};
use crate::{prophet::ModuleComponent, MethodComponent};

comm_repl_default_impl!(
    BinaryExpr, UnaryExpr, ParenExpr, DotExpr, IncDecExpr, LogExpr, Ident, Literal, IndexExpr
);

impl CommunicationReplacer for AssignExpr {
    fn replace_communication_call(
        &mut self,
        modules: &Vec<ModuleComponent>,
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
        modules: &Vec<ModuleComponent>,
        module: &ModuleComponent,
        callee_class: Option<&ClassOrInterfaceComponent>,
        callee_method: &MethodComponent,
    ) -> Option<Node> {
        // TODO: convert found call expressions that are REST or RPC calls.

        // Get name -- could be Ident, or DotExpr where the left hand is the identifier.
        // For REST and RPC calls, it should be a DotExpr most likely since it will be using
        // some kind of Client object to actually make the calls.

        // Use the identifier from the DotExpr and look for a declaration with that same name.
        // This could be in the module itself (unlikely???), in the class definition's fields,
        // or as a local variable defined in your current function.

        // Once we have the declaration we can try to use the type information to guess if it
        // is an RPC or REST call or a normal call expression.

        let client_call = match &*self.name {
            Expr::DotExpr(dot_expr) => Some(dot_expr),
            _ => {
                self.args.iter_mut().for_each(|arg| {
                    // println!("arg");
                    if let Some(Node::Expr(replacement)) =
                        arg.replace_communication_call(modules, module, callee_class, callee_method)
                    {
                        *arg = replacement;
                    }
                });
                None
            }
        }?;

        let client_ident = match_ident_or(&*client_call.expr)?.name.to_lowercase();
        let method_ident = match_ident_or(&*client_call.selected)?;
        let client_name = get_service_name(callee_class, &client_ident);
        if client_name.is_none() {
            self.args.iter_mut().for_each(|arg| {
                if let Some(Node::Expr(replacement)) =
                    arg.replace_communication_call(modules, module, callee_class, callee_method)
                {
                    *arg = replacement;
                }
            });
        }
        let client_name = client_name?;

        // Search through the modules pertaining to the client using the client name
        if is_communication_call(&client_ident, method_ident, &client_name) {
            let mut done = false;
            let mut result = None;
            for module in modules.iter() {
                for class in module.classes.iter() {
                    for method in class.component.methods.iter() {
                        // Same method name and parameter count
                        if method.method_name == method_ident.name
                            && self.args.len() == method.parameters.len()
                            && callee_method.component.path != class.component.component.path
                            && match callee_class {
                                Some(callee_class) => {
                                    callee_class.component.container_name
                                        != class.component.container_name
                                }
                                _ => true,
                            }
                        {
                            // Found it
                            result = Some(format!(
                                "found {} in class {} --- from {} {}->{} -- from file={}",
                                method.method_name,
                                class.component.container_name,
                                client_name,
                                client_ident,
                                method_ident.name,
                                callee_method.component.path
                            ));
                            break;
                        }
                    }
                    if class
                        .component
                        .container_name
                        .to_lowercase()
                        .contains(&client_name)
                        && result.is_some()
                    {
                        // prefer this method over any previously chosen ones.
                        done = true;
                        break;
                    }
                }
                if done {
                    break;
                }
            }
            if result.is_some() {
                println!("{}", result?);
            }
        }

        None
    }
}

fn match_ident_or(expr: &Expr) -> Option<&Ident> {
    match expr {
        Expr::Ident(ident) => Some(ident),
        _ => None,
    }
}

fn get_service_name(
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
            let field_name = if field.r#type != "" {
                field.r#type.to_lowercase()
            } else {
                field.field_name.to_lowercase()
            };
            if field_name.contains("client") {
                // println!("before first split {}", field_name);
                let mut field_name = if field_name.contains("client>") {
                    field_name.split("client>").next()?
                } else {
                    field_name.split("client").next()?
                };
                if field_name.contains("<") {
                    // println!("before 2nd split {}", field_name);
                    field_name = field_name.split("<").last()?;
                }
                // println!("{} -> {}", client_name, field_name);
                client_name = field_name.replace("_", "").replace("service", "").into();
            }
        }
    }
    Some(client_name)
}

fn is_communication_call(client_ident: &str, method_ident: &Ident, client_name: &str) -> bool {
    client_ident.contains("client")
        && client_name.len() > 0
        && match &*method_ident.name.to_lowercase() {
            "push" | "pop" | "getclient" => false,
            _ => true,
        }
}

impl CommunicationReplacer for InitListExpr {
    fn replace_communication_call(
        &mut self,
        modules: &Vec<ModuleComponent>,
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
        modules: &Vec<ModuleComponent>,
        module: &ModuleComponent,
        class: Option<&ClassOrInterfaceComponent>,
        method: &MethodComponent,
    ) -> Option<Node> {
        self.body
            .replace_communication_call(modules, module, class, method)
    }
}
