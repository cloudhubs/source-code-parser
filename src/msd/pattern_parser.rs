use super::NodeType;
use super::{CompiledPattern, MsdNodeExplorer, NodePattern, ParserContext};
use crate::prophet::*;
use crate::{ast::*, explore_all};

/// Defines how to parse an individual node that has been confirmed to be of interest
pub trait NodePatternParser {
    fn parse(&mut self, pattern: &mut NodePattern<'_>, ctx: &mut ParserContext) -> Option<()>;
}

fn write_to_context(
    to_match: &String,
    essential: bool,
    pattern: &mut Option<CompiledPattern<'_>>,
    ctx: &mut ParserContext,
) -> Option<()> {
    if let Some(compiled_pattern) = pattern.as_mut() {
        if compiled_pattern.match_and_insert(&*to_match.clone(), ctx) {
            return Some(());
        }
    }
    if essential {
        None
    } else {
        Some(())
    }
}

fn match_subsequence<T: MsdNodeExplorer>(
    params: &mut Vec<&mut NodePattern>,
    explorable: &mut Vec<T>,
    ctx: &mut ParserContext,
) -> Option<()> {
    let (mut start, mut end) = (0 as usize, params.len());
    let mut matched = true;

    while end < explorable.len() {
        // Pre
        matched = true;
        let mut pattern_iter = params.iter_mut();

        // Perform subsequence matching
        for i in start..end {
            let pattern = pattern_iter.next()?;
            if explorable.get_mut(i)?.explore(pattern, ctx).is_none() {
                matched = false;
                break;
            }
        }

        // Post
        if !matched {
            start += 1;
            end += 1;
        } else {
            break;
        }
    }

    // Determine if we matched the pattern at some point
    if matched {
        Some(())
    } else {
        None
    }
}

fn verify_match(match_str: &str, pattern: &NodePattern<'_>, ctx: &ParserContext) -> Option<()> {
    if let Some(compiled) = &pattern.compiled_pattern {
        if !compiled.matches(match_str, ctx) {
            return None;
        }
    }
    Some(())
}

impl NodePatternParser for ClassOrInterfaceComponent {
    fn parse(&mut self, pattern: &mut NodePattern<'_>, ctx: &mut ParserContext) -> Option<()> {
        verify_match(&self.component.container_name, pattern, ctx)?;

        // Check subpatterns
        for pattern in pattern.subpatterns.iter_mut() {
            explore_all!(
                pattern,
                ctx,
                self.annotations,
                self.constructors,
                self.field_components
            )?;
        }

        // If all subpatterns matched, extract context
        write_to_context(
            &self.component.component.instance_name,
            pattern.essential,
            &mut pattern.compiled_pattern,
            ctx,
        )
    }
}

impl NodePatternParser for MethodComponent {
    fn parse(&mut self, pattern: &mut NodePattern<'_>, ctx: &mut ParserContext) -> Option<()> {
        verify_match(&self.component.instance_name, pattern, ctx)?;

        // Match method parameters
        let mut params = pattern
            .subpatterns
            .iter_mut()
            .filter(|child| match child.identifier {
                crate::msd::NodeType::MethodParam => true,
                _ => false,
            })
            .collect::<Vec<&mut NodePattern>>();

        match_subsequence(&mut params, &mut self.parameters, ctx)?;

        // Search unordered parts of the signature
        for pattern in pattern
            .subpatterns
            .iter_mut()
            .filter(|child| match child.identifier {
                crate::msd::NodeType::MethodParam => false,
                _ => true,
            })
        {
            explore_all!(pattern, ctx, self.annotations, self.sub_methods)?;
        }

        write_to_context(
            &self.component.instance_name,
            pattern.essential,
            &mut pattern.compiled_pattern,
            ctx,
        )
    }
}

impl NodePatternParser for MethodParamComponent {
    fn parse(&mut self, pattern: &mut NodePattern<'_>, ctx: &mut ParserContext) -> Option<()> {
        // Verify
        verify_match(&*self.parameter_name, pattern, ctx)?;
        if let Some(type_pattern) = &pattern.compiled_type_pattern {
            if !type_pattern.matches(&self.r#type, ctx) {
                return None;
            }
        }

        // Verify subpatterns
        if let Some(annotations) = &mut self.annotation {
            explore_all!(pattern, ctx, annotations)?;
        }

        // Write and return
        write_to_context(
            &self.parameter_name,
            pattern.essential,
            &mut pattern.compiled_pattern,
            ctx,
        )?;
        write_to_context(
            &self.r#type,
            pattern.essential,
            &mut pattern.compiled_type_pattern,
            ctx,
        )
    }
}

impl NodePatternParser for FieldComponent {
    fn parse(&mut self, pattern: &mut NodePattern<'_>, ctx: &mut ParserContext) -> Option<()> {
        Some(())
    }
}

impl NodePatternParser for DeclStmt {
    fn parse(&mut self, pattern: &mut NodePattern<'_>, ctx: &mut ParserContext) -> Option<()> {
        Some(())
    }
}

impl NodePatternParser for VarDecl {
    fn parse(&mut self, pattern: &mut NodePattern<'_>, ctx: &mut ParserContext) -> Option<()> {
        Some(())
    }
}

impl NodePatternParser for CallExpr {
    fn parse(&mut self, pattern: &mut NodePattern<'_>, ctx: &mut ParserContext) -> Option<()> {
        let raw_name = match *self.name {
            Expr::DotExpr(DotExpr { ref selected, .. }) => match selected.as_ref() {
                Expr::Ident(Ident { ref name, .. }) => name,
                ref unknown => {
                    eprintln!("Currently unhandled CallExpression name {:?}", unknown);
                    return None;
                }
            },
            Expr::Ident(Ident { ref name, .. }) => &name,
            Expr::Literal(Literal { ref value, .. }) => &value,
            ref unknown => {
                eprintln!("Currently unhandled CallExpression name {:?}", unknown);
                return None;
            }
        };
        verify_match(&*raw_name, pattern, ctx)?;

        // Match method parameters
        let mut params = pattern
            .subpatterns
            .iter_mut()
            .filter(|child| match child.identifier {
                NodeType::CallExpr | NodeType::VarDecl | NodeType::Ident | NodeType::Literal => {
                    true
                }
                _ => false,
            })
            .collect::<Vec<&mut NodePattern>>();
        match_subsequence(&mut params, &mut self.args, ctx)?;

        // Extract to context
        write_to_context(
            raw_name,
            pattern.essential,
            &mut pattern.compiled_pattern,
            ctx,
        )
    }
}

impl NodePatternParser for AnnotationComponent {
    fn parse(&mut self, pattern: &mut NodePattern<'_>, ctx: &mut ParserContext) -> Option<()> {
        // Verify
        verify_match(&*self.name, pattern, ctx)?;
        if let Some(aux_pattern) = &pattern.compiled_type_pattern {
            if !aux_pattern.matches(&*self.value, ctx) {
                return None;
            }
        }

        // Verify subpatterns
        explore_all!(pattern, ctx, &mut self.key_value_pairs)?;

        // Write and return
        write_to_context(
            &self.name,
            pattern.essential,
            &mut pattern.compiled_pattern,
            ctx,
        )?;
        write_to_context(
            &self.value,
            pattern.essential,
            &mut pattern.compiled_pattern,
            ctx,
        )
    }
}

impl NodePatternParser for AnnotationValuePair {
    fn parse(&mut self, pattern: &mut NodePattern<'_>, ctx: &mut ParserContext) -> Option<()> {
        verify_match(&self.key, pattern, ctx)?;
        if let Some(aux_pattern) = &pattern.compiled_type_pattern {
            if !aux_pattern.matches(&*self.value, ctx) {
                return None;
            }
        }
        write_to_context(
            &self.value,
            pattern.essential,
            &mut pattern.compiled_pattern,
            ctx,
        )?;
        write_to_context(
            &self.key,
            pattern.essential,
            &mut pattern.compiled_pattern,
            ctx,
        )
    }
}

impl NodePatternParser for Ident {
    fn parse(&mut self, pattern: &mut NodePattern<'_>, ctx: &mut ParserContext) -> Option<()> {
        write_to_context(
            &self.name,
            pattern.essential,
            &mut pattern.compiled_pattern,
            ctx,
        )
    }
}

impl NodePatternParser for Literal {
    fn parse(&mut self, pattern: &mut NodePattern<'_>, ctx: &mut ParserContext) -> Option<()> {
        write_to_context(
            &self.value,
            pattern.essential,
            &mut pattern.compiled_pattern,
            ctx,
        )
    }
}
