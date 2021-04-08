use crate::ast::*;
use crate::java::modifier::find_modifier;
use crate::java::vartype::find_type;
use crate::ComponentInfo;
use crate::AST;

use super::util::log_unknown_tag;

/// Parse the body of a method, static block, constructor, etc.
pub(crate) fn parse_block(ast: &AST, component: &ComponentInfo) -> Block {
    Block::new(parse_child_nodes(ast, component))
}

fn parse_child_nodes(ast: &AST, component: &ComponentInfo) -> Vec<Node> {
    ast.children
        .iter()
        .map(|member| parse_node(member, component))
        .filter(|option| option.is_some())
        .flat_map(|some| some)
        .collect()
}

fn parse_node(ast: &AST, component: &ComponentInfo) -> Option<Node> {
    match &*ast.r#type {
        // Variables an initialization
        "local_variable_declaration" | "field_declaration" => {
            Some(parse_var(ast, component).into())
        }
        "variable_declarator" | "assignment_expression" => parse_assignment(ast, component),
        "identifier" => {
            let ident: Expr = Ident::new(ast.value.clone()).into();
            Some(ident.into())
        }
        "decimal_integer_literal"
        | "decimal_floating_point_literal"
        | "string_literal"
        | "false"
        | "true" => Some(Node::Expr(Expr::Literal(ast.value.clone().into()))),
        "object_creation_expression" => Some(parse_object_creation(ast, component)),
        "array_creation_expression" => {
            let ident: Expr = Ident::new("AN_ARRAY_HANDLE".into()).into();
            Some(ident.into())
        }

        // Language statements
        // "method_invocation" => Some(method_invoke(ast, component).into()),
        // "try_with_resources_statement" => Some(try_catch(ast, component).into()),

        // Base case
        unknown => {
            log_unknown_tag(unknown, "method body");
            None
        }
    }
}

// fn method_invoke(ast: &AST, component: &ComponentInfo) -> Expr {
//     let lhs: Ident;
//     let children = &ast.children;

//     for comp in ast.children.iter() {
//         match &*comp.r#type {
//             "identifier" => {
//                 lhs = Ident::new(ast.value.clone());
//             }
//             _ => {
//                 println!("{}", comp.r#type);
//             }
//         }
//     }
//     CallExpr::new(Box::new(lhs)).into()
// }

// fn try_catch(ast: &AST, component: &ComponentInfo) -> Stmt {
//     let try_body: Block;
//     let mut resources = None;

//     for comp in ast.children.iter() {
//         match &*comp.r#type {
//             "resource_specification" => {
//                 let rss = comp
//                     .children
//                     .iter()
//                     .filter(|resource| match &*resource.r#type {
//                         "resource" => true,
//                         _ => false,
//                     })
//                     .map(|resource| parse_assignment(resource, component))
//                     .collect();
//                 // resources = Some(DeclStmt::new(rss.iter().map(), expressions));
//             }
//             unknown => log_unknown_tag(unknown, "try/catch"),
//         }
//     }

//     // Generated wrappers and return
//     TryCatchStmt::new(try_body, catch_bodies).into()
// }

/// Parse an AST section containing a variable declaration
fn parse_var(ast: &AST, component: &ComponentInfo) -> Stmt {
    // Extract informtion about the variable
    let r#type = find_type(ast);
    let modifier = find_modifier(ast, &*component.path, &*component.package_name);

    // Determine the value it was set to
    let rhs = parse_child_nodes(ast, component)
        .into_iter()
        .map(|node| match node {
            Node::Expr(expr) => Some(expr),
            _ => None,
        })
        .flat_map(|expr| expr)
        .collect();

    // TODO: Use name
    let mut decl = DeclStmt::new(vec![], rhs);
    for var_decl in decl.variables.iter_mut() {
        var_decl.is_final = Some(modifier.is_final);
        var_decl.is_static = Some(modifier.is_static);
        var_decl.var_type = Some(r#type.clone());
    }
    decl.into()
}

/// Parse an assignment expression. May contain a variable declaration
fn parse_assignment(ast: &AST, component: &ComponentInfo) -> Option<Node> {
    // Define attributes
    let mut name = "";
    let mut rhs = None;

    // Find values
    for node in ast.children.iter() {
        match &*node.r#type {
            "identifier" => name = &*node.value,
            "=" => {}
            unknown => {
                if let Some(Node::Expr(parsed_rhs)) = parse_node(node, component) {
                    rhs = Some(parsed_rhs);
                } else {
                    // eprintln!(
                    //     "Encountered unknown tag {} while parsing variable assignment",
                    //     unknown
                    // );
                    rhs = None;
                }
            }
        }
    }

    // Assemble
    if let Some(rhs) = rhs {
        let bin: Expr = BinaryExpr::new(
            Box::new(Ident::new(name.into()).into()),
            "=".into(),
            Box::new(rhs),
        )
        .into();
        Some(bin.into())
    } else {
        let expr: Expr = Ident::new(name.into()).into();
        Some(expr.into())
    }
}

/// Parse instantiation using 'new'
fn parse_object_creation(ast: &AST, component: &ComponentInfo) -> Node {
    let mut name = String::new();
    let mut arg_list = vec![];
    for child in ast.children.iter() {
        match &*child.r#type {
            "type_identifier" => name = child.value.clone(),
            "argument_list" => {
                arg_list = parse_child_nodes(child, component)
                    .into_iter()
                    .map(|node| match node {
                        Node::Expr(expr) => Some(expr),
                        _ => None,
                    })
                    .flat_map(|expr| expr)
                    .collect()
            }
            _ => {}
        }
    }

    // Create ident
    let ident: Expr = CallExpr::new(Box::new(Ident::new(name).into()), arg_list).into();
    ident.into()
}
