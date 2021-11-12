use super::*;
use crate::ast::*;
use crate::parse::AST;
use Language::Cpp;

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
            Some(Literal::new(node.value.clone(), Cpp).into())
        }
        "condition_clause" => {
            let cond = node.children.get(1)?;
            expression(cond)
        }
        // Handle scoped identifiers
        "scoped_identifier" | "template_function" => {
            let s = type_ident(node);
            Some(Ident::new(s, Cpp).into())
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
        "this" => Literal::new(name, Cpp).into(),
        _ => Ident::new(name, Cpp).into(),
    };
    ptr_symbol
        .chars()
        .map(|star| Op::from(&*star.to_string()))
        .for_each(|star| ident = UnaryExpr::new(Box::new(ident.clone()), star, Cpp).into());
    Some(ident)
}

fn pointer_expression(ptr_expr: &AST) -> Option<Expr> {
    let mut ptr_symbol = String::new();
    let name = variable_ident_inner(ptr_expr, &mut ptr_symbol)?;
    let mut ident: Expr = match &*name {
        "this" => Literal::new(name, Cpp).into(),
        _ => Ident::new(name, Cpp).into(),
    };
    ptr_symbol
        .chars()
        .map(|star| Op::from(&*star.to_string()))
        .for_each(|star| ident = UnaryExpr::new(Box::new(ident.clone()), star, Cpp).into());
    Some(ident)
}

fn identifier(ident: &AST) -> Ident {
    let name = ident.value.clone();
    Ident::new(name, Cpp)
}

fn dot_expression(field_expr: &AST) -> Option<DotExpr> {
    let mut nodes = field_expr.children.iter();
    let lhs = expression(nodes.next()?)?;
    let rhs = expression(nodes.last()?)?;
    Some(DotExpr::new(Box::new(lhs), Box::new(rhs), Cpp))
}

fn unary_expression(unary_expr: &AST) -> Option<UnaryExpr> {
    let op = Op::from(&*unary_expr.children.first()?.value);
    let expr = expression(unary_expr.children.iter().last()?)?;
    Some(UnaryExpr::new(Box::new(expr), op, Cpp))
}

fn paren_expression(paren_expr: &AST) -> Option<ParenExpr> {
    let expr = paren_expr.children.get(1)?;
    Some(ParenExpr::new(Box::new(expression(expr)?), Cpp))
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
            Some(IncDecExpr::new(is_pre?, is_inc?, Box::new(expr), Cpp).into())
        }
        None => {
            let expr = expression(first)?;
            let (is_pre, is_inc) = update_expr_info(second);
            Some(IncDecExpr::new(is_pre?, is_inc?, Box::new(expr), Cpp).into())
        }
    }
}

/// Converts an "array_declarator", "subscript_expression", or "new_declarator" into an IndexExpr
fn index_expression(node: &AST) -> Option<IndexExpr> {
    // Identifier won't be found when the type is new_declarator.
    let ident = expression(node.children.first()?)
        .unwrap_or_else(|| Literal::new("Missing Ident".into(), Cpp).into());
    let ndx_expr = expression(node.children.get(2)?)?;
    let ndx = IndexExpr::new(Box::new(ident), Box::new(ndx_expr), Cpp);
    Some(ndx)
}

fn new_expression(new_expr: &AST) -> Option<CallExpr> {
    let r#type = expression(new_expr.children.get(1)?)?;
    let expr = expression(new_expr.children.last()?)?;
    match expr {
        Expr::IndexExpr(mut ndx) => {
            ndx.expr = Box::new(r#type);
            let new = Literal::new("new".into(), Cpp);
            let init = CallExpr::new(Box::new(new.into()), vec![ndx.into()], Cpp);
            Some(init)
        }
        _ => None,
    }
}

fn delete_expression(delete_expr: &AST) -> Option<CallExpr> {
    let expr = delete_expr
        .children
        .iter()
        .find(|node| !matches!(&*node.r#type, "delete" | "[" | "]"))?;
    let expr = expression(expr)?;
    let delete = Literal::new("delete".to_string(), Cpp);
    let del = CallExpr::new(Box::new(delete.into()), vec![expr], Cpp);
    Some(del)
}

fn init_list_expression(initializer_list: &AST) -> InitListExpr {
    let exprs = initializer_list
        .children
        .iter()
        .filter(|node| !matches!(&*node.r#type, "{" | "}" | ","))
        .map(expression)
        .flatten()
        .collect();
    InitListExpr::new(exprs, Cpp)
}

fn sizeof_expression(sizeof_expr: &AST) -> Option<CallExpr> {
    let expr = expression(
        sizeof_expr
            .children
            .iter()
            .filter(|node| !matches!(&*node.r#type, "(" | ")"))
            .last()?,
    )?;
    let sizeof = Literal::new("sizeof".to_string(), Cpp);
    let sizeof_call = CallExpr::new(Box::new(sizeof.into()), vec![expr], Cpp);
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
            Expr::Ident(ident) => matches!(&*ident.name.to_lowercase(), "log"),
            _ => false,
        },
        // LogExpr already created on left hand side, so append this expr
        // to its arguments
        (Op::BitShiftLeft, Expr::LogExpr(_)) => true,
        // Printing to screen counts as a log
        (Op::BitShiftLeft, Expr::Ident(ident)) => matches!(&*ident.name, "std::cout" | "cout"),
        _ => false,
    };

    let expr = match lhs {
        Expr::LogExpr(mut log) => {
            log.args.push(rhs);
            return Some(log.into());
        }
        _ => match op {
            Op::Equal => AssignExpr::new(vec![lhs], vec![rhs], Cpp).into(),
            _ => BinaryExpr::new(Box::new(lhs), op, Box::new(rhs), Cpp).into(),
        },
    };

    if is_log {
        // C++ logs should not be AssignExpr since they use the << operator
        let binary_expr = match expr {
            Expr::BinaryExpr(binary_expr) => binary_expr,
            _ => unreachable!(),
        };
        let log = convert_binary_expr_to_log(binary_expr)?;
        Some(log.into())
    } else {
        Some(expr)
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
                Box::new(Literal::new("".into(), Cpp).into()),
                vec![Ident::new("console".into(), Cpp).into()],
                Cpp,
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

    Some(LogExpr::new(log_level, children, Cpp))
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
        .map(expression)
        .flatten()
        .collect();
    Some(CallExpr::new(Box::new(function_name), args, Cpp).into())
}

fn lambda_expression(lambda_expr: &AST) -> Option<LambdaExpr> {
    let parameter_list = lambda_expr
        .find_child_by_type(&["abstract_function_declarator"])?
        .find_child_by_type(&["parameter_list"])?
        .children
        .iter()
        .filter(|child| child.r#type == "parameter_declaration")
        .map(variable_declaration)
        .collect();

    let body = func_body(lambda_expr.find_child_by_type(&["compound_statement"])?);

    Some(LambdaExpr::new(parameter_list, body, Cpp))
}

fn type_descriptor(type_desc: &AST) -> Option<Literal> {
    let specifier = type_desc.children.first()?;
    match &*specifier.r#type {
        "struct_specifier" => Some(Literal::new(
            format!("struct {}", type_ident(specifier.children.iter().last()?)),
            Cpp,
        )),
        _ => None,
    }
}
