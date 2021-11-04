use super::{CompiledPattern, NodePattern, ParserContext, RessaNodeExplorer};
use super::{LaastIndex, NodeType};
use crate::ast::*;
use crate::{prophet::*, ressa::choose_exit};
use itertools::Itertools;

/// Defines how to parse an individual node that has been confirmed to be of interest
pub trait NodePatternParser {
    fn parse(
        &self,
        pattern: &mut NodePattern,
        ctx: &mut ParserContext,
        index: &LaastIndex,
    ) -> Option<()>;
}

fn write_to_context(
    to_match: &String,
    essential: bool,
    pattern: &mut Option<CompiledPattern>,
    ctx: &mut ParserContext,
) -> Option<()> {
    if let Some(compiled_pattern) = pattern.as_mut() {
        if compiled_pattern.match_and_insert(&*to_match.clone(), ctx) {
            return Some(());
        } else if essential {
            None
        } else {
            Some(())
        }
    } else {
        Some(())
    }
}

fn match_subsequence<T: RessaNodeExplorer>(
    params: &mut Vec<&mut NodePattern>,
    explorable: &Vec<T>,
    ctx: &mut ParserContext,
    index: &LaastIndex,
) -> Option<()> {
    let (mut start, mut end) = (0 as usize, params.len());
    // let mut matched = true;

    while start < explorable.len() {
        // Pre
        //        matched = true;
        let mut pattern_iter = params.iter_mut();

        // Perform subsequence matching
        for i in start..end {
            let pattern = pattern_iter.next()?;
            if explorable.get(i)?.explore(pattern, ctx, index).is_none() {
                //                matched = false;
                break;
            }
        }

        // Post
        // if !matched {
        start += 1;
        if end < explorable.len() {
            end += 1;
        }
        // } else {
        //     break;
        // }
    }

    // Determine if we matched the pattern at some point
    // if matched {
    //     Some(())
    // } else {
    //     None
    // }
    Some(())
}

/// Verify if an Option<Regex> matches a specific string; if it fails, exits
macro_rules! verify_match {
    ( $match_str:expr, $pattern:expr, $ctx:expr, $essential:expr ) => {
        if let Some(compiled) = $pattern {
            if !compiled.matches($match_str, $ctx) {
                quit!($essential);
            }
        }
    };
}

/// Exit a search, voting to abort or continue
macro_rules! quit {
    ( $essential:expr ) => {
        if $essential {
            return None;
        } else {
            return Some(());
        }
    };
}

/// Explore all elements in the provided collections
#[macro_export]
macro_rules! explore_all {
    ( $pattern:expr, $ctx:expr, $index:expr, $( $explorable:expr ),+ ) => {{
        use crate::ressa::explorer::choose_exit;

        let mut explore_all_found_essential = false;
         $(
            for x in $explorable.iter() {
                if x.explore($pattern, $ctx, $index).is_some() {
                    explore_all_found_essential = true;

                 }
             }
         )*
        choose_exit($pattern.essential, explore_all_found_essential)
    }};
}

/// Explores all subpatterns, verifying each essential pattern matches
#[macro_export]
macro_rules! explore_all_subpatterns {
    ( $subpatterns:expr, $ctx:expr, $index:expr, $( $explorable:expr ),+ ) => {
        use crate::ressa::explorer::choose_exit;

        for subpattern in $subpatterns.iter_mut() {
            let mut explore_all_found_essential = false;
            $(
                if explore_all!(subpattern, $ctx, $index, $explorable).is_some() {
                    explore_all_found_essential = true;
                }
            )*
            choose_exit(subpattern.essential, explore_all_found_essential)?;
        }
    };
}

impl NodePatternParser for ClassOrInterfaceComponent {
    fn parse(
        &self,
        pattern: &mut NodePattern,
        ctx: &mut ParserContext,
        index: &LaastIndex,
    ) -> Option<()> {
        // If all subpatterns matched, extract context
        write_to_context(
            &self.component.container_name,
            pattern.essential,
            &mut pattern.compiled_pattern,
            ctx,
        )?;

        // Check subpatterns
        explore_all_subpatterns!(
            pattern.subpatterns,
            ctx,
            index,
            self.annotations,
            self.constructors,
            self.field_components,
            self.component.methods
        );
        Some(())
    }
}

