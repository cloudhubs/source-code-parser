use crate::AST;

/// File holding all Java statement parsing (e.g., while/for/trycatch)

/// Parse an AST fragment with a try/catch. May be try-with-resources, or standard try/catch, with any
/// number of catch/multi-catch blocks, and/or a finally block.
pub(crate) fn try_catch(ast: &AST, component: &ComponentInfo) -> Option<Node> {
    let mut try_body = None;
    let mut catch_clauses = vec![];
    let mut finally_clause = None;
    // let mut resources = None;

    for comp in ast.children.iter() {
        match &*comp.r#type {
            "resource_specification" => {
                let rss: Vec<Expr> = comp
                    .children
                    .iter()
                    .filter(|resource| match &*resource.r#type {
                        "resource" => true,
                        _ => false,
                    })
                    .map(|resource| parse_assignment(resource, component))
                    .flat_map(|n| n)
                    .collect();
                // resources = Some(DeclStmt::new(rss.iter().map(), expressions));
            }
            "block" => try_body = Some(parse_block(comp, component)),
            "catch_clause" => {
                let catch_comp = comp
                    .find_child_by_type(&["catch_formal_parameter"])
                    .expect("No catch variables declared!");

                // Modifiers
                let modifiers =
                    find_modifier(catch_comp, &*component.path, &*component.package_name);

                // Variables
                let caught_vars = {
                    let name = &catch_comp
                        .find_child_by_type(&["identifier"])
                        .expect("No name for caught variable!")
                        .value;
                    let types: Vec<String> = catch_comp
                        .find_child_by_type(&["catch_type"])
                        .expect("No type on catch block!")
                        .find_all_children_by_type(&["type_identifier"])
                        .expect("No type on catch block!")
                        .iter()
                        .map(|child| child.value.clone())
                        .collect();

                    types
                        .into_iter()
                        .map(|t| {
                            let mut decl = VarDecl::new(Some(t), Ident::new(name.clone()));
                            decl.is_final = Some(modifiers.is_final);
                            decl.is_static = Some(modifiers.is_static);
                            decl.annotation = modifiers.annotations.clone();
                            decl
                        })
                        .collect()
                };

                // Body
                let catch_body = parse_block(
                    comp.find_child_by_type(&["block"])
                        .expect("No block for the catch body!"),
                    component,
                );

                catch_clauses.push(CatchStmt::new(
                    DeclStmt::new(caught_vars, vec![]),
                    catch_body,
                ));
            }
            "finally_clause" => finally_clause = Some(parse_block(ast, component)),
            unknown => log_unknown_tag(unknown, "try/catch"),
        }
    }

    // Generated wrappers and return
    let mut try_catch =
        TryCatchStmt::new(try_body.expect("Try/Catch with no body!"), catch_clauses);
    if finally_clause.is_some() {
        try_catch.finally_body = finally_clause;
    }
    Some(Node::Stmt(try_catch.into()))
}
