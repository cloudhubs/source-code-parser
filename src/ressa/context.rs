use rune::{
    runtime::{Object, Shared, Value},
    Any,
};
use std::collections::HashMap;

use super::RessaResult;

/// Special attribute that's value is the name that a tag should resolve to
const RESOLVES_TO: &str = "???";

/// Special attribute indicating an object is transient
const TRANSIENT: &str = "";

#[allow(clippy::enum_variant_names)] // Certain errors are forwarding errors from Rune, named accordingly
#[derive(thiserror::Error, Debug, Any)]
pub enum Error {
    #[error("Unexpected value type: {0:?}")]
    UnexpectedType(Value),
    #[error("Requested value not present")]
    NotFound,
    #[error("Rune VM error: {0:?}")]
    RuneError(#[from] rune::runtime::VmError),
    #[error("Rune access error: {0:?}")]
    AccessError(#[from] rune::runtime::AccessError),
}

/// Context used by the Parser, storing local variables (#{varname}) and objects/tags
#[derive(Default, Debug, Clone, Any)]
pub struct ParserContext {
    objectlike_data: HashMap<String, Value>,
    variables: HashMap<String, String>,
    pub frame_number: i32,
}

/// Convert the context into the result format
impl From<ParserContext> for RessaResult {
    fn from(ctx: ParserContext) -> Self {
        ctx.objectlike_data
            .into_iter()
            .filter_map(|(name, val)| filter_map_value(val).map(|val| (name, val)))
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
            if !obj.contains_key(RESOLVES_TO) && !obj.contains_key(TRANSIENT) {
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
    fn make_object(&mut self, name: &str);
    fn save_object(&mut self, name: &str, new_obj: &Object);
    fn make_tag(&mut self, name: &str, resolves_to: &str);
    /// Makes an existing object transient, or initializes a new object as transient
    fn make_transient(&mut self, name: &str);
    /// Gets an object by name. Modify this object as you see fit, then persist with ContextObjectActions::save_object
    fn get_object(&self, name: &str) -> Result<Object, Error>;
    /// Shorthand for `ctx.make_object(name); let x = ctx.get_object(name);`
    fn get_or_create_object(&mut self, name: &str) -> Object;
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
}

impl ContextObjectActions for ParserContext {
    fn make_object(&mut self, name: &str) {
        let obj_name: String = name.into();
        if !self.objectlike_data.contains_key(&obj_name) {
            // tracing::info!("Making: {}", obj_name);
            (&mut self.objectlike_data).insert(obj_name, Object::new().into());
        }
    }

    fn save_object(&mut self, name: &str, new_obj: &Object) {
        // tracing::info!("Saving {}: {:?}", name, new_obj);
        self.objectlike_data
            .insert(self.resolve_tag(name), new_obj.clone().into());
    }

    fn make_tag(&mut self, name: &str, resolves_to: &str) {
        // tracing::info!("Made: ?{} => {}", name, resolves_to);
        self.do_make_attribute(name, RESOLVES_TO, Some(resolves_to.into()));
    }

    fn make_transient(&mut self, name: &str) {
        // tracing::info!("Making transient {}", name);
        self.make_object(name);
        self.do_make_attribute(self.resolve_tag(name).as_str(), TRANSIENT, None);
    }

    fn get_object(&self, name: &str) -> Result<Object, Error> {
        // tracing::info!("Looking for {}...", name);
        let name = self.resolve_tag(name);

        match self.objectlike_data.get(&name) {
            Some(Value::Object(obj)) => {
                let borrowed = obj.borrow_ref()?;
                Ok(borrowed.clone())
            }
            Some(val) => Err(Error::UnexpectedType(val.clone())),
            None => Err(Error::NotFound),
        }
        // tracing::info!("Retrieved {} Found {:?}", name, obj);
    }

    fn get_or_create_object(&mut self, name: &str) -> Object {
        self.make_object(name);
        self.get_object(name).unwrap() // Guaranteed safe, just made
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
