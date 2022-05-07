use std::cell::Ref;

use super::{CompiledPattern, ExplorerContext, NodePattern, ParserContext, RessaNodeExplorer};
use super::{LaastIndex, NodeType};
use crate::ast::*;
use crate::prophet::*;
use itertools::Itertools;

/// Defines how to parse an individual node that has been confirmed to be of interest
/// Returns: None if match inconclusive (regexes fail), Some(true) if matched, Some(false) if failed
pub trait NodePatternParser {
    fn parse(
        &self,
        pattern: &NodePattern,
        ctx: &mut ExplorerContext,
        index: &LaastIndex,
    ) -> Option<()>;
}

fn write_to_context(
    to_match: &str,
    pattern: Ref<Option<CompiledPattern>>,
    ctx: &mut ParserContext,
) -> Option<()> {
    match pattern.as_ref() {
        Some(compiled_pattern) if compiled_pattern.match_and_insert(to_match, ctx) => Some(()),
        _ => None,
    }
}

fn match_subsequence<T: RessaNodeExplorer>(
    params: &[&NodePattern],
    explorable: &[T],
    ctx: &mut ExplorerContext,
    index: &LaastIndex,
) -> Option<()> {
    let (mut start, mut end) = (0_usize, params.len());
    let mut matched = !params.iter().any(|x| x.essential) && explorable.is_empty();

    while start < explorable.len() {
        // Pre
        let mut found_match = true;
        let mut pattern_iter = params.iter();

        // Perform subsequence matching
        for i in start..end {
            let pattern = pattern_iter.next()?;
            if explorable.get(i)?.explore(pattern, ctx, index).is_none() {
                found_match = false;
                break;
            }
        }

        // Post
        if found_match {
            matched = true;
        }
        start += 1;
        if end < explorable.len() {
            end += 1;
        }
    }

    // Determine if we matched the pattern at some point
    make_result(matched)
}

fn make_result(found: bool) -> Option<()> {
    if found {
        Some(())
    } else {
        None
    }
}

// /// Verify if an Option<Regex> matches a specific string; if it fails, exits
// macro_rules! verify_match {
//     ( $match_str:expr, $pattern:expr, $ctx:expr, $essential:expr ) => {
//         if let Some(compiled) = $pattern {
//             if !compiled.matches($match_str, $ctx) {
//                 quit!($essential);
//             }
//         }
//     };
// }

/// Explore all elements in the provided collections
#[macro_export]
macro_rules! explore_all {
    ( $pattern:expr, $ctx:expr, $index:expr, $( $explorable:expr ),+ ) => {{
        let mut explore_all_found_essential = !$pattern.essential;
         $(
            for x in $explorable.iter() {
                if x.explore($pattern, $ctx, $index).is_some() {
                    explore_all_found_essential = true;
                 }
             }
         )*
        make_result(explore_all_found_essential)
    }};
}

/// Explores all subpatterns, verifying each essential pattern matches
#[macro_export]
macro_rules! explore_all_subpatterns {
    ( $subpatterns:expr, $ctx:expr, $index:expr, $( $explorable:expr ),+ ) => {
        for subpattern in $subpatterns.iter() {
            let mut explore_all_found_essential = !subpattern.essential;
            $(
                if explore_all!(subpattern, $ctx, $index, $explorable).is_some() {
                    explore_all_found_essential = true;
                }
            )*
            make_result(explore_all_found_essential)?;
        }
    };
}

