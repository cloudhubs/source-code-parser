use std::collections::HashMap;

const RESOLVES_TO: &'static str = "resolves_to";

/// Context used by the Parser, storing local variables (#{varname}) and objects/tags
struct ParserContext {
    variables: HashMap<String, HashMap<String, Option<String>>>,
}

/// Interface of the Context, offering ability to create, read, and update objects/tags
trait ContextObjectActions {
    fn make_obj(&mut self, name: &str);
    fn make_attribute(&mut self, name: &str, attr_name: &str, attr_type: Option<&str>);
    fn make_tag(&mut self, name: &str, resolves_to: &str);
    fn get_obj(&self, name: &str) -> Option<&HashMap<String, Option<String>>>;
}

impl ContextObjectActions for ParserContext {
    fn make_obj(&mut self, name: &str) {
        let obj_name: String = name.into();
        if !self.variables.contains_key(&obj_name) {
            (&mut self.variables).insert(obj_name, HashMap::new());
        }
    }

    fn make_attribute(&mut self, name: &str, attr_name: &str, attr_type: Option<&str>) {
        let obj_name: String = name.into();
        if !self.variables.contains_key(&obj_name) {
            eprintln!("Defining attribute on a non-existant object. Defining...");
            self.make_obj(name);
        }

        // Insert
        let attr_type = match attr_type {
            Some(val) => Some(val.into()),
            None => None,
        };
        let vars = self.variables.get_mut(&obj_name).unwrap();
        if vars.insert(attr_name.into(), attr_type).is_some() {
            eprintln!("Warning: overwriting an attribute!");
        }
    }

    fn make_tag(&mut self, name: &str, resolves_to: &str) {
        self.make_attribute(
            format!("?{}", name).as_str(),
            RESOLVES_TO,
            resolves_to.into(),
        );
    }

    fn get_obj(&self, name: &str) -> Option<&HashMap<String, Option<String>>> {
        if let Some(obj) = self.variables.get(name.into()) {
            if name.starts_with("?") {
                // Get the object. Extensive `expect`s because, if we don't have a RESOLVES_TO
                // attribute with a name the tag aliases on it, then we have serious data corruption
                // we shouldn't just ignore.
                self.get_obj(
                    obj.get(RESOLVES_TO)
                        .expect(format!("Invalid tag! no {} value!", RESOLVES_TO).as_str())
                        .as_ref()
                        .expect(format!("Invalid tag! no {} value!", RESOLVES_TO).as_str())
                        .as_str(),
                )
            } else {
                Some(obj)
            }
        } else {
            None
        }
    }
}
