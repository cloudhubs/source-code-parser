use std::cell::{Ref, RefCell};

use derive_new::new;
use regex::{Captures, Regex};

use super::{ContextLocalVariableActions, ContextObjectActions, ParserContext};

/// A wrapper around a compiled enhanced Regex (supporting named variable capture)
#[derive(Debug, Clone, new)]
pub struct CompiledPattern {
    pub pattern: Regex,
    pub variables: Vec<String>,
    reference_vars: Vec<String>,
}

/// Generate a new compiled pattern
impl CompiledPattern {
    /// Create a compiled pattern from a ReSSA pattern and the context.
    pub fn from_pattern(pattern: &str) -> Result<CompiledPattern, regex::Error> {
        /*
         * Expanding and explaining the following Regex for posterity:
         *
         * #(&)?              - Variable declared with #, maybe as a reference (&)
         * \{                 1 Followed by literal curly braces
         *   ([a-zA-Z_-]+)    - Wrapping a capture of the variable's name
         * \}                 1
         * (                  1 Optionally capture
         *   \(               2 Something wrapped in parenthesis literals
         *     (              3 Containing the desired RegEx pattern consisting of
         *       (            4 Any number of
         *         [^(]       - Characters other than opening parentheses
         *         |\\\(      - Or escaped opening parenthesis
         *         |\(        5 Or a clause wrapped in literal open/close parenthesis
         *           (        6 Containing any number of
         *             [^)]   - characters other than closing parentheses
         *             |\\\)  - Or escaped closing parenthesis
         *           )*       6
         *         \)         5
         *       )*           4
         *     )              3
         *   \)               2
         * )?                 1
         */
        let tag_regex =
            Regex::new(r#"#(&)?\{([a-zA-Z_-]+)\}(\((([^(]|\\\(|\(([^)]|\\\))*\))*)\))?"#)?;

        let mut variables = vec![];
        let mut references = vec![];

        let matches = tag_regex.captures_iter(pattern);
        let mut pattern: String = pattern.into();
        for captures in matches {
            let is_ref = captures.get(1).is_some();
            let name = captures.get(2).expect("Variable must have a name").as_str();
            let s = captures
                .get(0)
                .expect("Entire pattern should have matched")
                .as_str();

            // Find a user-defined capture definition
            let mut regex_pattern = ".*";
            if let Some(group) = &captures.get(4) {
                regex_pattern = group.as_str();
            }

            // Replace the capture with the processed capture group
            pattern = pattern
                .as_str()
                .replace(s, &format!("(?P<{}>{})", name, regex_pattern));

            // Register variables and references
            if is_ref {
                references.push(name.into());
            }
            variables.push(name.into());
        }
        let transformed_pattern = Regex::new(pattern.as_str())?;
        Ok(CompiledPattern::new(
            transformed_pattern,
            variables,
            references,
        ))
    }

    pub fn primitive_matches<'a>(&self, match_str: &'a str) -> Option<Captures<'a>> {
        self.pattern.clone().captures(&*match_str)
    }

    /// Returns a callback that records all matches to the context, if there was a match
    pub fn match_callback<'a>(
        &'a self,
        match_str: &'a str,
        ctx: &'a mut ParserContext,
    ) -> Option<Box<dyn FnMut() + 'a>> {
        match self.primitive_matches(match_str) {
            Some(matches) => {
                for reference in self.reference_vars.iter() {
                    if ctx
                        .get(
                            matches
                                .name(reference)
                                .expect("Reference variable name not found in regex!")
                                .as_str(),
                        )
                        .is_none()
                    {
                        tracing::info!(
                            "Failed to find {}={:?}",
                            reference,
                            matches.name(reference)
                        );
                        return None;
                    }
                }
                Some(Box::new(move || {
                    for var_name in self.variables.iter() {
                        if let Some(val) = matches.name(var_name) {
                            ctx.make_variable(var_name, val.as_str())
                        }
                        // ctx.make_variable(
                        //     var_name,
                        //     matches
                        //         .name(var_name)
                        //         .expect("Failed to match a variable name")
                        //         .clone()
                        //         .as_str(),
                        // );
                    }
                }))
            }
            None => None,
        }
    }

    pub fn matches(&self, match_str: &str, ctx: &ParserContext) -> bool {
        match self.primitive_matches(match_str) {
            Some(matches) => {
                for reference in self.reference_vars.iter() {
                    if ctx
                        .get(
                            matches
                                .name(reference)
                                .expect("Reference variable name not found in regex!")
                                .as_str(),
                        )
                        .is_none()
                    {
                        tracing::info!(
                            "Failed to find {}={:?}",
                            reference,
                            matches.name(reference)
                        );
                        return false;
                    }
                }
                true
            }
            None => false,
        }
    }

    pub fn match_and_insert(&self, match_str: &str, ctx: &mut ParserContext) -> bool {
        let tmp_pattern = self.pattern.clone();
        match tmp_pattern.captures(&*match_str) {
            Some(matches) => {
                // Verify all references
                for reference in self.reference_vars.iter() {
                    if ctx
                        .get(
                            matches
                                .name(reference)
                                .expect("Reference variable name not found in regex!")
                                .as_str(),
                        )
                        .is_none()
                    {
                        return false;
                    }
                }

                // Extract variables to context
                for var_name in self.variables.iter() {
                    if let Some(val) = matches.name(var_name) {
                        ctx.make_variable(var_name, val.as_str())
                    }
                    // ctx.make_variable(
                    //     var_name,
                    //     matches
                    //         .name(var_name)
                    //         .expect("Failed to match a variable name")
                    //         .clone()
                    //         .as_str(),
                    // );
                }
                true
            }
            None => false,
        }
    }

    /// Optionally compile a provided pattern, if possible
    pub fn try_new(pattern: &str) -> Option<CompiledPattern> {
        let compiled_result = CompiledPattern::from_pattern(pattern);
        match compiled_result {
            Ok(compiled_result) => Some(compiled_result),
            Err(error) => {
                tracing::warn!("Error compiling pattern: {:#?}", error);
                None
            }
        }
    }
}

#[derive(Default, Clone, Debug)]
pub struct CachedCompiledPattern {
    inner: RefCell<Option<CompiledPattern>>,
}

impl CachedCompiledPattern {
    pub fn borrow(&self) -> Ref<Option<CompiledPattern>> {
        self.inner.borrow()
    }

    pub fn get_or_compile(&self, pattern: &str) -> Ref<Option<CompiledPattern>> {
        if self.inner.borrow().is_none() {
            let mut compiled_pattern = self.inner.borrow_mut();
            *compiled_pattern = CompiledPattern::try_new(pattern);
        }
        self.borrow()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bson() {
        assert_eq!(
            "BSON_APPEND_(?P<type>.*)_(?P<another_one>.*)",
            CompiledPattern::from_pattern("BSON_APPEND_#{type}_#&{another_one}")
                .expect("Failed to construct regex from pattern input")
                .pattern
                .as_str()
        );
    }

    #[test]
    fn test_tt() {
        assert_eq!(
            "(https?://)(?P<host>[^/]*)/(?P<path_var_val>.+)/?\"",
            CompiledPattern::from_pattern("(https?://)#{host}([^/]*)/#{path_var_val}(.+)/?\"")
                .expect("Failed to construct regex from pattern input")
                .pattern
                .as_str()
        );
    }
}
