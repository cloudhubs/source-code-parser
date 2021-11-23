use crate::Language;
use crate::go::util::identifier::parse_identifier;

use crate::ast::*;
use crate::ComponentInfo;
use crate::AST;

use super::is_common_junk_tag;

pub(crate) fn parse_expr(ast: &AST, component: &ComponentInfo) -> Option<Expr> {
    match &*ast.r#type {
        // Variables and initialization
        "identifier" | "field_identifier" => parse_ident(ast, component),
        "int_literal" | 
        "interpreted_string_literal" | "nil" | "true" | "false" => Some(Expr::Literal(Literal::new(ast.value.clone(), Language::Go))),
        "assignment_statement" => parse_assignment(ast, component),

        //language specific
        "binary_expression" => parse_binary(ast, component),
        "expression_list" => parse_expr_stmt(ast, component),
        "inc_statement" | "dec_statement" => parse_inc_dec(ast, component),

        //function and method calls
        "call_expression" => parse_function(ast, component),
        "selector_expression" => Some(parse_dot_expr(ast, component)?.into()),

        unknown => None,
    }
}

pub(crate) fn parse_expr_stmt(ast: &AST, component: &ComponentInfo) -> Option<Expr> {
    let mut expr = None;
    for comp in ast.children.iter() {
        expr = parse_expr(comp, component);
        if expr.is_some() {
            break;
        }
    }
    expr
}

fn parse_ident(ast: &AST, component: &ComponentInfo) -> Option<Expr> {
    let ident: Expr = Ident::new(ast.value.clone(), Language::Go).into();
    Some(ident.into())
}

/// Parse an assignment expression. May contain a variable declaration
pub(crate) fn parse_assignment(ast: &AST, component: &ComponentInfo) -> Option<Expr> {
    // Define attributes
    let mut lhs = None;
    let mut rhs = None;

    // Find values
    for node in ast.children.iter() {

        let unknown = &*node.r#type;
        if unknown == "=" {
            continue;
        }

        let result = parse_expr(node, component);

        if result.is_some() {
            if lhs.is_none() {
                lhs = result;
            } else if rhs.is_none() {
                rhs = result;
            } else {
                eprintln!(
                    "Extra parsable tag {} encountered while parsing assignment",
                    unknown
                );
            }
        } else {
            //log_unknown_tag(unknown, "parse_assignment");
        }
    }

    // Assemble
    if let Some(lhs) = lhs {
        if let Some(rhs) = rhs {
            let bin: Expr = BinaryExpr::new(Box::new(lhs.into()), "=".into(), Box::new(rhs), Language::Go).into();
            Some(bin.into())
        } else {
            Some(lhs.into())
        }
    } else {
        eprintln!("Assignment with no lefthand side!");
        None
    }
}

//parse increment or decrement statments
fn parse_inc_dec(ast: &AST, component: &ComponentInfo) -> Option<Expr> {
    let name = if ast.children[0].r#type == "identifier" {
        0
    } else {
        1
    };
    let op = (name + 1) % 2;

    Some(
        IncDecExpr::new(
            op < name,
            ast.children[op].r#type == "++",
            Box::new(parse_expr(&ast.children[name], component)?),
            Language::Go,
        )
        .into(),
    )
}

fn parse_binary(ast: &AST, component: &ComponentInfo) -> Option<Expr> {
    let mut lhs = None;
    let mut op = None;
    let mut rhs = None;
    for child in ast.children.iter() {
        if !is_common_junk_tag(&child.r#type) {
            let res = Some(child);
            if lhs.is_none() {
                lhs = res;
            } else if op.is_none() {
                op = res;
            } else if rhs.is_none() {
                rhs = res;
                break;
            }
        }
    }

    if let Some(lhs) = lhs {
        if let Some(op) = op {
            if let Some(rhs) = rhs {
                return Some(
                    BinaryExpr::new(
                        Box::new(parse_expr(lhs, component)?),
                        op.value.as_str().into(),
                        Box::new(parse_expr(rhs, component)?),
                        Language::Go
                    )
                    .into(),
                );
            }
        }
    }
    eprintln!("Malformed binary expression detected!");
    None
}

fn parse_function(ast: &AST, component: &ComponentInfo) -> Option<Expr> {
    let selector = match ast.find_child_by_type(&["selector_expression"]) {
        Some(node) => node,
        None => ast
    };

    let argument_list = match ast.find_child_by_type(&["argument_list"]) {
        Some(node) => node,
        None => ast
    };

    let args: Vec<Expr> = argument_list
        .children
        .iter()
        .map(|arg| parse_expr(arg, component))
        .flat_map(|arg| arg)
        .collect();
    

    //determine the type of function call
    if selector.find_child_by_type(&["."]).is_some() {
        //member functions
        let function_name = parse_dot_expr(selector, component)?;

        Some(CallExpr::new(Box::new(function_name.into()), args, Language::Go).into())
    } else {
        //regular functions
        let name = Ident::new(parse_identifier(&selector.children[0]), Language::Go);
        Some(CallExpr::new(Box::new(name.into()), args, Language::Go).into())
    }
}

fn parse_dot_expr(node: &AST, component: &ComponentInfo) -> Option<DotExpr>{
    //get the name of what called the function
    //let lhs = Ident::new(parse_identifier(&node.children[0]), Language::Go);
    let lhs = parse_expr(&node.children[0], component)?;
    //let rhs = Ident::new(parse_identifier(&node.children[2]), Language::Go);
    let rhs = parse_expr(&node.children[2], component)?;
    
    Some(DotExpr::new(Box::new(lhs), Box::new(rhs), Language::Go))
}



