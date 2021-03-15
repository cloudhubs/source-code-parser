use super::*;
use crate::ast::*;
use crate::parse::AST;

/// Converts a given expression node into an Expr
pub fn expression(node: &AST) -> Option<Expr> {
    match &*node.r#type {
        "pointer_declarator" | "reference_declarator" | "parameter_declaration" => declarator(node),
        "pointer_expression" | "reference_expression" => pointer_expression(node),
        "identifier" | "field_identifier" | "type_identifier" => Some(identifier(node).into()),
        "assignment_expression" | "binary_expression" => binary_expression(node),
        "call_expression" => call_expression(node),
        "field_expression" => Some(dot_expression(node)?.into()),
        "unary_expression" => Some(unary_expression(node)?.into()),
        "parenthesized_expression" => Some(paren_expression(node)?.into()),
        "true" | "false" | "number_literal" | "this" | "auto" | "string_literal" => {
            Some(Literal::new(node.value.clone()).into())
        }
        "condition_clause" => {
            let cond = node.children.iter().nth(1)?;
            expression(cond)
        }
        // Handle scoped identifiers
        "scoped_identifier" | "template_function" => {
            let s = type_ident(node);
            Some(Ident::new(s).into())
        }
        "update_expression" => update_expression(node),
        "array_declarator" | "subscript_expression" | "new_declarator" => {
            Some(index_expression(node)?.into())
        }
        "new_expression" => Some(new_expression(node)?.into()),
        "delete_expression" => Some(delete_expression(node)?.into()),
        "initializer_list" => Some(init_list_expression(node).into()),
        "sizeof_expression" => Some(sizeof_expression(node)?.into()),
        "lambda_expression" => Some(lambda_expression(node)?.into()),
        "type_descriptor" => Some(type_descriptor(node)?.into()),
        _ => None,
    }
}

fn declarator(node: &AST) -> Option<Expr> {
    let mut ptr_symbol = String::new();
    let name = variable_ident(node, &mut ptr_symbol)?;
    let mut ident: Expr = match &*name {
        "this" => Literal::new(name).into(),
        _ => Ident::new(name).into(),
    };
    ptr_symbol
        .chars()
        .map(|star| Op::from(&*star.to_string()))
        .for_each(|star| ident = UnaryExpr::new(Box::new(ident.clone()), star).into());
    Some(ident)
}

fn pointer_expression(ptr_expr: &AST) -> Option<Expr> {
    let mut ptr_symbol = String::new();
    let name = variable_ident_inner(ptr_expr, &mut ptr_symbol)?;
    let mut ident: Expr = match &*name {
        "this" => Literal::new(name).into(),
        _ => Ident::new(name).into(),
    };
    ptr_symbol
        .chars()
        .map(|star| Op::from(&*star.to_string()))
        .for_each(|star| ident = UnaryExpr::new(Box::new(ident.clone()), star).into());
    Some(ident)
}

fn identifier(ident: &AST) -> Ident {
    let name = ident.value.clone();
    Ident::new(name)
}

fn dot_expression(field_expr: &AST) -> Option<DotExpr> {
    let mut nodes = field_expr.children.iter();
    let lhs = expression(nodes.next()?)?;
    let rhs = expression(nodes.last()?)?;
    Some(DotExpr::new(Box::new(lhs), Box::new(rhs)))
}

fn unary_expression(unary_expr: &AST) -> Option<UnaryExpr> {
    let op = Op::from(&*unary_expr.children.iter().next()?.value);
    let expr = expression(unary_expr.children.iter().last()?)?;
    Some(UnaryExpr::new(Box::new(expr), op))
}

fn paren_expression(paren_expr: &AST) -> Option<ParenExpr> {
    let expr = paren_expr.children.iter().nth(1)?;
    Some(ParenExpr::new(Box::new(expression(expr)?)))
}

