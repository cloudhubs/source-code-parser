use runestick::Any;
use std::collections::HashMap;

pub type ContextData = HashMap<String, HashMap<String, Option<String>>>;

/// Special attribute that's value is the name that a tag should resolve to
const RESOLVES_TO: &'static str = "???";

/// Special attribute indicating an object is transient
const TRANSIENT: &'static str = "";

/// Context used by the Parser, storing local variables (#{varname}) and objects/tags
#[derive(Default, Debug, Clone, Any, PartialEq)]
pub struct ParserContext {
    variables: ContextData,
    local_variables: HashMap<String, String>,
    pub frame_number: i32,
}

impl Into<ContextData> for ParserContext {
    fn into(self) -> ContextData {
        self.variables
            .into_iter()
            .filter(|(_, val)| !val.contains_key(RESOLVES_TO) && !val.contains_key(TRANSIENT))
            .collect()
    }
}

/// Interface of the Context, offering ability to create, read, and update objects/tags
pub trait ContextObjectActions {
    fn make_object(&mut self, name: &str);
    fn make_attribute(&mut self, name: &str, attr_name: &str, attr_value: Option<String>);
    fn make_tag(&mut self, name: &str, resolves_to: &str);
    /// Makes an existing object transient, or initializes a new object as transient
    fn make_transient(&mut self, name: &str);
    /// Gets an object by name. The object returned shouldn't be modified from this return value. Use ContextObjectActions::make_attribute
    fn get_object(&self, name: &str) -> Option<HashMap<String, Option<String>>>;
    fn resolve_tag(&self, name: &str) -> String;
}

/// Interface of the Context, offering ability to create, read, and update objects/tags
pub trait ContextLocalVariableActions {
    fn make_variable(&mut self, name: &str, val: &str);
    fn get_variable(&self, name: &str) -> Option<String>;
    fn clear_variables(&mut self);
}

impl ParserContext {
    fn do_make_attribute(&mut self, obj_name: &str, attr_name: &str, attr_type: Option<String>) {
        // If a reference to a non-existant object, create it
        if !self.variables.contains_key(obj_name) {
            eprintln!("Defining attribute on a non-existant object. Defining...");
            self.make_object(obj_name);
        }

        // Insert
        let vars = self.variables.get_mut(obj_name).unwrap();
        match vars.insert(attr_name.into(), attr_type.clone()) {
            Some(Some(overwritten)) => eprintln!(
                "Warning: overwrote {} on {}.{}!",
                overwritten, obj_name, attr_name
            ),
            _ => {
                // println!("Made: {}.{}={:?}", obj_name, attr_name, attr_type);
            }
        }
    }
}

impl ContextObjectActions for ParserContext {
    fn make_object(&mut self, name: &str) {
        let obj_name: String = name.into();
        if !self.variables.contains_key(&obj_name) {
            // println!("Making: {}", obj_name);
            (&mut self.variables).insert(obj_name, HashMap::new());
        }
    }

    fn make_attribute(&mut self, name: &str, attr_name: &str, attr_type: Option<String>) {
        self.do_make_attribute(&*self.resolve_tag(name), attr_name, attr_type);
    }

    fn make_tag(&mut self, name: &str, resolves_to: &str) {
        // println!("Made: ?{} => {}", name, resolves_to);
        self.do_make_attribute(name, RESOLVES_TO, Some(resolves_to.into()));
    }

    fn make_transient(&mut self, name: &str) {
        self.make_object(name);
        self.make_attribute(name, TRANSIENT, None);
    }

    fn get_object(&self, name: &str) -> Option<HashMap<String, Option<String>>> {
        // println!("Looking for {}...", name);
        let name = self.resolve_tag(name);

        if let Some(obj) = self.variables.get(&name) {
            // println!("Retrieved {} Found {:?}", name, obj);
            Some(obj.clone())
        } else {
            None
        }
    }

    fn resolve_tag(&self, name: &str) -> String {
        if let Some(map) = &self.variables.get(name).as_ref() {
            if map.contains_key(RESOLVES_TO) {
                if &*map
                    .get(RESOLVES_TO)
                    .unwrap()
                    .as_ref()
                    .expect("RESOLVES_TO malformed, does not have a value")
                    == name
                {
                    panic!("{}", name);
                }
                return self.resolve_tag(
                    &*map
                        .get(RESOLVES_TO)
                        .unwrap()
                        .as_ref()
                        .expect("RESOLVES_TO malformed, does not have a value"),
                );
            }
        }
        name.into()
    }
}

impl ContextLocalVariableActions for ParserContext {
    fn make_variable(&mut self, name: &str, val: &str) {
        // println!("Made: ({:?}, {:?})", name, val);
        if let Some(overwritten) = self.local_variables.insert(name.into(), val.into()) {
            eprintln!(
                "Warning: overwrote {} with {} for name {}",
                overwritten, val, name
            );
        }
    }

    fn get_variable(&self, name: &str) -> Option<String> {
        let var = match self.local_variables.get(name.into()) {
            Some(value) => Some(value.clone()),
            None => None,
        };
        // println!("Found: {:?}", var);
        var
    }

    fn clear_variables(&mut self) {
        self.local_variables.clear();
    }
}
