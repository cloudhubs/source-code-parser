use crate::parse::AST;
use crate::prophet::*;
use crate::go::util::vartype::find_type;
use crate::go::util::identifier::parse_identifier;
use crate::msd::NodeType::MethodParam;
use crate::go::function_body::parse_block;

pub(crate) fn parse_function(
    ast: &AST,
    module_name: &str,
    path: &str
) -> Option<MethodComponent> {
    //find the function name
    let fn_identifier = parse_identifier(ast);
    //get return type
    let return_type = find_type(ast);

    let component = ComponentInfo {
        path: path.to_string(),
        package_name: module_name.to_string(),
        instance_name: fn_identifier.clone(),
        instance_type: InstanceType::MethodComponent,
    };

    let mut body = None;
    let span = ast.span.expect("No span for a method! AST malformed!");
    let line_begin = span.0 as i32;
    let line_end = span.2 as i32;

    //parse parameter list
    let mut parameters = vec![];
    let param_list = match ast.find_child_by_type(&["parameter_list"]) {
        Some(list) => {
            //iterate through the list, pushing each parameter to parameters
            for node in list.children.iter() {
                match &*node.r#type {
                    "parameter_declaration" => parameters.push(parse_parameter(node, &component)),
                    _ => {},
                }
            }
        },
        None => {}
    };

    for member in ast.children.iter() {
        match &*member.r#type {
            "block" => {
                body = Some(parse_block(member, &component));
            }
            _ => {}
        }
    }

    let func = MethodComponent {
        component: ComponentInfo {
            path: path.into(),
            package_name: module_name.to_string(),
            instance_name: fn_identifier.clone(),
            instance_type: InstanceType::MethodComponent
        },
        accessor: AccessorType::Public,
        method_name: fn_identifier,
        return_type: return_type,
        parameters: parameters,
        is_static: false,
        is_abstract: false,
        is_final: false,
        sub_methods: vec![],
        annotations: vec![],
        line_count: line_end - line_begin + 1,
        line_begin,
        line_end,
        body
    };
    Some(func)
}

pub(crate) fn parse_method(ast: &AST, module_name: &str, path: &str)
    -> (String, MethodComponent) {
    let method_identifier = parse_identifier(ast);
    let return_type = find_type(ast);

    let component = ComponentInfo {
        path: path.to_string(),
        package_name: module_name.to_string(),
        instance_name: method_identifier.clone(),
        instance_type: InstanceType::MethodComponent,
    };

    //Define fields
    let mut body = None;
    let span = ast.span.expect("No span for a method! AST malformed!");
    let line_begin = span.0 as i32;
    let line_end = span.2 as i32;

    //parse first parameter list, which ideally is where the actual parent struct is
    let mut parent_struct_type_name = String::new();
    match ast.find_child_by_type(&["parameter_list"]) {
        Some(parameter_list) => {
            match parameter_list.find_child_by_type(&["parameter_declaration"]) {
                Some(parameter_node) => {
                    parent_struct_type_name = parse_parameter(parameter_node, &component).r#type.clone()
                },
                None => {},
            }
        },
        None => {},
    };

    let mut i = 0;
    let mut parameters = vec![];
    for node in ast.find_all_children_by_type(&["parameter_list"]).get_or_insert(vec![]).iter() {
        if i == 0 {
            i = 1;
        }
        else {
            for param_node in node.children.iter() {
                match &*param_node.r#type {
                    "parameter_declaration" => parameters.push(parse_parameter(param_node, &component)),
                    _ => {},
                }
            }
        }
    };

   for member in ast.children.iter() {
       match &*member.r#type {
           "block" => {
               body = Some(parse_block(member, &component));
           }
           _ => {}
       }
   }

    let func = MethodComponent {
        component: ComponentInfo {
            path: path.into(),
            package_name: module_name.to_string(),
            instance_name: method_identifier.clone(),
            instance_type: InstanceType::MethodComponent
        },
        accessor: AccessorType::Public,
        method_name: method_identifier,
        return_type: return_type,
        parameters: parameters,
        is_static: false,
        is_abstract: false,
        is_final: false,
        sub_methods: vec![],
        annotations: vec![],
        line_count: line_end - line_begin + 1,
        line_begin,
        line_end,
        body
    };
    (parent_struct_type_name, func)
}





fn parse_parameter(ast: &AST, component: &ComponentInfo) -> MethodParamComponent {
    let mut name = parse_identifier(ast);
    //let mut modifier = Modifier::new();
    let param_type = find_type(ast);

    MethodParamComponent {
        component: ComponentInfo {
            path: component.path.clone(),
            package_name: component.package_name.clone(),
            instance_name: component.instance_name.clone(),
            instance_type: InstanceType::MethodParamComponent,
        },
        annotation: None,
        r#type: param_type.into(),
        parameter_name: name.into(),
    }

}