impl NodePatternParser for MethodComponent {
    fn parse(
        &self,
        pattern: &mut NodePattern,
        ctx: &mut ParserContext,
        index: &LaastIndex,
    ) -> Option<()> {
        write_to_context(
            &self.component.instance_name,
            pattern.essential,
            &mut pattern.compiled_pattern,
            ctx,
        )?;

        write_to_context(
            &self.return_type,
            pattern.essential,
            &mut pattern.compiled_auxiliary_pattern,
            ctx,
        )?;

        // Match method parameters
        let mut params = pattern
            .subpatterns
            .iter_mut()
            .filter(|child| match child.identifier {
                crate::ressa::NodeType::MethodParam => true,
                _ => false,
            })
            .collect::<Vec<&mut NodePattern>>();
        match_subsequence(&mut params, &self.parameters, ctx, index)?;

        // If there's a method body, explore it
        let mut tmp = vec![];
        let body_nodes = if let Some(body) = &self.body {
            &body.nodes
        } else {
            &mut tmp
        };

        // Search unordered parts of the signature
        for pattern in pattern
            .subpatterns
            .iter_mut()
            .filter(|child| match child.identifier {
                crate::ressa::NodeType::MethodParam => false,
                _ => true,
            })
        {
            explore_all!(
                pattern,
                ctx,
                index,
                self.annotations,
                self.sub_methods,
                body_nodes
            )?;
        }
        Some(())
    }
}

impl NodePatternParser for MethodParamComponent {
    fn parse(
        &self,
        pattern: &mut NodePattern,
        ctx: &mut ParserContext,
        index: &LaastIndex,
    ) -> Option<()> {
        // Verify
        // Ensure primary pattern matches, then extract auxiliary pattern (exiting if fail), finally extract main pattern
        // (done to cut one regex match)
        verify_match!(
            &*self.parameter_name,
            &pattern.compiled_pattern,
            ctx,
            pattern.essential
        );
        write_to_context(
            &self.r#type,
            pattern.essential,
            &mut pattern.compiled_auxiliary_pattern,
            ctx,
        )?;
        write_to_context(
            &self.parameter_name,
            pattern.essential,
            &mut pattern.compiled_pattern,
            ctx,
        )?;

        // If there are annotation subnodes, check if they match the subpatterns--then exit
        // Otherwise, exit based on whether there are any essential subnodes
        if let Some(annotations) = &self.annotation {
            explore_all_subpatterns!(pattern.subpatterns, ctx, index, annotations);
            Some(())
        } else {
            choose_exit(
                pattern
                    .subpatterns
                    .iter()
                    .find(|pat| pat.essential)
                    .is_some(),
                false,
            )
        }
    }
}

impl NodePatternParser for FieldComponent {
    fn parse(
        &self,
        pattern: &mut NodePattern,
        ctx: &mut ParserContext,
        index: &LaastIndex,
    ) -> Option<()> {
        // Verify
        verify_match!(
            &*self.field_name,
            &pattern.compiled_pattern,
            ctx,
            pattern.essential
        );
        write_to_context(
            &self.r#type,
            pattern.essential,
            &mut pattern.compiled_auxiliary_pattern,
            ctx,
        )?;
        write_to_context(
            &self.field_name,
            pattern.essential,
            &mut pattern.compiled_pattern,
            ctx,
        )?;
        let mut expr_vec = Vec::new();
        if let Some(expr) = &self.expression {
            expr_vec.push(expr);
        }

        // Verify subpatterns
        explore_all_subpatterns!(
            pattern.subpatterns,
            ctx,
            index,
            self.annotations,
            expr_vec,
            self.variables
                .iter()
                .map(|var| Ident::new(var.clone(), self.component.language))
                .collect::<Vec<Ident>>()
        );
        Some(())
    }
}

