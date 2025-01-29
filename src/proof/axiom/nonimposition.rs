use z3::ast::Bool;

use crate::proof::{profile::Profile, rule::VotingRule};

use super::{Axiom, AxiomType};

pub struct Nonimposition {}

impl Axiom for Nonimposition {
    fn condition_generator<'a: 'b, 'b>(
        profile: &'b Profile<'a>,
        rule: &'b Box<&'b (dyn VotingRule<'a> + 'b)>,
    ) -> impl Iterator<Item = Bool<'a>> + 'b {
        // Every partition should be able to win.
        profile.partitions.iter().map(move |partition| {
            log::info!("Checking for winner: {}", partition.to_string());
            rule.only_winner(partition, profile, None)
        })
    }

    fn get_type() -> AxiomType {
        AxiomType::Exists
    }

    fn condition<'a: 'b, 'b>(
        profile: &'b Profile<'a>,
        rule: &'b Box<&'b (dyn VotingRule<'a> + 'b)>,
    ) -> Vec<Bool<'a>> {
        Nonimposition::condition_generator(profile, rule).collect()
    }

    fn short_name() -> &'static str {
        "noim"
    }

    fn full_name() -> &'static str {
        "noimposition"
    }
}
