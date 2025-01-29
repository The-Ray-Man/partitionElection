use z3::ast::Bool;

use crate::proof::{profile::Profile, rule::VotingRule};
use crate::utils::structures::powerset_generator;

use super::{Axiom, AxiomType};

pub struct Unanimity {}

impl Axiom for Unanimity {
    fn condition_generator<'a: 'b, 'b>(
        profile: &'b Profile<'a>,
        rule: &'b Box<&'b (dyn VotingRule<'a> + 'b)>,
    ) -> impl Iterator<Item = Bool<'a>> + 'b {
        let ctx = profile.get_ctx();
        // Whenever a set is ranked first by all voters, then this set must win.
        powerset_generator(&profile.partitions).filter_map(move |winning_set| {
            if winning_set.is_empty() {
                return None;
            }
            log::info!(
                "Checking set {:?}",
                winning_set
                    .iter()
                    .map(|x| { x.to_string() })
                    .collect::<Vec<_>>()
                    .join(", ")
            );
            // All voters must rank the winning_set as their first indifference class.
            let preconditions = profile
                .votes
                .iter()
                .filter_map(|(ranking, _)| {
                    if !ranking.is_exacty_equiv_class(0, &winning_set) {
                        let zero_votes = rule.zero_votes(ranking, profile, None);
                        Some(zero_votes)
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();
            let preconditions = preconditions.iter().collect::<Vec<_>>();
            let precondition = Bool::and(ctx, &preconditions);

            // This winning_set must be the exact winner.
            let winning_condition = rule.exact_winner_set(&winning_set, profile, None);

            // Whenever the preconditions are met, then the winning_condition must hold.
            let formula = precondition.implies(&winning_condition);
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
        Unanimity::condition_generator(profile, rule).collect()
    }

    fn short_name() -> &'static str {
        "unam"
    }

    fn full_name() -> &'static str {
        "unanimity"
    }
}
