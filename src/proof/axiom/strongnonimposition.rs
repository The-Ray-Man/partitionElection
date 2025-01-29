use z3::ast::Bool;

use crate::proof::{profile::Profile, rule::VotingRule};
use crate::utils::structures::powerset_generator;

use super::{Axiom, AxiomType};

pub struct Strongnonimposition {}

impl Axiom for Strongnonimposition {
    fn condition_generator<'a: 'b, 'b>(
        profile: &'b Profile<'a>,
        rule: &'b Box<&'b (dyn VotingRule<'a> + 'b)>,
    ) -> impl Iterator<Item = Bool<'a>> + 'b {
        // Every non-empty set must be able to win.
        powerset_generator(&profile.partitions).filter_map(move |winner_set| {
            if winner_set.is_empty() {
                None
            } else {
                log::info!(
                    "Checking for winning set: {}",
                    winner_set
                        .iter()
                        .map(|x| { x.to_string() })
                        .collect::<Vec<_>>()
                        .join(", ")
                );
                Some(rule.exact_winner_set(&winner_set, profile, None))
            }
        })
    }

    fn get_type() -> AxiomType {
        AxiomType::Exists
    }

    fn condition<'a: 'b, 'b>(
        profile: &'b Profile<'a>,
        rule: &'b Box<&'b (dyn VotingRule<'a> + 'b)>,
    ) -> Vec<Bool<'a>> {
        Strongnonimposition::condition_generator(profile, rule).collect()
    }
    fn short_name() -> &'static str {
        "snoim"
    }

    fn full_name() -> &'static str {
        "strong-nonimposition"
    }
}
