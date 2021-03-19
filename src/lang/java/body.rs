use crate::ast::*;
use crate::java::modifier::find_modifier;
use crate::java::vartype::find_type;
use crate::ComponentInfo;
use crate::AST;

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
        "local_variable_declaration" | "field_declaration" => {
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
            let decl: Stmt = decl.into();
            Some(decl.into())
        }

        "variable_declarator" | "assignment_expression" => {
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
                            eprintln!(
                                "Encountered unknown tag {} while parsing variable assignment",
                                unknown
                            );
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

        "identifier" => {
            let ident: Expr = Ident::new(ast.value.clone()).into();
            Some(ident.into())
        }

        "decimal_integer_literal"
        | "decimal_floating_point_literal"
        | "string_literal"
        | "false"
        | "true" => Some(Node::Expr(Expr::Literal(ast.value.clone().into()))),

        "object_creation_expression" => {
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
            Some(ident.into())
        }

        "array_creation_expression" => {
            let ident: Expr = Ident::new("AN_ARRAY_HANDLE".into()).into();
            Some(ident.into())
        }

        unknown => {
            eprintln!("{} unknown tag in parsing method body!", unknown);
            None
        }
    }
}
