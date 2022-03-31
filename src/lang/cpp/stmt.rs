use super::*;
use crate::ast::*;
use crate::parse::AST;
use Language::Cpp;

/// Takes child of compound_statement and turns it into a Node
pub fn body_node(node: &AST) -> Option<Node> {
    match &*node.r#type {
        "declaration" => {
            let decl: Stmt = variable_declaration(node).into();
            Some(decl.into())
        }
        "while_statement" => {
            let while_stmt: Stmt = while_statement(node)?.into();
            Some(while_stmt.into())
        }
        "for_statement" => {
            let for_stmt: Stmt = for_statement(node)?.into();
            Some(for_stmt.into())
        }
        "for_range_loop" => {
            let for_range_stmt: Stmt = for_range_statement(node)?.into();
            Some(for_range_stmt.into())
        }
        "if_statement" => {
            let if_stmt: Stmt = if_statement(node)?.into();
            Some(if_stmt.into())
        }
        "switch_statement" => {
            let switch_stmt: Stmt = switch_statement(node)?.into();
            Some(switch_stmt.into())
        }
        "expression_statement" => {
            let stmt: Stmt = expression_statement(node)?;
            Some(stmt.into())
        }
        "using_declaration" => {
            let using: Stmt = import_statement(node)?.into();
            Some(using.into())
        }
        "return_statement" => {
            let ret: Stmt = return_statement(node).into();
            Some(ret.into())
        }
        "break_statement" => {
            let brk: Stmt = BreakStmt::new(Cpp).into();
            Some(brk.into())
        }
        "continue_statment" => {
            let cont: Stmt = ContinueStmt::new(Cpp).into();
            Some(cont.into())
        }
        "throw_statement" => {
            let throw: Stmt = throw_statement(node).into();
            Some(throw.into())
        }
        "try_statement" => {
            let try_catch: Stmt = try_catch_statement(node)?.into();
            Some(try_catch.into())
        }
        "compound_statement" => {
            let nodes = block_nodes(node);
            let block = Block::new(nodes, Cpp);
            Some(block.into())
        }
        // ...
        _ => {
            let expr: Stmt = expression(node)?.into();
            Some(expr.into())
        }
    }
}

/// Takes in node type "declaration" and converts it to a DeclStmt
pub fn variable_declaration(node: &AST) -> DeclStmt {
    let mut variable_type = node
        .find_child_by_type(&[
            "primitive_type",
            "scoped_type_identifier",
            "type_identifier",
            "template_type",
            "auto",
        ])
        .map_or_else(|| "".into(), type_ident);

    let init_declarator =
        node.find_child_by_type(&["init_declarator", "function_declarator", "array_declarator"]);
    match init_declarator {
        Some(init_declarator) => match &*init_declarator.r#type {
            "init_declarator" | "function_declarator" => {
                variable_init_declaration(init_declarator, variable_type)
            }
            "array_declarator" => {
                let (variables, expressions) = match expression(init_declarator) {
                    Some(index_expr) => match index_expr {
                        Expr::IndexExpr(index_expr) => match *index_expr.expr.clone() {
                            Expr::Ident(ident) => (
                                vec![VarDecl::new(Some(variable_type), ident, Cpp)],
                                vec![Some(index_expr.into())], // TODO add multiple variables on 1 line parsing
                            ),
                            _ => (vec![], vec![Some(index_expr.into())]), // TODO add multiple variables on 1 line parsing
                        },
                        _ => (vec![], vec![]),
                    },
                    None => {
                        tracing::warn!("Malformed array_declarator {:#?}", init_declarator);
                        (vec![], vec![])
                    }
                };
                DeclStmt::new(variables, expressions, Cpp)
            }
            _ => unreachable!(),
        },
        None => {
            let name = variable_ident(node, &mut variable_type).expect(&*format!(
                "No variable name for declaration with no init {:#?}",
                node,
            ));
            let ident = Ident::new(name, Cpp);
            DeclStmt::new(
                vec![VarDecl::new(Some(variable_type), ident, Cpp)],
                vec![],
                Cpp,
            )
        }
    }
}

