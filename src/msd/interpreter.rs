use super::{CompiledPattern, NodePattern};
use super::{ContextLocalVariableActions, ParserContext};
use crate::ast::*;
use crate::prophet::*;

/// Defines how to parse an individual node that has been confirmed to be of interest
pub trait NodePatternParser {
    fn parse(&self, pattern: &NodePattern, ctx: &mut ParserContext);
}

impl NodePatternParser for ClassOrInterfaceComponent {
    fn parse(&self, pattern: &NodePattern, ctx: &mut ParserContext) {
        //
    }
}

impl NodePatternParser for MethodComponent {
    fn parse(&self, pattern: &NodePattern, ctx: &mut ParserContext) {
        //
    }
}

impl NodePatternParser for MethodParamComponent {
    fn parse(&self, pattern: &NodePattern, ctx: &mut ParserContext) {
        //
    }
}

impl NodePatternParser for FieldComponent {
    fn parse(&self, pattern: &NodePattern, ctx: &mut ParserContext) {
        //
    }
}

impl NodePatternParser for DeclStmt {
    fn parse(&self, pattern: &NodePattern, ctx: &mut ParserContext) {
        //
    }
}

impl NodePatternParser for VarDecl {
    fn parse(&self, pattern: &NodePattern, ctx: &mut ParserContext) {
        //
    }
}

impl NodePatternParser for CallExpr {
    fn parse(&self, pattern: &NodePattern, ctx: &mut ParserContext) {
        //
    }
}

impl NodePatternParser for AnnotationComponent {
    fn parse(&self, pattern: &NodePattern, ctx: &mut ParserContext) {
        //
    }
}
