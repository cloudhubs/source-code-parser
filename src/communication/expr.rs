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
        class: Option<&ClassOrInterfaceComponent>,
        callee_method: &MethodComponent,
    ) -> Option<Node> {
        // TODO: convert call expressions that are REST or RPC calls.

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
            _ => None,
        }?;

        let client_ident = match_ident_or(&*client_call.expr)?.name.to_lowercase();
        let method_ident = match_ident_or(&*client_call.selected)?;

        // Search through the modules pertaining to the client using the client name
        let mut client_name = client_ident.replace("client", "").replace("_", "");
        println!("class??? {:?}", class);
        for field in class?.field_components.iter() {
            let field_name = field.field_name.to_lowercase();
            println!("field {}", field_name);
            if field_name.contains("client") {
                println!("before first split {}", field_name);
                let mut field_name = field_name.split("client").next()?;
                if field_name.contains("<") {
                    println!("before 2nd split {}", field_name);
                    field_name = field_name.split("<").last()?;
                }
                println!("{} -> {}", client_name, field_name);
                client_name = field_name.into();
            }
        }

        if client_ident.contains("client") && client_name.len() > 0 {
            println!(
                "looking for {} {}->{}",
                client_name, client_ident, method_ident.name
            );
            for module in modules.iter().filter(|module| {
                module
                    .component
                    .component
                    .path
                    .to_lowercase()
                    .contains(&*client_name)
            }) {
                // println!("checking moudle {}", module.component.component.path);
                // Iterate through class methods looking for a matching endpoint declarations
                for class in module.classes.iter() {
                    for method in class.component.methods.iter() {
                        // Same method name and parameter count
                        if method.method_name == method_ident.name
                            && self.args.len() == method.parameters.len()
                        {
                            // Found it
                            println!(
                                "found {} in class {} --- from {} {}->{} -- file={}",
                                method.method_name,
                                class.component.container_name,
                                client_name,
                                client_ident,
                                method_ident.name,
                                callee_method.component.path
                            );
                            // println!("comm method? {:#?}", method);
                            break;
                        }
                    }
                }
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
