use std::collections::BTreeMap;

use itertools::Itertools;
use z3::ast::{Bool, Int};

use crate::proof::{profile::Profile, rule::VotingRule};

use super::{Axiom, AxiomType};

pub struct Strategyproof {}

impl Axiom for Strategyproof {
    fn condition<'a: 'b, 'b>(
        profile: &'b Profile<'a>,
        rule: &'b Box<&'b (dyn VotingRule<'a> + 'b)>,
    ) -> Vec<Bool<'a>> {
        Strategyproof::condition_generator(profile, rule).collect::<Vec<_>>()
    }

    fn get_type() -> AxiomType {
        AxiomType::Forall
    }

    fn condition_generator<'a: 'b, 'b>(
        profile: &'b Profile<'a>,
        rule: &'b Box<&'b (dyn VotingRule<'a> + 'b)>,
    ) -> impl Iterator<Item = Bool<'a>> + 'b {
        let ctx = profile.get_ctx();
        profile
            .votes
            .keys()
            .cartesian_product(profile.votes.keys())
            .filter(|(a, b)| a != b)
            .flat_map(move |(true_pref, fake_pref)| {
                log::info!("Checking for preferences:");
                log::info!("true: {}", true_pref.to_string());
                log::info!("fake: {}", fake_pref.to_string());

                let extra_votes_true = BTreeMap::from([(true_pref.clone(), Int::from_i64(ctx, 1))]);
                let extra_votes_fake = BTreeMap::from([(fake_pref.clone(), Int::from_i64(ctx, 1))]);

                profile.partitions.iter().filter_map(move |partition| {
                    log::info!("True winner {}", partition.to_string());
                    let winner_true = rule.only_winner(partition, profile, Some(&extra_votes_true));

                    let strictly_more_preferred = profile
                        .partitions
                        .iter()
                        .filter(|other| true_pref.is_strictly_preferred(other, partition))
                        .collect::<Vec<_>>();

                    if strictly_more_preferred.is_empty() {
                        log::info!("No more preferred alternative");
                        None
                    } else {
                        let winning_conditions = strictly_more_preferred
                            .iter()
                            .map(|p| rule.only_winner(p, profile, Some(&extra_votes_fake)).not())
                            .collect::<Vec<_>>();

                        let winning_conditions =
                            Bool::and(ctx, &winning_conditions.iter().collect::<Vec<_>>());

                        let condition = winner_true.implies(&winning_conditions);
                        Some(condition)
                    }
                })
            })
    }
    fn short_name() -> &'static str {
        "strat"
    }

    fn full_name() -> &'static str {
        "strategyproof"
    }
}
