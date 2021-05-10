use super::{NodePattern, ParserContext};
use crate::ast::*;
use crate::prophet::*;

/// Defines how to parse an individual node that has been confirmed to be of interest
pub trait NodePatternParser<'a> {
    fn parse(&'a self, pattern: &mut NodePattern<'a>, ctx: &mut ParserContext);
}

fn write_to_context<'a>(
    to_match: &'a str,
    pattern: &mut NodePattern<'a>,
    ctx: &mut ParserContext,
) -> bool {
    if let Some(compiled_pattern) = pattern.compiled_pattern.as_mut() {
        if compiled_pattern.matches(to_match, ctx) {
            compiled_pattern.insert_match_result(ctx);
            return true;
        }
    }
    return false;
}

impl<'a> NodePatternParser<'a> for ClassOrInterfaceComponent {
    fn parse(&'a self, pattern: &mut NodePattern<'a>, ctx: &mut ParserContext) {
        write_to_context(&*self.component.component.instance_name, pattern, ctx);
    }
}

impl<'a> NodePatternParser<'a> for MethodComponent {
    fn parse(&'a self, pattern: &mut NodePattern<'a>, ctx: &mut ParserContext) {
        write_to_context(&*self.component.instance_name, pattern, ctx);
    }
}

impl<'a> NodePatternParser<'a> for MethodParamComponent {
    fn parse(&'a self, pattern: &mut NodePattern<'a>, ctx: &mut ParserContext) {
        write_to_context(&*self.component.instance_name, pattern, ctx);
    }
}

impl<'a> NodePatternParser<'a> for FieldComponent {
    fn parse(&'a self, pattern: &mut NodePattern<'a>, ctx: &mut ParserContext) {
        write_to_context(&*self.component.instance_name, pattern, ctx);
    }
}

impl<'a> NodePatternParser<'a> for DeclStmt {
    fn parse(&'a self, pattern: &mut NodePattern<'a>, ctx: &mut ParserContext) {
        //
    }
}

impl<'a> NodePatternParser<'a> for VarDecl {
    fn parse(&'a self, pattern: &mut NodePattern<'a>, ctx: &mut ParserContext) {
        write_to_context(&*self.ident.name, pattern, ctx);
    }
}

impl<'a> NodePatternParser<'a> for CallExpr {
    fn parse(&'a self, pattern: &mut NodePattern<'a>, ctx: &mut ParserContext) {
        //
    }
}

impl<'a> NodePatternParser<'a> for AnnotationComponent {
    fn parse(&'a self, pattern: &mut NodePattern<'a>, ctx: &mut ParserContext) {
        //
    }
}