fn update_expression(update_expr: &AST) -> Option<Expr> {
    // As far as I can tell, update_expressions are always increment/decrement operations.
    // If there's another type of node where update_expression would show up, the call to
    // expression() from the for_statement() function should be able to handle the case.
    let mut it = update_expr.children.iter();
    let first = it.next()?;
    let second = it.next()?;
    let update_expr_info = |node: &AST| match &*node.r#type {
        "++" => (Some(true), Some(true)),
        "--" => (Some(true), Some(false)),
        _ => (None, None),
    };
    let (is_pre, is_inc) = update_expr_info(first);
    match is_pre {
        Some(_) => {
            let expr = expression(second)?;
            Some(IncDecExpr::new(is_pre?, is_inc?, Box::new(expr)).into())
        }
        None => {
            let expr = expression(first)?;
            let (is_pre, is_inc) = update_expr_info(second);
            Some(IncDecExpr::new(is_pre?, is_inc?, Box::new(expr)).into())
        }
    }
}

/// Converts an "array_declarator", "subscript_expression", or "new_declarator" into an IndexExpr
fn index_expression(node: &AST) -> Option<IndexExpr> {
    // Identifier won't be found when the type is new_declarator.
    let ident = expression(node.children.iter().nth(0)?)
        .unwrap_or(Literal::new("Missing Ident".into()).into());
    let ndx_expr = expression(node.children.iter().nth(2)?)?;
    let ndx = IndexExpr::new(Box::new(ident), Box::new(ndx_expr));
    Some(ndx)
}

fn new_expression(new_expr: &AST) -> Option<CallExpr> {
    let r#type = expression(new_expr.children.iter().nth(1)?)?;
    let expr = expression(new_expr.children.iter().last()?)?;
    match expr {
        Expr::IndexExpr(mut ndx) => {
            ndx.expr = Box::new(r#type);
            let new: Literal = "new".into();
            let init = CallExpr::new(Box::new(new.into()), vec![ndx.into()]);
            Some(init)
        }
        _ => None,
    }
}

fn delete_expression(delete_expr: &AST) -> Option<CallExpr> {
    let expr = delete_expr
        .children
        .iter()
        .find(|node| match &*node.r#type {
            "delete" | "[" | "]" => false,
            _ => true,
        })?;
    let expr = expression(expr)?;
    let delete: Literal = "delete".to_string().into();
    let del = CallExpr::new(Box::new(delete.into()), vec![expr]);
    Some(del)
}

fn init_list_expression(initializer_list: &AST) -> InitListExpr {
    let exprs = initializer_list
        .children
        .iter()
        .filter(|node| match &*node.r#type {
            "{" | "}" | "," => false,
            _ => true,
        })
        .map(|node| expression(node))
        .flatten()
        .collect();
    InitListExpr::new(exprs)
}

fn sizeof_expression(sizeof_expr: &AST) -> Option<CallExpr> {
    let expr = expression(
        sizeof_expr
            .children
            .iter()
            .filter(|node| match &*node.r#type {
                "(" | ")" => false,
                _ => true,
            })
            .last()?,
    )?;
    let sizeof: Literal = "sizeof".to_string().into();
    let sizeof_call = CallExpr::new(Box::new(sizeof.into()), vec![expr]);
    Some(sizeof_call)
}

fn binary_expression(node: &AST) -> Option<Expr> {
    let mut nodes = node.children.iter();
    let lhs = expression(nodes.next()?)?;
    let op = Op::from(&*nodes.next()?.value);
    let rhs = expression(nodes.next()?)?;

    // Check if the binary expression is a logging statement
    // taking the form of (LOG(level) || cout) << ... << ...;
    let is_log = match (&op, &lhs) {
        // Identify the log call to know to create a LogExpr
        (Op::BitShiftLeft, Expr::CallExpr(call)) => match &*call.name {
            Expr::Ident(ident) => match &*ident.name.to_lowercase() {
                "log" => true,
                _ => false,
            },
            _ => false,
        },
        // LogExpr already created on left hand side, so append this expr
        // to its arguments
        (Op::BitShiftLeft, Expr::LogExpr(_)) => true,
        // Printing to screen counts as a log
        (Op::BitShiftLeft, Expr::Ident(ident)) => match &*ident.name {
            "std::cout" | "cout" => true,
            _ => false,
        },
        _ => false,
    };

    let expr = match lhs {
        Expr::LogExpr(mut log) => {
            log.args.push(rhs);
            return Some(log.into());
        }
        _ => BinaryExpr::new(Box::new(lhs), op, Box::new(rhs)).into(),
    };

    if is_log {
        let log = convert_binary_expr_to_log(expr)?;
        Some(log.into())
    } else {
        Some(expr.into())
    }
}

