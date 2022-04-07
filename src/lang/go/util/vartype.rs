use crate::AST;

/// Indicates when no type is found
pub const NO_TYPE: &str = "N/A";


pub(crate) fn find_type(ast: &AST) -> String {
    let my_node = unwrap_pointer_type(ast);
    let mut is_ptr = false;

    match my_node.find_child_by_type(&["*"]) {
        Some(_node) => {is_ptr = true;},
        None => {}
    }

    let mut r#type = match my_node.find_child_by_type(&["qualified_type"]) {
        Some(node) => {
            parse_qualified_type(node)
        },
        None => {
            let id_node = match my_node.find_child_by_type(&["type_identifier"]) {
                Some(node) => node,
                None => ast,
            };
            let str = parse_type(id_node);
            str
        }
    };
    
    if is_ptr == true {
        r#type = format!("*{}", r#type);
    }
    r#type
}

pub(crate) fn find_return(ast: &AST) -> String {
    let mut i = 0;
    let mut ret = "".to_string();
    
    for node in ast.children.iter() {
        match &*node.r#type {
            "parameter_list" => {
                //if it has multiple return values
                if i == 4 {
                    for sub_node in node.children.iter() {
                        match &*sub_node.r#type {
                            "parameter_declaration" => {
                                ret += &find_type(unwrap_pointer_type(sub_node));
                                ret += ", "
                            },
                            _ => {}
                        }
                        
                    }

                    ret.pop();
                    ret.pop();
                }
            },
            "type_identifier" => {
                //if it ends with a single return
                if i == 4 {
                    ret = node.value.clone();
                }
            },
            _ => {}
        }
        i += 1;
    }
    

    ret
}

pub(crate) fn unwrap_pointer_type(ast: &AST) -> &AST {
    let mut my_node = ast;

    for node in ast.children.iter() {
        match &*node.r#type {
            "pointer_type" => {
                my_node = node;
            },
            _ => {}
        }
    }

    my_node
}

fn parse_type(ast: &AST) -> String {
    match &*ast.r#type {
        "type_identifier" => ast.value.clone(),
        _ => NO_TYPE.into()
    }
}

fn parse_qualified_type(ast: &AST) -> String {
    let mut type_str = "".to_string();

    for node in ast.children.iter() {
        match &*node.r#type {
            "package_identifier" | "." | "type_identifier" => {
                type_str += &node.value;
            },
            _ => {}
        }
    }

    type_str
}