impl NodePatternParser for ClassOrInterfaceComponent {
    fn parse(
        &self,
        pattern: &NodePattern,
        ctx: &mut ExplorerContext,
        index: &LaastIndex,
    ) -> Option<()> {
        // If all subpatterns matched, extract context
        pattern.match_regexes(
            &self.component.container_name,
            &self.component.component.package_name,
            &mut ctx.parser,
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
        pattern: &NodePattern,
        ctx: &mut ExplorerContext,
        index: &LaastIndex,
    ) -> Option<()> {
        pattern.match_regexes(
            &self.component.instance_name,
            &self.return_type,
            &mut ctx.parser,
        )?;

        // Match method parameters
        let params = pattern
            .subpatterns
            .iter()
            .filter(|child| matches!(child.identifier, crate::ressa::NodeType::MethodParam))
            .collect::<Vec<&NodePattern>>();
        match_subsequence(&params, &self.parameters, ctx, index)?;

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
            .iter()
            .filter(|child| !matches!(child.identifier, crate::ressa::NodeType::MethodParam))
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
        pattern: &NodePattern,
        ctx: &mut ExplorerContext,
        index: &LaastIndex,
    ) -> Option<()> {
        // Verify
        pattern.match_regexes(&self.parameter_name, &self.r#type, &mut ctx.parser)?;

        // If there are annotation subnodes, check if they match the subpatterns--then exit
        // Otherwise, exit based on whether there are any essential subnodes
        if let Some(annotations) = &self.annotation {
            explore_all_subpatterns!(pattern.subpatterns, ctx, index, annotations);
            Some(())
        } else {
            None
        }
    }
}

impl NodePatternParser for FieldComponent {
    fn parse(
        &self,
        pattern: &NodePattern,
        ctx: &mut ExplorerContext,
        index: &LaastIndex,
    ) -> Option<()> {
        // Verify
        pattern.match_regexes(&*self.field_name, &self.r#type, &mut ctx.parser)?;
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
        pattern: &NodePattern,
        ctx: &mut ExplorerContext,
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
            if self.variables.len() != self.expressions.len() || self.variables.is_empty() {
                return None;
            }

            // Analyze pattern
            let len_both = decl_patterns.len();
            for pattern_index in 0..len_both {
                for i in 0..self.variables.len() {
                    // Parse variable declarations
                    self.variables.get(i)?.explore(
                        pattern
                            .subpatterns
                            .get(*decl_patterns.get(pattern_index)?)?,
                        ctx,
                        index,
                    )?;

                    // Parse equality
                    let non_decl_pattern =
                        pattern.subpatterns.get(*non_decl.get(pattern_index)?)?;
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
            let pattern = pattern.subpatterns.get(*decl_patterns.first()?)?;
            explore_all!(pattern, ctx, index, self.variables)?;
        } else {
            // Case 3: multiple non-Decls to fewer Decls
            let real_expressions = self.expressions.iter().flatten().collect::<Vec<&Expr>>();
            match_subsequence(
                // Get the actual pattern references from the subpatterns array using the indices in decl_patterns (I'm sorry.)
                &pattern
                    .subpatterns
                    .iter()
                    .enumerate()
                    .filter(|(ndx, _)| decl_patterns.contains(ndx))
                    .map(|(_, p)| p)
                    .collect_vec(),
                &self.variables,
                ctx,
                index,
            )?;
            for ndx in non_decl.iter_mut() {
                let pattern = pattern.subpatterns.get(*ndx)?;
                explore_all!(pattern, ctx, index, real_expressions)?;
            }
        }
        Some(())
    }
}

impl NodePatternParser for VarDecl {
    fn parse(
        &self,
        pattern: &NodePattern,
        ctx: &mut ExplorerContext,
        index: &LaastIndex,
    ) -> Option<()> {
        // Verify the main pattern matches (in its own scope to allow ctx.parser to be borrowed mutably later)

        // Verify the auxiliary pattern matches (and inject variables)
        if pattern.auxiliary_pattern.is_some() && self.var_type.is_some() {
            pattern.match_regexes(
                &*self.ident.name,
                &*self.var_type.as_ref().unwrap(),
                &mut ctx.parser,
            )?;
        } else if let Some(result) = pattern.compiled_pattern.borrow().as_ref() {
            result.match_callback(&*self.ident.name, &mut ctx.parser)?();
        } else {
            return None;
        }

        // Verify subpatterns
        explore_all_subpatterns!(pattern.subpatterns, ctx, index, self.annotation);
        Some(())
    }
}

impl NodePatternParser for CallExpr {
    fn parse(
        &self,
        pattern: &NodePattern,
        ctx: &mut ExplorerContext,
        index: &LaastIndex,
    ) -> Option<()> {
        // Extract the function name, and the lefthand side's name (if needed)
        let mut lhs = None;
        let (raw_name, auxiliary_name) = match &*self.name {
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
                        _unknown => {
                            // tracing::warn!(
                            //     "Currently unhandled CallExpression auxiliary match {:?}",
                            //     unknown
                            // );
                            return None;
                        }
                    }
                } else {
                    // If no auxiliary pattern is provided, it is assumed we must visit the lefthand side
                    lhs = Some(&*self.name);
                    None
                };
                match selected.as_ref() {
                    Expr::Ident(Ident { ref name, .. }) => (name, aux_name),
                    Expr::Literal(Literal { ref value, .. }) => (value, aux_name),
                    _unknown => {
                        // tracing::warn!("Currently unhandled CallExpression name {:?}", unknown);
                        return None;
                    }
                }
            }
            Expr::Ident(Ident { ref name, .. }) => (name, None),
            Expr::Literal(Literal { ref value, .. }) => (value, None),
            _unknown => {
                // tracing::warn!("Currently unhandled CallExpression name {:?}", unknown);
                return None;
            }
        };

        // Verify matches on the function's name and its lefthand side, if the latter was found
        write_to_context(raw_name, pattern.compiled_pattern.borrow(), &mut ctx.parser)?;
        if let Some(lhs) = auxiliary_name {
            write_to_context(
                lhs,
                pattern.compiled_auxiliary_pattern.borrow(),
                &mut ctx.parser,
            )?;
        } else if pattern.auxiliary_pattern.is_some() {
            return None;
        }

        // Check lefthand side, if it exists
        if let Some(lhs) = lhs.as_mut() {
            for pattern in pattern.subpatterns.iter() {
                lhs.explore(pattern, ctx, index)?;
            }
        }

        // Match method parameters
        let params = pattern
            .subpatterns
            .iter()
            // .filter(|child| match child.identifier {
            //     NodeType::CallExpr | NodeType::VarDecl | NodeType::Ident | NodeType::Literal => {
            //         true
            //     }
            //     _ => false,
            // })
            .collect::<Vec<&NodePattern>>();
        match_subsequence(&params, &self.args, ctx, index)
    }
}

impl NodePatternParser for AnnotationComponent {
    fn parse(
        &self,
        pattern: &NodePattern,
        ctx: &mut ExplorerContext,
        index: &LaastIndex,
    ) -> Option<()> {
        // Verify
        pattern.match_regexes(&*self.name, &*self.value, &mut ctx.parser)?;

        // Verify subpatterns
        explore_all_subpatterns!(pattern.subpatterns, ctx, index, &self.key_value_pairs);
        Some(())
    }
}

impl NodePatternParser for AnnotationValuePair {
    fn parse(
        &self,
        pattern: &NodePattern,
        ctx: &mut ExplorerContext,
        _index: &LaastIndex,
    ) -> Option<()> {
        pattern.match_regexes(&self.key, &self.value, &mut ctx.parser)
    }
}

impl NodePatternParser for Ident {
    fn parse(
        &self,
        pattern: &NodePattern,
        ctx: &mut ExplorerContext,
        _index: &LaastIndex,
    ) -> Option<()> {
        write_to_context(
            &self.name,
            pattern.compiled_pattern.borrow(),
            &mut ctx.parser,
        )
    }
}

impl NodePatternParser for Literal {
    fn parse(
        &self,
        pattern: &NodePattern,
        ctx: &mut ExplorerContext,
        _index: &LaastIndex,
    ) -> Option<()> {
        write_to_context(
            &self.value,
            pattern.compiled_pattern.borrow(),
            &mut ctx.parser,
        )
    }
}

impl NodePatternParser for BinaryExpr {
    fn parse(
        &self,
        pattern: &NodePattern,
        ctx: &mut ExplorerContext,
        index: &LaastIndex,
    ) -> Option<()> {
        let op: String = self.op.clone().into();
        write_to_context(&op, pattern.compiled_pattern.borrow(), &mut ctx.parser)?;
        self.lhs.explore(&pattern.subpatterns[0], ctx, index)?;
        self.rhs.explore(&pattern.subpatterns[1], ctx, index)?;
        Some(())
    }
}

impl NodePatternParser for AssignExpr {
    fn parse(
        &self,
        pattern: &NodePattern,
        ctx: &mut ExplorerContext,
        index: &LaastIndex,
    ) -> Option<()> {
        explore_all_subpatterns!(pattern.subpatterns, ctx, index, self.lhs, self.rhs);
        Some(())
    }
}
