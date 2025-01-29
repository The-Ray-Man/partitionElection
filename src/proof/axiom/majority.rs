use std::collections::BTreeMap;

use z3::ast::{Bool, Int};

use crate::structures::Partition;

use crate::proof::{profile::Profile, rule::VotingRule};

use super::{Axiom, AxiomType};

pub struct Majority {}

impl Axiom for Majority {
    fn condition_generator<'a: 'b, 'b>(
        profile: &'b Profile<'a>,
        rule: &'b Box<&'b (dyn VotingRule<'a> + 'b)>,
    ) -> impl Iterator<Item = Bool<'a>> + 'b {
        let ctx = profile.get_ctx();

        // To have a strict majority, one needs more than halv_votes.
        let all_vars = profile.votes.values().collect::<Vec<_>>();
        let sum = Int::add(ctx, &all_vars);
        let half_votes = sum.div(&Int::from_i64(ctx, 2));

        // For every partition we calculate the sum of the first priority votes.
        let first_prio_sums = profile
            .partitions
            .iter()
            .map(|partition| {
                let vars_first_prio = profile
                    .votes
                    .iter()
                    .filter_map(|(ranking, var)| {
                        if ranking.index(partition) == 0 {
                            Some(var)
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>();

                let sum = if vars_first_prio.is_empty() {
                    Int::from_i64(ctx, 0)
                } else {
                    Int::add(ctx, &vars_first_prio)
                };
                (partition, sum)
            })
            .collect::<BTreeMap<&Partition, Int>>();

        profile.partitions.iter().map(move |partition| {
            log::info!("Checking for majority partition {}", partition.to_string());
            // Precondition
            // partition must be a strict majority partition
            // Every other partition must not be a strict majority partition.
            let mut precondition = Vec::new();
            let is_majority = first_prio_sums.get(partition).unwrap().gt(&half_votes); // Save, first_prio_sums contains all partitions
            precondition.push(&is_majority);
            let only_majority_conditions = profile
                .partitions
                .iter()
                .filter_map(|other| {
                    if other == partition {
                        None
                    } else {
                        let score = first_prio_sums.get(other).unwrap(); // Save, first_prio_sums contains all partitions
                        let condition = score.le(&half_votes);
                        Some(condition)
                    }
                })
                .collect::<Vec<_>>();
            precondition.extend(only_majority_conditions.iter());
            let precondition = Bool::and(ctx, &precondition);

            // The partition should be winning.
            let winner = rule.winner(partition, profile, None);

            // Preconditions implies winner
            let formula = precondition.implies(&winner);
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
        Majority::condition_generator(profile, rule).collect()
    }

    fn short_name() -> &'static str {
        "maj"
    }

    fn full_name() -> &'static str {
        "majority"
    }
}
