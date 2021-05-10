use super::{CompiledPattern, MsdNodeExplorer, NodePattern, ParserContext};
use crate::prophet::*;
use crate::{ast::*, explore_all};

/// Defines how to parse an individual node that has been confirmed to be of interest
pub trait NodePatternParser {
    fn parse(&mut self, pattern: &mut NodePattern<'_>, ctx: &mut ParserContext);
}

fn write_to_context(
    to_match: &str,
    pattern: &mut Option<CompiledPattern<'_>>,
    ctx: &mut ParserContext,
) -> bool {
    if let Some(compiled_pattern) = pattern.as_mut() {
        compiled_pattern.match_and_insert(to_match, ctx)
    } else {
        false
    }
}

impl NodePatternParser for ClassOrInterfaceComponent {
    fn parse(&mut self, pattern: &mut NodePattern<'_>, ctx: &mut ParserContext) {
        write_to_context(
            &*self.component.component.instance_name.clone(),
            &mut pattern.compiled_pattern,
            ctx,
        );

        // Explore all
        explore_all!(
            pattern,
            ctx,
            self.annotations,
            self.constructors,
            self.field_components
        );
    }
}

impl NodePatternParser for MethodComponent {
    fn parse(&mut self, pattern: &mut NodePattern<'_>, ctx: &mut ParserContext) {
        write_to_context(
            &*self.component.instance_name.clone(),
            &mut pattern.compiled_pattern,
            ctx,
        );
    }
}

impl NodePatternParser for MethodParamComponent {
    fn parse(&mut self, pattern: &mut NodePattern<'_>, ctx: &mut ParserContext) {
        //
    }
}

impl NodePatternParser for FieldComponent {
    fn parse(&mut self, pattern: &mut NodePattern<'_>, ctx: &mut ParserContext) {
        //
    }
}

impl NodePatternParser for DeclStmt {
    fn parse(&mut self, pattern: &mut NodePattern<'_>, ctx: &mut ParserContext) {
        //
    }
}

impl NodePatternParser for VarDecl {
    fn parse(&mut self, pattern: &mut NodePattern<'_>, ctx: &mut ParserContext) {
        //
    }
}

impl NodePatternParser for CallExpr {
    fn parse(&mut self, pattern: &mut NodePattern<'_>, ctx: &mut ParserContext) {
        //
    }
}

impl NodePatternParser for AnnotationComponent {
    fn parse(&mut self, pattern: &mut NodePattern<'_>, ctx: &mut ParserContext) {
        //
    }
}
