use rune::{
    runtime::{Object, Shared, Value},
    Any,
};
use std::collections::{HashMap, HashSet};

use super::RessaResult;

/// Special attribute that's value is the name that a tag should resolve to
const RESOLVES_TO: &str = "???";

/// Context used by the Parser, storing local variables (#{varname}) and objects/tags
#[derive(Default, Debug, Clone, Any)]
pub struct ParserContext {
    objectlike_data: HashMap<String, Value>,
    variables: HashMap<String, String>,
    transients: HashSet<String>,
}

/// Convert the context into the result format
impl From<ParserContext> for RessaResult {
    fn from(ctx: ParserContext) -> Self {
        let ParserContext {
            objectlike_data,
            transients,
            ..
        } = ctx;
        objectlike_data
            .into_iter()
            .filter_map(|(name, val)| {
                if transients.contains(&name) {
                    // Filter out transient objects in the result
                    None
                } else {
                    filter_map_value(val).map(|val| (name, val))
                }
            })
            .collect()
    }
}

fn filter_map_value(val: Value) -> Option<Value> {
    // This would be some much cleaner if Shared implemented Deref
    match val {
        Value::Vec(vec) => {
            let filtered = vec
                .take()
                .ok()?
                .into_iter()
                .filter_map(filter_map_value)
                .collect::<Vec<_>>();
            Some(Value::Vec(Shared::new(filtered.into())))
        }
        Value::Object(obj) => {
            let obj = obj.take().ok()?;
            if !obj.contains_key(RESOLVES_TO) {
                Some(Value::Object(Shared::new(obj)))
            } else {
                None
            }
        }
        val => Some(val),
    }
}

/// Interface of the Context, offering ability to create, read, and update objects/tags
pub trait ContextObjectActions {
    /// Save to the context under the given name; returns the input, for ease of use
    fn save(&mut self, name: &str, val: Value) -> &Value;

    /// Retrieve from the context
    fn get(&self, name: &str) -> Option<&Value>;

    /// Either retrieve the named value from the context, or save the provided value and
    /// return it. You should ensure the created object is not too expensive to make, given
    /// there's a good chance it will be discarded.
    fn get_or_save(&mut self, name: &str, val: Value) -> &Value;

    /// Create a tag for the given data in the context
    fn make_tag(&mut self, name: &str, resolves_to: &str);

    /// Denote an object as transient, or initializes a new object as transient
    fn make_transient(&mut self, name: &str);

    /// Resolve a name to the name of the real object it refers to
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
        if !self.objectlike_data.contains_key(obj_name) {
            tracing::warn!(
                "Defining attribute on a non-existant object. Defining {}...",
                obj_name
            );
            self.make_object(obj_name);
        }

        // Insert
        let vars = self.objectlike_data.get_mut(obj_name).unwrap();
        if let Value::Object(ref mut vars) = vars {
            let mut vars = vars.borrow_mut().unwrap();
            if let Some(attr_type) = attr_type {
                vars.insert(attr_name.into(), Value::from(attr_type));
            } else {
                vars.insert(attr_name.into(), Value::from(Shared::new(None)));
            }
        }
    }

    fn make_object(&mut self, name: &str) {
        let obj_name: String = name.into();
        if !self.objectlike_data.contains_key(&obj_name) {
            // tracing::info!("Making: {}", obj_name);
            (&mut self.objectlike_data).insert(obj_name, Object::new().into());
        }
    }
}

impl ContextObjectActions for ParserContext {
    fn save(&mut self, name: &str, value: Value) -> &Value {
        // tracing::info!("Saving {}: {:?}", name, new_obj);
        self.objectlike_data.insert(self.resolve_tag(name), value);
        self.get(name).unwrap()
    }

    fn get(&self, name: &str) -> Option<&Value> {
        self.objectlike_data.get(&self.resolve_tag(name))
    }

    fn get_or_save(&mut self, name: &str, val: Value) -> &Value {
        if !self.objectlike_data.contains_key(name) {
            self.save(name, val)
        } else {
            self.get(name).unwrap()
        }
    }

    fn make_tag(&mut self, name: &str, resolves_to: &str) {
        // tracing::info!("Made: ?{} => {}", name, resolves_to);
        self.do_make_attribute(name, RESOLVES_TO, Some(resolves_to.into()));
    }

    fn make_transient(&mut self, name: &str) {
        // tracing::info!("Making transient {}", name);
        self.make_object(name);
        self.transients.insert(self.resolve_tag(name));
    }

    fn resolve_tag(&self, name: &str) -> String {
        // Check that the object exists
        if let Some(Value::Object(map)) = self.objectlike_data.get(name) {
            // Check that the tag is not self-referential, and return resolving it
            if let Some(Value::String(tag)) = map.borrow_ref().unwrap().get(RESOLVES_TO) {
                if let Ok(tag) = tag.borrow_ref() {
                    let tag = tag.as_str();
                    assert!(tag != name, "{}", name);
                    return self.resolve_tag(tag);
                } else {
                    panic!("Could not acquire tag field on {} for comparison", name);
                }
            }
        }
        name.into()
    }
}

impl ContextLocalVariableActions for ParserContext {
    fn make_variable(&mut self, name: &str, val: &str) {
        // tracing::info!("Made: ({:?}, {:?})", name, val);
        // if let Some(overwritten) = self.variables.insert(name.into(), val.into()) {
        // tracing::warn!(
        //     "Warning: overwrote {} with {} for name {}",
        //     overwritten, val, name
        // );
        // }
        self.variables.insert(name.into(), val.into());
    }

    fn get_variable(&self, name: &str) -> Option<String> {
        let var = self.variables.get(name).cloned();
        // tracing::info!("Found: {:?}", var);
        var
    }

    fn clear_variables(&mut self) {
        self.variables.clear();
    }
}
