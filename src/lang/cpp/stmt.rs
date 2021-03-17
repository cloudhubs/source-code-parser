use super::*;
use crate::ast::*;
use crate::parse::AST;

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
            let brk: Stmt = BreakStmt::new().into();
            Some(brk.into())
        }
        "continue_statment" => {
            let cont: Stmt = ContinueStmt::new().into();
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
            let block = Block::new(nodes);
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
        .map_or_else(|| "".into(), |node| type_ident(node));

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
                                vec![VarDecl::new(Some(variable_type), ident)],
                                vec![index_expr.into()],
                            ),
                            _ => (vec![], vec![index_expr.into()]),
                        },
                        _ => (vec![], vec![]),
                    },
                    None => {
                        eprintln!("Malformed array_declarator {:#?}", init_declarator);
                        (vec![], vec![])
                    }
                };
                DeclStmt::new(variables, expressions)
            }
            _ => unreachable!(),
        },
        None => {
            let name = variable_ident(node, &mut variable_type).expect(&*format!(
                "No variable name for declaration with no init {:#?}",
                node,
            ));
            let ident = Ident::new(name);
            DeclStmt::new(vec![VarDecl::new(Some(variable_type), ident)], vec![])
        }
    }
}

fn variable_init_declaration(init_declarator: &AST, mut variable_type: String) -> DeclStmt {
    let name = variable_ident(init_declarator, &mut variable_type).map_or_else(
        || {
            eprintln!("No identifier for init declarator {:#?}", init_declarator);
            "".into()
        },
        |name| name,
    );
    let decl_type = init_declarator.find_child_by_type(&["=", "argument_list", "parameter_list"]);
    let rhs = match decl_type {
        Some(decl_type) => match &*decl_type.r#type {
            "=" => {
                let value = init_declarator.children.iter().next_back();
                value.map_or_else(|| None, |value| expression(value))
            }
            "argument_list" | "parameter_list" => {
                let argument_list = decl_type;
                let args: Vec<Expr> = argument_list
                    .children
                    .iter()
                    .map(|arg| expression(arg))
                    .flat_map(|arg| arg)
                    .collect();
                let new: Literal = "new".to_string().into();
                let init = CallExpr::new(Box::new(new.into()), args).into();
                Some(init)
            }
            _ => None,
        },
        None => None,
    };
    let ident = Ident::new(name);
    let var_decl = VarDecl::new(Some(variable_type), ident);
    let rhs = match rhs {
        Some(rhs) => vec![rhs],
        None => vec![],
    };
    DeclStmt::new(vec![var_decl], rhs)
}

fn expression_statement(node: &AST) -> Option<Stmt> {
    let expr = node
        .children
        .iter()
        .next()
        .map_or_else(|| None, |node| expression(node))?;
    Some(expr.into())
}

fn import_statement(using_declaration: &AST) -> Option<ImportStmt> {
    let ident = using_declaration
        .find_child_by_type(&["namespace_identifier", "scoped_namespace_identifier"])?;
    let using = type_ident(ident);
    Some(ImportStmt::new(false, false, using))
}

fn return_statement(return_stmt: &AST) -> ReturnStmt {
    // If there isn't an expression and the 2nd child is of type ";",
    // the expression function will return None anyways.
    let expr = return_stmt
        .children
        .iter()
        .nth(1)
        .map(|expr| expression(expr))
        .flatten();
    ReturnStmt::new(expr)
}

fn throw_statement(throw_stmt: &AST) -> ThrowStmt {
    match throw_stmt.children.iter().nth(1) {
        Some(expr) => {
            let expr = expression(expr);
            ThrowStmt::new(expr)
        }
        None => ThrowStmt::new(None),
    }
}

fn try_catch_statement(try_catch_stmt: &AST) -> Option<TryCatchStmt> {
    let mut children = try_catch_stmt.children.iter();
    let body = Block::new(block_nodes(children.nth(1)?));
    let catch_bodies = children
        .map(|catch| catch_statement(catch))
        .filter_map(|catch| catch)
        .collect();
    Some(TryCatchStmt::new(body, catch_bodies))
}

