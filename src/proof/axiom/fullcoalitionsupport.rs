use crate::proof::{profile::Profile, rule::VotingRule};
use z3::ast::Bool;

use super::{Axiom, AxiomType};

pub struct FullCoalitionSupport {}

impl Axiom for FullCoalitionSupport {
    fn condition_generator<'a: 'b, 'b>(
        profile: &'b Profile<'a>,
        rule: &'b Box<&'b (dyn VotingRule<'a> + 'b)>,
    ) -> impl Iterator<Item = Bool<'a>> + 'b {
        let ctx = profile.get_ctx();
        // Whenever a set is ranked first by all voters, then this set must win.
        profile.coalitions.iter().map(move |coalition| {
            log::info!("Checking for coalition {:?}", coalition.to_string());

            // All voters must rank the winning_set as their first indifference class.
            let preconditions = profile
                .votes
                .iter()
                .filter_map(|(ranking, _)| {
                    if !ranking.contains_coalition_in_indiff_class(0, coalition) {
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

            let partitions_with_coalition = profile
                .partitions
                .iter()
                .filter(|p| p.contains(coalition))
                .collect::<Vec<_>>();

            let winning_conditions = partitions_with_coalition
                .iter()
                .map(|p| rule.winner(p, profile, None))
                .collect::<Vec<_>>();

            let winning_condition = Bool::or(ctx, &winning_conditions.iter().collect::<Vec<_>>());

            // Whenever the preconditions are met, then the winning_condition must hold.
            let formula: Bool<'a> = precondition.implies(&winning_condition);
            formula
        })
    }

    fn get_type() -> AxiomType {
        AxiomType::Forall
    }

    fn condition<'a: 'b, 'b>(
        profile: &'b Profile<'a>,
        rule: &'b Box<&'b (dyn VotingRule<'a> + 'b)>,
    ) -> Vec<Bool<'a>> {
        FullCoalitionSupport::condition_generator(profile, rule).collect()
    }

    fn short_name() -> &'static str {
        "fcs"
    }

    fn full_name() -> &'static str {
        "full-coalition-support"
    }
}
