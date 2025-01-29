use itertools::Itertools;
use z3::ast::Bool;

use crate::proof::{profile::Profile, rule::VotingRule};

use super::{Axiom, AxiomType};

pub struct WeakPairSupport {}

impl Axiom for WeakPairSupport {
    fn condition_generator<'a: 'b, 'b>(
        profile: &'b Profile<'a>,
        rule: &'b Box<&'b (dyn VotingRule<'a> + 'b)>,
    ) -> impl Iterator<Item = Bool<'a>> + 'b {
        let ctx = profile.get_ctx();
        // Whenever a set is ranked first by all voters, then this set must win.
        profile
            .candidates
            .iter()
            .cartesian_product(profile.candidates.iter())
            .filter_map(move |pair| {
                if pair.0 == pair.1 {
                    return None;
                }
                log::info!(
                    "Checking for pair {}, {}",
                    pair.0.to_string(),
                    pair.1.to_string()
                );

                // All voters who do not contain pair in a first priority partition must not exist.
                let preconditions = profile
                    .votes
                    .iter()
                    .filter_map(|(ranking, _)| {
                        if !ranking.contains_pair_in_indiff_class(0, &pair) {
                            let zero_votes = rule.zero_votes(ranking, profile, None);
                            Some(zero_votes)
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>();

                let precondition = if preconditions.is_empty() {
                    Bool::from_bool(ctx, true)
                } else {
                    let preconditions = preconditions.iter().collect::<Vec<_>>();
                    Bool::and(ctx, &preconditions)
                };

                let partition_with_pair = profile
                    .partitions
                    .iter()
                    .filter(|p| p.contains_pair(&pair))
                    .collect::<Vec<_>>();

                let winning_conditions = partition_with_pair
                    .iter()
                    .map(|p| rule.winner(p, profile, None))
                    .collect::<Vec<_>>();

                let winning_condition =
                    Bool::or(ctx, &winning_conditions.iter().collect::<Vec<_>>());

                // Whenever the preconditions are met, then the winning_condition must hold.
                let formula: Bool<'a> = precondition.implies(&winning_condition);
                Some(formula)
            })
    }

    fn get_type() -> AxiomType {
        AxiomType::Forall
    }

    fn condition<'a: 'b, 'b>(
        profile: &'b Profile<'a>,
        rule: &'b Box<&'b (dyn VotingRule<'a> + 'b)>,
    ) -> Vec<Bool<'a>> {
        WeakPairSupport::condition_generator(profile, rule).collect()
    }

    fn short_name() -> &'static str {
        "wps"
    }

    fn full_name() -> &'static str {
        "weak-pair-support"
    }
}