impl NodePatternParser for DeclStmt {
    fn parse(
        &self,
        pattern: &mut NodePattern,
        ctx: &mut ParserContext,
        index: &LaastIndex,
    ) -> Option<()> {
        let mut decl_patterns: Vec<usize> = vec![];
        let mut non_decl: Vec<usize> = vec![];
        for (i, pattern) in pattern.subpatterns.iter().enumerate() {
            match pattern.identifier {
                NodeType::VarDecl => decl_patterns.push(i),
                _ => non_decl.push(i),
            }
        }

        if decl_patterns.len() == non_decl.len() {
            // Case 2: equal lengths

            // If can't be right, return
            if self.variables.len() != self.expressions.len() || self.variables.len() == 0 {
                quit!(pattern.essential);
            }

            // Analyze pattern
            for pattern_index in 0..decl_patterns.len() {
                for i in 0..self.variables.len() {
                    // Parse variable declarations
                    self.variables.get(i)?.explore(
                        pattern
                            .subpatterns
                            .get_mut(*decl_patterns.get(pattern_index)?)?,
                        ctx,
                        index,
                    )?;

                    // Parse equality
                    let non_decl_pattern = pattern
                        .subpatterns
                        .get_mut(*non_decl.get_mut(pattern_index)?)?;
                    if let Some(expr) = self.expressions.get(i)?.as_ref() {
                        expr.explore(non_decl_pattern, ctx, index)?;
                    } else if non_decl_pattern.essential {
                        // If we needed that righthand side, then abort the search
                        return None;
                    }
                }
            }
        } else if decl_patterns.len() == 1 {
            // Case 1: one Decl no non-decls
            let pattern = pattern.subpatterns.get_mut(*decl_patterns.iter().next()?)?;
            explore_all!(pattern, ctx, index, self.variables)?;
        } else {
            // Case 3: multiple non-Decls to fewer Decls
            let real_expressions = self
                .expressions
                .iter()
                .flat_map(|c| c)
                .collect::<Vec<&Expr>>();
            match_subsequence(
                // Get the actual pattern references from the subpatterns array using the indices in decl_patterns (I'm sorry.)
                &mut pattern
                    .subpatterns
                    .iter_mut()
                    .enumerate()
                    .filter(|(ndx, _)| decl_patterns.contains(ndx))
                    .map(|(_, p)| p)
                    .collect_vec(),
                &self.variables,
                ctx,
                index,
            )?;
            for ndx in non_decl.iter_mut() {
                let pattern = pattern.subpatterns.get_mut(*ndx)?;
                explore_all!(pattern, ctx, index, real_expressions)?;
            }
        }
        Some(())
    }
}

impl NodePatternParser for VarDecl {
    fn parse(
        &self,
        pattern: &mut NodePattern,
        ctx: &mut ParserContext,
        index: &LaastIndex,
    ) -> Option<()> {
        // Verify
        verify_match!(
            &*self.ident.name,
            &pattern.compiled_pattern,
            ctx,
            pattern.essential
        );
        if pattern.auxiliary_pattern.is_some() {
            if let Some(var_type) = &self.var_type {
                verify_match!(
                    &*var_type,
                    &pattern.compiled_auxiliary_pattern,
                    ctx,
                    pattern.essential
                );
            } else {
                quit!(pattern.essential);
            }
        }

        // Verify subpatterns
        explore_all_subpatterns!(pattern.subpatterns, ctx, index, self.annotation);

        // Write and return
        write_to_context(
            &self.ident.name,
            pattern.essential,
            &mut pattern.compiled_pattern,
            ctx,
        )?;
        if let Some(var_type) = &self.var_type {
            write_to_context(
                var_type,
                pattern.essential,
                &mut pattern.compiled_auxiliary_pattern,
                ctx,
            )
        } else {
            Some(())
        }
    }
}

