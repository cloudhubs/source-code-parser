use once_cell::sync::OnceCell;
use rune::{Diagnostics, Options, Sources};
use runestick::{FromValue, Source, Value, Vm};
use std::sync::Arc;

use super::{ContextLocalVariableActions, ContextObjectActions, NodePattern, ParserContext};

static EXEC: OnceCell<Executor> = OnceCell::new();

#[derive(Default)]
pub struct Executor {
    executor_ctx: runestick::Context,
}

impl Executor {
    pub fn new() -> runestick::Result<Executor> {
        // Create an empty Context. If we need more default functions, etc we can use
        // runestick::Context::default()
        let mut executor_ctx = runestick::Context::with_default_modules()?;
        let mut module = runestick::Module::default();

        // Register context type and methods
        module.ty::<ParserContext>()?;
        module.inst_fn("make_object", ParserContext::make_object)?;
        module.inst_fn("make_tag", ParserContext::make_tag)?;
        module.inst_fn("make_variable", ParserContext::make_variable)?;
        module.inst_fn("make_transient", ParserContext::make_transient)?;
        module.inst_fn("get_variable", ParserContext::get_variable)?;
        module.inst_fn("clear_variables", ParserContext::clear_variables)?;
        module.inst_fn("make_attribute", ParserContext::make_attribute)?;
        module.inst_fn("get_object", ParserContext::get_object)?;
        module.inst_fn("resolve_tag", ParserContext::resolve_tag)?;

        // Add more option methods
        module.inst_fn("as_ref", Option::<Value>::as_ref)?;
        module.inst_fn("clone", Option::<Value>::clone)?;
        executor_ctx.install(&module)?;

        Ok(Executor { executor_ctx })
    }

    pub fn execute(
        &self,
        pattern: &NodePattern,
        ctx: ParserContext,
    ) -> runestick::Result<ParserContext> {
        match &pattern.callback {
            Some(callback) => {
                let mut sources = Sources::new();
                let source = format!("pub fn main(ctx) {{ {} ctx }}", callback);
                sources.insert(Source::new("callback", source));

                let mut diagnostics = Diagnostics::new();

                let runtime = Arc::new(self.executor_ctx.runtime());
                let unit = Arc::new(rune::load_sources(
                    &self.executor_ctx,
                    &Options::default(),
                    &mut sources,
                    &mut diagnostics,
                )?);

                let vm = Vm::new(runtime, unit);
                // For some reason we have to pass ownership of the context and retrieve it back from a return value...
                // This is being done with a wrapper "main" function that calls the callback and returns the context again.
                let ret_val = vm.execute(&["main"], (ctx,))?.complete()?;
                let ctx = FromValue::from_value(ret_val)?;
                Ok(ctx)
            }
            None => Ok(ctx),
        }
    }

    pub fn get() -> &'static Executor {
        EXEC.get_or_init(|| {
            Executor::new()
                .map_err(|err| {
                    tracing::warn!("Failed to initialize callback executor: {:#?}", err);
                    err
                })
                .unwrap()
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::ressa::NodeType;

    use super::*;

    #[test]
    pub fn add_get_variables_from_script() {
        let pattern = NodePattern::new(
            NodeType::CallExpr,
            None,
            None,
            vec![],
            Some(
                r#"
            ctx.make_variable("foo", "bar");
            ctx.get_variable("foo");
            "#
                .into(),
            ),
            true,
            "".into(),
            None,
            false,
        );
        let mut ctx = ParserContext::default();
        let old = ctx.clone();
        ctx = Executor::get().execute(&pattern, ctx).unwrap();
        assert_ne!(old, ctx);
        assert_eq!("bar", ctx.get_variable("foo").unwrap())
    }

    #[test]
    pub fn add_get_variables_outside_script() {
        let mut ctx = ParserContext::default();
        ctx.make_variable("foo", "bar");
        let pattern = NodePattern::new(
            NodeType::CallExpr,
            None,
            None,
            vec![],
            Some(
                r#"
            if ctx.get_variable("foo").is_none() {
                panic("bad");
            }
            "#
                .into(),
            ),
            true,
            "".into(),
            None,
            false,
        );
        let old = ctx.clone();
        ctx = Executor::get().execute(&pattern, ctx).unwrap();
        assert_eq!(old, ctx);
        assert_eq!(
            old.get_variable("foo").unwrap(),
            ctx.get_variable("foo").unwrap()
        )
    }
}
