use runestick::Any;
use std::collections::HashMap;

pub type ContextData = HashMap<String, HashMap<String, Option<String>>>;

/// Prefix for an entry in ParserContext.variables indicating it's a tag, not an object
const TAG_PREFIX: &'static str = "?";

/// Special attribute that's value is the name that a tag should resolve to
const RESOLVES_TO: &'static str = "resolves_to";

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
            .filter(|(key, _)| !key.starts_with("?"))
            .collect()
    }
}

/// Interface of the Context, offering ability to create, read, and update objects/tags
pub trait ContextObjectActions {
    fn make_object(&mut self, name: &str);
    fn make_attribute(&mut self, name: &str, attr_name: &str, attr_type: Option<String>);
    fn make_tag(&mut self, name: &str, resolves_to: &str);
    /// Gets an object by name. The object returned shouldn't be modified from this return value. Use ContextObjectActions::make_attribute
    fn get_object(&self, name: &str) -> Option<HashMap<String, Option<String>>>;
}

/// Interface of the Context, offering ability to create, read, and update objects/tags
pub trait ContextLocalVariableActions {
    fn make_variable(&mut self, name: &str, val: &str);
    fn get_variable(&self, name: &str) -> Option<String>;
    fn clear_variables(&mut self);
}

impl ContextObjectActions for ParserContext {
    fn make_object(&mut self, name: &str) {
        let obj_name: String = name.into();
        if !self.variables.contains_key(&obj_name) {
            (&mut self.variables).insert(obj_name, HashMap::new());
        }
    }

    fn make_attribute(&mut self, name: &str, attr_name: &str, attr_type: Option<String>) {
        let obj_name: String = name.into();
        if !self.variables.contains_key(&obj_name) {
            eprintln!("Defining attribute on a non-existant object. Defining...");
            self.make_object(name);
        }

        // Insert
        let vars = self.variables.get_mut(&obj_name).unwrap();
        match vars.insert(attr_name.into(), attr_type) {
            Some(Some(overwritten)) => eprintln!(
                "Warning: overwrote {} on {}.{}!",
                overwritten, name, attr_name
            ),
            _ => {}
        }
    }

    fn make_tag(&mut self, name: &str, resolves_to: &str) {
        self.make_attribute(
            format!("{}{}", TAG_PREFIX, name).as_str(),
            RESOLVES_TO,
            Some(resolves_to.into()),
        );
    }

    fn get_object(&self, name: &str) -> Option<HashMap<String, Option<String>>> {
        if let Some(obj) = self.variables.get(name.into()) {
            if name.starts_with(TAG_PREFIX) {
                // Get the object. Extensive `expect`s because, if we don't have a RESOLVES_TO
                // attribute with a name the tag aliases on it, then we have serious data corruption
                // we shouldn't just ignore.
                self.get_object(
                    obj.get(RESOLVES_TO)
                        .expect(format!("Invalid tag! no {} value!", RESOLVES_TO).as_str())
                        .as_ref()
                        .expect(format!("Invalid tag! no {} value!", RESOLVES_TO).as_str())
                        .as_str(),
                )
            } else {
                Some(obj.clone())
            }
        } else {
            None
        }
    }
}

impl ContextLocalVariableActions for ParserContext {
    fn make_variable(&mut self, name: &str, val: &str) {
        if let Some(overwritten) = self.local_variables.insert(name.into(), val.into()) {
            eprintln!(
                "Warning: overwrote {} with {} for name {}",
                overwritten, val, name
            );
        }
    }

    fn get_variable(&self, name: &str) -> Option<String> {
        match self.local_variables.get(name.into()) {
            Some(value) => Some(value.clone()),
            None => None,
        }
    }

    fn clear_variables(&mut self) {
        self.local_variables.clear();
    }
}