impl NodePatternParser for CallExpr {
    fn parse(
        &self,
        pattern: &mut NodePattern,
        ctx: &mut ParserContext,
        index: &LaastIndex,
    ) -> Option<()> {
        // Extract the function name, and the lefthand side's name (if needed)
        let mut lhs = None;
        let (raw_name, auxiliary_name) = match *self.name {
            Expr::DotExpr(DotExpr {
                ref selected,
                ref expr,
                ..
            }) => {
                // Find the lefthand side's name
                let aux_name = if pattern.auxiliary_pattern.is_some() {
                    match expr.as_ref() {
                        Expr::Ident(Ident { ref name, .. }) => Some(name),
                        Expr::Literal(Literal { ref value, .. }) => Some(value),
                        ref _unknown => {
                            // tracing::warn!(
                            //     "Currently unhandled CallExpression auxiliary match {:?}",
                            //     unknown
                            // );
                            return None;
                        }
                    }
                } else {
                    // If no auxiliary pattern is provided, it is assumed we must visit the lefthand side
                    lhs = Some(expr);
                    None
                };
                match selected.as_ref() {
                    Expr::Ident(Ident { ref name, .. }) => (name, aux_name),
                    Expr::Literal(Literal { ref value, .. }) => (value, aux_name),
                    ref _unknown => {
                        // tracing::warn!("Currently unhandled CallExpression name {:?}", unknown);
                        return None;
                    }
                }
            }
            Expr::Ident(Ident { ref name, .. }) => (name, None),
            Expr::Literal(Literal { ref value, .. }) => (value, None),
            ref _unknown => {
                // tracing::warn!("Currently unhandled CallExpression name {:?}", unknown);
                return None;
            }
        };

        // Verify matches on the function's name and its lefthand side, if the latter was found
        write_to_context(
            raw_name,
            pattern.essential,
            &mut pattern.compiled_pattern,
            ctx,
        )?;
        if let Some(lhs) = auxiliary_name {
            write_to_context(
                lhs,
                pattern.essential,
                &mut pattern.compiled_auxiliary_pattern,
                ctx,
            )?;
        } else if pattern.auxiliary_pattern.is_some() {
            quit!(pattern.essential);
        }

        // Check lefthand side, if it exists
        if let Some(lhs) = lhs.as_mut() {
            for pattern in pattern.subpatterns.iter_mut() {
                lhs.explore(pattern, ctx, index)?;
            }
        }

        // Match method parameters
        let mut params = pattern
            .subpatterns
            .iter_mut()
            // .filter(|child| match child.identifier {
            //     NodeType::CallExpr | NodeType::VarDecl | NodeType::Ident | NodeType::Literal => {
            //         true
            //     }
            //     _ => false,
            // })
            .collect::<Vec<&mut NodePattern>>();
        match_subsequence(&mut params, &self.args, ctx, index)
    }
}

impl NodePatternParser for AnnotationComponent {
    fn parse(
        &self,
        pattern: &mut NodePattern,
        ctx: &mut ParserContext,
        index: &LaastIndex,
    ) -> Option<()> {
        // Verify
        verify_match!(
            &*self.name,
            &pattern.compiled_pattern,
            ctx,
            pattern.essential
        );
        verify_match!(
            &*self.value,
            &pattern.compiled_auxiliary_pattern,
            ctx,
            pattern.essential
        );

        // Verify subpatterns
        explore_all_subpatterns!(pattern.subpatterns, ctx, index, &self.key_value_pairs);

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
            &mut pattern.compiled_auxiliary_pattern,
            ctx,
        )
    }
}

impl NodePatternParser for AnnotationValuePair {
    fn parse(
        &self,
        pattern: &mut NodePattern,
        ctx: &mut ParserContext,
        _index: &LaastIndex,
    ) -> Option<()> {
        verify_match!(&self.key, &pattern.compiled_pattern, ctx, pattern.essential);
        verify_match!(
            &self.value,
            &pattern.compiled_auxiliary_pattern,
            ctx,
            pattern.essential
        );
        write_to_context(
            &self.value,
            pattern.essential,
            &mut pattern.compiled_auxiliary_pattern,
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
    fn parse(
        &self,
        pattern: &mut NodePattern,
        ctx: &mut ParserContext,
        _index: &LaastIndex,
    ) -> Option<()> {
        write_to_context(
            &self.name,
            pattern.essential,
            &mut pattern.compiled_pattern,
            ctx,
        )
    }
}

impl NodePatternParser for Literal {
    fn parse(
        &self,
        pattern: &mut NodePattern,
        ctx: &mut ParserContext,
        _index: &LaastIndex,
    ) -> Option<()> {
        verify_match!(
            &self.value,
            &pattern.compiled_pattern,
            ctx,
            pattern.essential
        );
        write_to_context(
            &self.value,
            pattern.essential,
            &mut pattern.compiled_pattern,
            ctx,
        )
    }
}