fn variable_init_declaration(init_declarator: &AST, mut variable_type: String) -> DeclStmt {
    let name = variable_ident(init_declarator, &mut variable_type).map_or_else(
        || {
            tracing::warn!("No identifier for init declarator {:#?}", init_declarator);
            "".into()
        },
        |name| name,
    );
    let decl_type = init_declarator.find_child_by_type(&["=", "argument_list", "parameter_list"]);
    let rhs = match decl_type {
        Some(decl_type) => match &*decl_type.r#type {
            "=" => {
                let value = init_declarator.children.iter().next_back();
                value.map_or_else(|| None, expression)
            }
            "argument_list" | "parameter_list" => {
                let argument_list = decl_type;
                let args: Vec<Expr> = argument_list
                    .children
                    .iter()
                    .map(expression)
                    .flatten()
                    .collect();
                let new = Literal::new("new".to_string(), Cpp);
                let init = CallExpr::new(Box::new(new.into()), args, Cpp).into();
                Some(init)
            }
            _ => None,
        },
        None => None,
    };
    let ident = Ident::new(name, Cpp);
    let var_decl = VarDecl::new(Some(variable_type), ident, Cpp);
    let rhs = vec![rhs]; // TODO add multiple variables on 1 line parsing
    DeclStmt::new(vec![var_decl], rhs, Cpp)
}

fn expression_statement(node: &AST) -> Option<Stmt> {
    let expr = node.children.first().map_or_else(|| None, expression)?;
    Some(expr.into())
}

fn import_statement(using_declaration: &AST) -> Option<ImportStmt> {
    let ident = using_declaration
        .find_child_by_type(&["namespace_identifier", "scoped_namespace_identifier"])?;
    let using = type_ident(ident);
    Some(ImportStmt::new(false, false, using, Cpp))
}

fn return_statement(return_stmt: &AST) -> ReturnStmt {
    // If there isn't an expression and the 2nd child is of type ";",
    // the expression function will return None anyways.
    let expr = return_stmt.children.get(1).map(expression).flatten();
    ReturnStmt::new(expr, Cpp)
}

fn throw_statement(throw_stmt: &AST) -> ThrowStmt {
    match throw_stmt.children.get(1) {
        Some(expr) => {
            let expr = expression(expr);
            ThrowStmt::new(expr, Cpp)
        }
        None => ThrowStmt::new(None, Cpp),
    }
}

fn try_catch_statement(try_catch_stmt: &AST) -> Option<TryCatchStmt> {
    let mut children = try_catch_stmt.children.iter();
    let body = Block::new(block_nodes(children.nth(1)?), Cpp);
    let catch_bodies = children.map(catch_statement).flatten().collect();
    Some(TryCatchStmt::new(body, catch_bodies, None, Cpp))
}

fn if_statement(if_stmt: &AST) -> Option<IfStmt> {
    let cond = if_stmt
        .find_child_by_type(&["condition_clause"])
        .map(expression)??;
    let mut blocks = if_stmt
        .children
        .iter()
        .filter(|node| &*node.r#type == "compound_statement")
        .map(|block| Block::new(block_nodes(block), Cpp));
    let body = match blocks.next() {
        Some(block) => block,
        None => {
            let stmt = if_stmt.children.iter().last()?;
            let stmt = body_node(stmt)?;
            Block::new(vec![stmt], Cpp)
        }
    };
    // Check for else block, if else block, or no else block.
    let else_body = match blocks.next() {
        Some(else_body) => Some(else_body),
        None => {
            let else_if = if_stmt
                .find_child_by_type(&["if_statement"])
                .map(if_statement)
                .flatten()
                .map(|if_stmt| {
                    let if_stmt: Stmt = if_stmt.into();
                    Block::new(vec![if_stmt.into()], Cpp)
                });
            else_if
        }
    };
    Some(IfStmt::new(cond, body, else_body, Cpp))
}

fn switch_statement(switch_stmt: &AST) -> Option<SwitchExpr> {
    let cond = switch_stmt
        .find_child_by_type(&["condition_clause"])
        .map(expression)??;
    let cases = switch_stmt
        .find_child_by_type(&["compound_statement"])
        .map(|switch_stmt| switch_stmt.children.iter())?
        .flat_map(switch_case)
        .collect();

    let switch_stmt = SwitchExpr::new(Box::new(cond), cases, Cpp);
    Some(switch_stmt)
}

fn switch_case(case_statement: &AST) -> Option<CaseExpr> {
    let expr = case_statement.find_child_by_type(&["case", "default"])?;
    // todo: add literals to expression function
    let expr = match &*expr.r#type {
        "case" => expression(case_statement.children.get(1)?),
        // "default"
        _ => None,
    };
    if case_statement.children.len() < 4 {
        tracing::info!("Malformed case statement {:#?}", case_statement);
        return None;
    }
    let nodes = block_nodes_iter(&case_statement.children[3..]);
    let block = Block::new(nodes, Cpp);
    let case = CaseExpr::new(expr.map(Box::new), Box::new(block), Cpp);
    Some(case)
}