fn convert_binary_expr_to_log(cpp_log: BinaryExpr) -> Option<LogExpr> {
    // Get the flattened children of the binary expression, with the left
    // hand most child being the log call or cout identifier.
    let mut children = flatten_binary_expr_children(cpp_log);
    children.reverse();

    // Get the log call expression
    let log_call = match children.pop()? {
        // LOG call identified
        Expr::CallExpr(call) => call,
        // Printed to screen, convert Ident to CallExpr
        Expr::Ident(ident) => match &*ident.name {
            "std::cout" | "cout" => CallExpr::new(
                Box::new(Literal::new("".into()).into()),
                vec![Ident::new("console".into()).into()],
            ),
            _ => return None,
        },
        _ => return None,
    };

    children.reverse();

    // Get the log level from the first argument.
    let log_level = match &log_call.args.first()? {
        Expr::Ident(ident) => LogLevel::from(&*ident.name),
        _ => return None,
    };

    Some(LogExpr::new(log_level, children))
}

/// Convert all of the children of a binary expression into their left and right hand
/// expressions, in the order they appear in the code.
fn flatten_binary_expr_children(expr: BinaryExpr) -> Vec<Expr> {
    match (*expr.lhs, *expr.rhs) {
        (Expr::BinaryExpr(lhs), Expr::BinaryExpr(rhs)) => flatten_binary_expr_children(lhs)
            .into_iter()
            .chain(flatten_binary_expr_children(rhs))
            .collect(),
        (expr, Expr::BinaryExpr(rhs)) => vec![expr]
            .into_iter()
            .chain(flatten_binary_expr_children(rhs))
            .collect(),
        (Expr::BinaryExpr(lhs), expr) => flatten_binary_expr_children(lhs)
            .into_iter()
            .chain(vec![expr])
            .collect(),
        (lhs, rhs) => vec![lhs, rhs],
    }
}

// Takes AST node type "call_expression"
fn call_expression(node: &AST) -> Option<Expr> {
    let mut nodes = node.children.iter();
    // field_expression, identifier
    let function_name = expression(nodes.next()?)?;
    let argument_list = nodes.next_back()?;
    let args: Vec<Expr> = argument_list
        .children
        .iter()
        .map(|arg| expression(arg))
        .flat_map(|arg| arg)
        .collect();
    Some(CallExpr::new(Box::new(function_name), args).into())
}

fn lambda_expression(lambda_expr: &AST) -> Option<LambdaExpr> {
    let parameter_list = lambda_expr
        .find_child_by_type(&["abstract_function_declarator"])?
        .find_child_by_type(&["parameter_list"])?
        .children
        .iter()
        .filter(|child| child.r#type == "parameter_declaration")
        .map(|param_decl| variable_declaration(param_decl))
        .collect();

    let body = func_body(lambda_expr.find_child_by_type(&["compound_statement"])?);

    Some(LambdaExpr::new(parameter_list, body))
}

fn type_descriptor(type_desc: &AST) -> Option<Literal> {
    let specifier = type_desc.children.iter().next()?;
    match &*specifier.r#type {
        "struct_specifier" => Some(Literal::new(format!(
            "struct {}",
            type_ident(specifier.children.iter().last()?)
        ))),
        _ => None,
    }
}
