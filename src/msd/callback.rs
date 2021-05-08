use rune::{Diagnostics, Options, Parser, Sources};
use runestick::{Call, FromValue, IntoTypeHash, Rtti, Source, UnsafeFromValue, Vm};
use std::sync::Arc;

use super::{ContextLocalVariableActions, ContextObjectActions, NodePattern, ParserContext};

#[derive(Default)]
pub struct Executor {
    executor_ctx: runestick::Context,
}

impl Executor {
    pub fn new() -> runestick::Result<Executor> {
        let mut executor_ctx = runestick::Context::default();
        let mut module = runestick::Module::default();
        // TODO: install usable types, functions, etc from callback scripts
        /*
        // Installing a function
        module.function(&["add"], |a: i64| a + 1)?;
        */

        // Registering a type and methods / associated type functions
        module.ty::<ParserContext>()?;
        // module.function(&["MyBytes", "new"], MyBytes::new)?;
        module.inst_fn("make_object", ParserContext::make_object)?;
        module.inst_fn("make_tag", ParserContext::make_tag)?;
        module.inst_fn("make_variable", ParserContext::make_variable)?;
        module.inst_fn("get_variable", ParserContext::get_variable)?;
        module.inst_fn("clear_variables", ParserContext::clear_variables)?;
        module.inst_fn("make_attribute", ParserContext::make_attribute)?;

        // This seems to be complaining because of lifetimes. I fixed this with make_attribute by
        // making the attr_type an owned String since we cloned it to one anyways, but I'm not
        // sure what to do here.
        // module.inst_fn("get_object", ParserContext::get_object)?;

        executor_ctx.install(&module)?;
        Ok(Executor { executor_ctx })
    }

    pub fn execute(
        &self,
        pattern: &NodePattern<'_>,
        ctx: ParserContext,
    ) -> runestick::Result<ParserContext> {
        let mut sources = Sources::new();
        let main_fn = r#"pub fn main(ctx) { callback(ctx); ctx }"#;
        let source = format!("{}\n{}", main_fn, pattern.callback);
        sources.insert(Source::new("callback", source));

        let mut diagnostics = Diagnostics::new();

        let unit = rune::load_sources(
            &self.executor_ctx,
            &Options::default(),
            &mut sources,
            &mut diagnostics,
        )?;

        let vm = Vm::new(Arc::new(self.executor_ctx.runtime()), Arc::new(unit));
        // For some reason we have to pass ownership of the context and retrieve it back from a return value...
        // This is being done with a wrapper "main" function that calls the callback and returns the context again.
        let ret_val = vm.execute(&["main"], (ctx,))?.complete()?;
        let ctx = FromValue::from_value(ret_val)?;
        Ok(ctx)
    }
}

// impl runestick::Named for ParserContext {
//     const NAME: runestick::RawStr = runestick::RawStr::from_str("ParserContext");
// }

// impl runestick::TypeOf for ParserContext {
//     fn type_hash() -> runestick::Hash {
//         runestick::Hash::type_hash(&["source_code_parser", "msd", "ParserContext"])
//     }

//     fn type_info() -> runestick::TypeInfo {
//         // TODO: How to create a runestick::Rtti?
//         // runestick::TypeInfo::Typed(Arc::new(/* runestick::Rtti */))
//         todo!()
//     }
// }

// impl runestick::InstallWith for ParserContext {}