fn while_statement(while_stmt: &AST) -> Option<WhileStmt> {
    let cond = while_stmt
        .find_child_by_type(&["condition_clause"])
        .map(expression)??;
    let nodes = while_stmt
        .find_child_by_type(&["compound_statement"])
        .map(block_nodes)?;
    Some(WhileStmt::new(cond, Block::new(nodes, Cpp), Cpp))
}

fn for_statement(for_stmt: &AST) -> Option<ForStmt> {
    let block = for_stmt.find_child_by_type(&["compound_statement"])?;
    let block = func_body(block);

    let mut init = None;
    let mut cond = None;
    let mut post = None;
    let mut semicolons = 0u8;
    for part in for_stmt
        .children
        .iter()
        .filter(|child| !matches!(&*child.r#type, "for" | "(" | ")" | "compound_statement"))
    {
        if &*part.r#type == ";" {
            semicolons += 1;
        } else {
            match semicolons {
                // Declarations need to be considered for the initialization. Regular BinExpr
                // are treated as ExprStmt here.
                0 => init = body_node(part),
                1 => cond = expression(part),
                2 => post = expression(part),
                _ => {}
            }
        }
    }

    // Convert generic node to statement
    let init = match init {
        Some(Node::Stmt(stmt)) => Some(Box::new(stmt)),
        _ => None,
    };

    let for_stmt = ForStmt::new(
        init.map_or_else(Vec::new, |init| vec![*init]),
        cond,
        post.map_or_else(Vec::new, |post| vec![post]),
        block,
        Cpp,
    );
    Some(for_stmt)
}

fn for_range_statement(for_range_loop: &AST) -> Option<ForRangeStmt> {
    let block = for_range_loop.find_child_by_type(&["compound_statement"])?;
    let block = func_body(block);

    let mut r#type = None;
    let mut decl = None;
    let mut iterator = None;
    for (i, part) in for_range_loop
        .children
        .iter()
        .filter(|child| {
            !matches!(
                &*child.r#type,
                "for" | "(" | ":" | ")" | "compound_statement"
            )
        })
        .enumerate()
    {
        match i {
            // Declarations need to be considered for the initialization. Regular BinExpr
            // are treated as ExprStmt here.
            0 => r#type = expression(part),
            1 => decl = expression(part),
            2 => iterator = expression(part),
            _ => {}
        }
    }

    // Convert generic node to statement
    let mut r#type = match r#type {
        Some(Expr::Literal(t)) => t.value,
        Some(Expr::Ident(ident)) => ident.name,
        _ => "".into(),
    };
    let mut is_unary = true;
    while is_unary {
        match decl {
            Some(Expr::UnaryExpr(expr)) => {
                r#type.push_str(match expr.op {
                    Op::Star => "*",
                    Op::And => "&",
                    _ => "",
                });
                decl = Some(*expr.expr);
                is_unary = true;
            }
            _ => is_unary = false,
        }
    }
    // let decl = DeclStmt::new(Some(r#type), vec![decl?]);
    let variables = match decl {
        Some(Expr::Ident(ident)) => vec![VarDecl::new(Some(r#type), ident, Cpp)],
        _ => vec![],
    };
    let decl = DeclStmt::new(variables, vec![], Cpp);
    let for_range_stmt = ForRangeStmt::new(Box::new(decl.into()), iterator, block, Cpp);
    Some(for_range_stmt)
}

fn catch_statement(catch_clause: &AST) -> Option<CatchStmt> {
    let params = catch_clause.find_child_by_type(&["parameter_list"])?;
    let decl = params
        .children
        .get(1)
        .map(|decl| match body_node(decl) {
            Some(Node::Stmt(Stmt::DeclStmt(decl))) => Some(decl),
            _ => None,
        })
        .flatten()
        .unwrap_or_else(|| DeclStmt::new(vec![], vec![], Cpp));
    let body = Block::new(
        block_nodes(catch_clause.find_child_by_type(&["compound_statement"])?),
        Cpp,
    );
    Some(CatchStmt::new(decl, body, Cpp))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn return_stmt_test() {
        let ast = AST {
            children: vec![
                AST {
                    children: vec![],
                    span: None,
                    r#type: "return".to_string(),
                    value: "return".to_string(),
                },
                AST {
                    children: vec![
                        AST {
                            children: vec![],
                            span: None,
                            r#type: "!".to_string(),
                            value: "!".to_string(),
                        },
                        AST {
                            children: vec![
                                AST {
                                    children: vec![],
                                    span: None,
                                    r#type: "(".to_string(),
                                    value: "(".to_string(),
                                },
                                AST {
                                    children: vec![
                                        AST {
                                            children: vec![
                                                AST {
                                                    children: vec![],
                                                    span: None,
                                                    r#type: "*".to_string(),
                                                    value: "*".to_string(),
                                                },
                                                AST {
                                                    children: vec![],
                                                    span: None,
                                                    r#type: "this".to_string(),
                                                    value: "this".to_string(),
                                                },
                                            ],
                                            span: None,
                                            r#type: "pointer_expression".to_string(),
                                            value: "".to_string(),
                                        },
                                        AST {
                                            children: vec![],
                                            span: None,
                                            r#type: "==".to_string(),
                                            value: "==".to_string(),
                                        },
                                        AST {
                                            children: vec![],
                                            span: None,
                                            r#type: "identifier".to_string(),
                                            value: "rhs".to_string(),
                                        },
                                    ],
                                    span: None,
                                    r#type: "binary_expression".to_string(),
                                    value: "".to_string(),
                                },
                                AST {
                                    children: vec![],
                                    span: None,
                                    r#type: ")".to_string(),
                                    value: ")".to_string(),
                                },
                            ],
                            span: None,
                            r#type: "parenthesized_expression".to_string(),
                            value: "".to_string(),
                        },
                    ],
                    span: None,
                    r#type: "unary_expression".to_string(),
                    value: "".to_string(),
                },
                AST {
                    children: vec![],
                    span: None,
                    r#type: ";".to_string(),
                    value: ";".to_string(),
                },
            ],
            span: None,
            r#type: "return_statement".to_string(),
            value: "".to_string(),
        };

        let expected: Stmt = ReturnStmt::new(
            Some(
                UnaryExpr::new(
                    Box::new(
                        ParenExpr::new(
                            Box::new(
                                BinaryExpr::new(
                                    Box::new(
                                        UnaryExpr::new(
                                            Box::new(Expr::Literal(Literal::new(
                                                "this".into(),
                                                Cpp,
                                            ))),
                                            Op::Star,
                                            Cpp,
                                        )
                                        .into(),
                                    ),
                                    Op::EqualEqual,
                                    Box::new(Ident::new("rhs".into(), Cpp).into()),
                                    Cpp,
                                )
                                .into(),
                            ),
                            Cpp,
                        )
                        .into(),
                    ),
                    Op::ExclamationPoint,
                    Cpp,
                )
                .into(),
            ),
            Cpp,
        )
        .into();
        let expected: Node = expected.into();

        let actual = body_node(&ast).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn init_declaration_test() {
        let ast = AST {
            children: vec![
                AST {
                    children: vec![],
                    span: None,
                    r#type: "primitive_type".to_string(),
                    value: "uint32_t".to_string(),
                },
                AST {
                    children: vec![
                        AST {
                            children: vec![],
                            span: None,
                            r#type: "identifier".to_string(),
                            value: "xfer".to_string(),
                        },
                        AST {
                            children: vec![],
                            span: None,
                            r#type: "=".to_string(),
                            value: "=".to_string(),
                        },
                        AST {
                            children: vec![],
                            span: None,
                            r#type: "number_literal".to_string(),
                            value: "0".to_string(),
                        },
                    ],
                    span: None,
                    r#type: "init_declarator".to_string(),
                    value: "".to_string(),
                },
                AST {
                    children: vec![],
                    span: None,
                    r#type: ";".to_string(),
                    value: ";".to_string(),
                },
            ],
            span: None,
            r#type: "declaration".to_string(),
            value: "".to_string(),
        };

        let expected: Stmt = DeclStmt::new(
            vec![VarDecl::new(
                Some("uint32_t".into()),
                Ident::new("xfer".into(), Cpp),
                Cpp,
            )],
            vec![Some(Literal::new("0".into(), Cpp).into())], // TODO add multiple variables on 1 line parsing
            Cpp,
        )
        .into();
        let expected: Node = expected.into();

        let actual = body_node(&ast).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn empty_for_loop_test() {
        let ast = AST {
            children: vec![
                AST {
                    children: vec![],
                    span: None,
                    r#type: "for".to_string(),
                    value: "for".to_string(),
                },
                AST {
                    children: vec![],
                    span: None,
                    r#type: "(".to_string(),
                    value: ")".to_string(),
                },
                AST {
                    children: vec![
                        AST {
                            children: vec![],
                            span: None,
                            r#type: "identifier".to_string(),
                            value: "_i284".to_string(),
                        },
                        AST {
                            children: vec![],
                            span: None,
                            r#type: "=".to_string(),
                            value: "=".to_string(),
                        },
                        AST {
                            children: vec![],
                            span: None,
                            r#type: "number_literal".to_string(),
                            value: "0".to_string(),
                        },
                    ],
                    span: None,
                    r#type: "assignment_expression".to_string(),
                    value: "".to_string(),
                },
                AST {
                    children: vec![],
                    span: None,
                    r#type: ";".to_string(),
                    value: ";".to_string(),
                },
                AST {
                    children: vec![
                        AST {
                            children: vec![],
                            span: None,
                            r#type: "identifier".to_string(),
                            value: "_i284".to_string(),
                        },
                        AST {
                            children: vec![],
                            span: None,
                            r#type: "<".to_string(),
                            value: "<".to_string(),
                        },
                        AST {
                            children: vec![],
                            span: None,
                            r#type: "identifier".to_string(),
                            value: "_size280".to_string(),
                        },
                    ],
                    span: None,
                    r#type: "binary_expression".to_string(),
                    value: "".to_string(),
                },
                AST {
                    children: vec![],
                    span: None,
                    r#type: ";".to_string(),
                    value: ";".to_string(),
                },
                AST {
                    children: vec![
                        AST {
                            children: vec![],
                            span: None,
                            r#type: "++".to_string(),
                            value: "++".to_string(),
                        },
                        AST {
                            children: vec![],
                            span: None,
                            r#type: "identifier".to_string(),
                            value: "_i284".to_string(),
                        },
                    ],
                    span: None,
                    r#type: "update_expression".to_string(),
                    value: "".to_string(),
                },
                AST {
                    children: vec![],
                    span: None,
                    r#type: ")".to_string(),
                    value: ")".to_string(),
                },
                AST {
                    children: vec![],
                    span: None,
                    r#type: "compound_statement".to_string(),
                    value: "".to_string(),
                },
            ],
            span: None,
            r#type: "for_statement".to_string(),
            value: "".to_string(),
        };

        let init: Expr = AssignExpr::new(
            vec![Ident::new("_i284".into(), Cpp).into()],
            vec![Expr::Literal(Literal::new("0".into(), Cpp))],
            Cpp,
        )
        .into();
        let init: Stmt = init.into();

        let expected: Stmt = ForStmt::new(
            vec![init],
            Some(
                BinaryExpr::new(
                    Box::new(Ident::new("_i284".into(), Cpp).into()),
                    Op::LessThan,
                    Box::new(Ident::new("_size280".into(), Cpp).into()),
                    Cpp,
                )
                .into(),
            ),
            vec![IncDecExpr::new(
                true,
                true,
                Box::new(Ident::new("_i284".into(), Cpp).into()),
                Cpp,
            )
            .into()],
            Block::new(vec![], Cpp),
            Cpp,
        )
        .into();
        let expected: Node = expected.into();

        let actual = body_node(&ast).unwrap();

        assert_eq!(expected, actual);
    }
}
