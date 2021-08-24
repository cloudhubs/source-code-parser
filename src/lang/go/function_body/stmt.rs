use crate::ast::*;
use crate::ComponentInfo;
use crate::AST;

use crate::go::util::vartype::find_type;


//use super::{expr::parse_expr, is_common_junk_tag, node::parse_node};
use super::node::parse_node;
use crate::go::function_body::node::parse_child_nodes;

/// Parse an AST section containing a variable declaration
pub(crate) fn parse_decl(ast: &AST, component: &ComponentInfo) -> DeclStmt {
    // Extract informtion about the variable
    let ast = match ast.find_child_by_type(&["var_spec"]) {
        Some(var) => var,
        None => ast
    };
    let r#type = find_type(ast);



    // Determine the value it was set to
    let rhs = parse_child_nodes(ast, component);

    let mut decl = DeclStmt::new(vec![], vec![]);
    for var in rhs.iter() {
        // Extract expression from the hierarchy
        let base = match var {
            Node::Stmt(Stmt::ExprStmt(ExprStmt { expr, .. })) | Node::Expr(expr) => expr,
            _ => {
                eprintln!("Unable to interpret as variable: {:#?}", var);
                continue;
            }
        };

        // Parse variable
        match base {
            Expr::BinaryExpr(expr) => match expr.lhs.as_ref() {
                Expr::Ident(lhs) => {
                    decl.variables
                        .push(VarDecl::new(Some(r#type.clone()), lhs.clone()));
                    decl.expressions.push(Some(expr.rhs.as_ref().clone()));
                }
                unknown => eprintln!("Expected Ident got {:#?}", unknown),
            },
            Expr::Ident(id) => {
                decl.variables
                    .push(VarDecl::new(Some(r#type.clone()), id.clone()));
                decl.expressions.push(None);
            }
            unknown => {
                eprintln!("Expected BinaryExpr or Ident, got {:#?}", unknown);
            }
        }
    }


    for var_decl in decl.variables.iter_mut() {
        var_decl.var_type = Some(r#type.clone());
    }
    decl.into()
}