fn if_statement(if_stmt: &AST) -> Option<IfStmt> {
    let cond = if_stmt
        .find_child_by_type(&["condition_clause"])
        .map(|cond| expression(cond))??;
    let mut blocks = if_stmt
        .children
        .iter()
        .filter(|node| &*node.r#type == "compound_statement")
        .map(|block| Block::new(block_nodes(block)));
    let body = match blocks.next() {
        Some(block) => block,
        None => {
            let stmt = if_stmt.children.iter().last()?;
            let stmt = body_node(stmt)?;
            Block::new(vec![stmt])
        }
    };
    // Check for else block, if else block, or no else block.
    let else_body = match blocks.next() {
        Some(else_body) => Some(else_body),
        None => {
            let else_if = if_stmt
                .find_child_by_type(&["if_statement"])
                .map(|if_stmt| if_statement(if_stmt))
                .flatten()
                .map(|if_stmt| {
                    let if_stmt: Stmt = if_stmt.into();
                    Block::new(vec![if_stmt.into()])
                });
            else_if
        }
    };
    Some(IfStmt::new(cond, body, else_body))
}

fn switch_statement(switch_stmt: &AST) -> Option<SwitchStmt> {
    let cond = switch_stmt
        .find_child_by_type(&["condition_clause"])
        .map(|cond| expression(cond))??;
    let cases = switch_stmt
        .find_child_by_type(&["compound_statement"])
        .map(|switch_stmt| switch_stmt.children.iter())?
        .map(|case| switch_case(case))
        .flat_map(|case| case)
        .collect();

    let switch_stmt = SwitchStmt::new(cond, cases);
    Some(switch_stmt)
}

fn switch_case(case_statement: &AST) -> Option<CaseStmt> {
    let expr = case_statement.find_child_by_type(&["case", "default"])?;
    // todo: add literals to expression function
    let expr = match &*expr.r#type {
        "case" => expression(case_statement.children.iter().nth(1)?),
        "default" | _ => None,
    };
    if case_statement.children.len() < 4 {
        println!("Malformed case statement {:#?}", case_statement);
        return None;
    }
    let nodes = block_nodes_iter(&case_statement.children[3..]);
    let block = Block::new(nodes);
    let case = CaseStmt::new(expr, block);
    Some(case.into())
}

fn while_statement(while_stmt: &AST) -> Option<WhileStmt> {
    let cond = while_stmt
        .find_child_by_type(&["condition_clause"])
        .map(|cond| expression(cond))??;
    let nodes = while_stmt
        .find_child_by_type(&["compound_statement"])
        .map(|block| block_nodes(block))?;
    Some(WhileStmt::new(cond, Block::new(nodes)))
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
        .filter(|child| match &*child.r#type {
            "for" | "(" | ")" | "compound_statement" => false,
            _ => true,
        })
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

    let for_stmt = ForStmt::new(init, cond, post, block);
    Some(for_stmt)
}

fn for_range_statement(for_range_loop: &AST) -> Option<ForRangeStmt> {
    let block = for_range_loop.find_child_by_type(&["compound_statement"])?;
    let block = func_body(block);

    let mut i = 0;
    let mut r#type = None;
    let mut decl = None;
    let mut iterator = None;
    for part in for_range_loop
        .children
        .iter()
        .filter(|child| match &*child.r#type {
            "for" | "(" | ":" | ")" | "compound_statement" => false,
            _ => true,
        })
    {
        match i {
            // Declarations need to be considered for the initialization. Regular BinExpr
            // are treated as ExprStmt here.
            0 => r#type = expression(part),
            1 => decl = expression(part),
            2 => iterator = expression(part),
            _ => {}
        }
        i += 1;
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
        Some(Expr::Ident(ident)) => vec![VarDecl::new(Some(r#type), ident)],
        _ => vec![],
    };
    let decl = DeclStmt::new(variables, vec![]);
    let for_range_stmt = ForRangeStmt::new(Box::new(decl.into()), iterator, block);
    Some(for_range_stmt)
}

fn catch_statement(catch_clause: &AST) -> Option<CatchStmt> {
    let params = catch_clause.find_child_by_type(&["parameter_list"])?;
    let decl = params
        .children
        .iter()
        .nth(1)
        .map(|decl| match body_node(decl) {
            Some(Node::Stmt(Stmt::DeclStmt(decl))) => Some(decl),
            _ => None,
        })
        .flatten()
        .unwrap_or(DeclStmt::new(vec![], vec![]));
    let body = Block::new(block_nodes(
        catch_clause.find_child_by_type(&["compound_statement"])?,
    ));
    Some(CatchStmt::new(decl, body))
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

        let expected: Stmt = ReturnStmt::new(Some(
            UnaryExpr::new(
                Box::new(
                    ParenExpr::new(Box::new(
                        BinaryExpr::new(
                            Box::new(
                                UnaryExpr::new(Box::new(Expr::Literal("this".into())), Op::Star)
                                    .into(),
                            ),
                            Op::EqualEqual,
                            Box::new(Ident::new("rhs".into()).into()),
                        )
                        .into(),
                    ))
                    .into(),
                ),
                Op::ExclamationPoint,
            )
            .into(),
        ))
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
                Ident::new("xfer".into()),
            )],
            vec![Literal::new("0".into()).into()],
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
            vec![Ident::new("_i284".into()).into()],
            vec![Expr::Literal("0".into())],
        )
        .into();
        let init: Stmt = init.into();

        let expected: Stmt = ForStmt::new(
            Some(Box::new(init)),
            Some(
                BinaryExpr::new(
                    Box::new(Ident::new("_i284".into()).into()),
                    Op::LessThan,
                    Box::new(Ident::new("_size280".into()).into()),
                )
                .into(),
            ),
            Some(IncDecExpr::new(true, true, Box::new(Ident::new("_i284".into()).into())).into()),
            Block::new(vec![]).into(),
        )
        .into();
        let expected: Node = expected.into();

        let actual = body_node(&ast).unwrap();

        assert_eq!(expected, actual);
    }
}
