use std::collections::BTreeMap;

use z3::ast::{Bool, Int};

use crate::proof::{profile::Profile, rule::VotingRule};

use super::{Axiom, AxiomType};

pub struct Participation {}

impl Axiom for Participation {
    fn condition<'a: 'b, 'b>(
        profile: &'b Profile<'a>,
        rule: &'b Box<&'b (dyn VotingRule<'a> + 'b)>,
    ) -> Vec<Bool<'a>> {
        Participation::condition_generator(profile, rule).collect::<Vec<_>>()
    }

    fn get_type() -> AxiomType {
        AxiomType::Forall
    }

    fn condition_generator<'a: 'b, 'b>(
        profile: &'b Profile<'a>,
        rule: &'b Box<&'b (dyn VotingRule<'a> + 'b)>,
    ) -> impl Iterator<Item = Bool<'a>> + 'b {
        let ctx = profile.get_ctx();
        // Iterate over all possible rankings (this is the ranking from the new voter)
        profile.votes.iter().flat_map(move |(ranking, _)| {
            log::info!("Checking for voter with ranking {}", ranking.to_string());
            // Create a new profile with the new ranking
            let one = Int::from_i64(ctx, 1);
            let extra_votes = BTreeMap::from([(ranking.clone(), one)]);
            // Iterate over all partitions. In each iteration we assume that partition has won the election with the old profile.
            profile.partitions.iter().filter_map(move |partition| {
                log::info!("Winner without voter {}", partition.to_string());
                // Winner of the old profile
                let winner_condition = rule.only_winner(partition, profile, None);

                // Check which partitions are strictly less preferred by the new voter.
                let less_preferred = profile
                    .partitions
                    .iter()
                    .filter(|other| ranking.is_strictly_preferred(partition, other))
                    .collect::<Vec<_>>();

                // If no partition is strictly less preferred, then the axiom holds trivially and we can skip this case.
                if less_preferred.is_empty() {
                    log::info!("None partition are less preferred");
                    None
                } else {
                    log::info!(
                        "Less preferred {}",
                        less_preferred
                            .iter()
                            .map(|x| { x.to_string() })
                            .collect::<Vec<_>>()
                            .join(", ")
                    );
                    // We want that none of the less preferred partitions win the election with the new profile.
                    let more_preferred_does_not_win = less_preferred
                        .iter()
                        .map(|p| rule.only_winner(p, profile, Some(&extra_votes)))
                        .collect::<Vec<_>>();
                    let more_preferred_does_not_win =
                        Bool::or(ctx, &more_preferred_does_not_win.iter().collect::<Vec<_>>())
                            .not();
                    let condition = winner_condition.implies(&more_preferred_does_not_win);
                    Some(condition)
                }
            })
        })
    }
    fn short_name() -> &'static str {
        "part"
    }

    fn full_name() -> &'static str {
        "participation"
    }
}
