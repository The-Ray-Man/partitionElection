use crate::structures::Ranking;
use crate::structures::Structure;
use crate::{proof::profile::Profile, structures::Partition};
use std::collections::BTreeMap;
use std::{
    collections::BTreeSet,
    ops::{Add, Mul},
};
use z3::ast::Int;
use z3::ast::Real;

use super::rule_trait::VotingRule;

pub struct Borda {
    pub partitions: BTreeSet<Partition>,
    pub alternatives: usize,
    pub num_candidates: usize,
}

impl<'a> VotingRule<'a> for Borda {
    fn new(m: usize) -> Self {
        let partitions = Partition::all(m);
        let alternatives = partitions.len();
        let num_candidates = m;

        Borda {
            partitions,
            alternatives,
            num_candidates,
        }
    }

    fn score(
        &self,
        partition: &Partition,
        profile: &Profile<'a>,
        extra_vote: Option<&BTreeMap<Ranking, Int<'a>>>,
    ) -> Real<'a> {
        let ctx = profile.get_ctx();
        let sub_scores = profile
            .votes
            .iter()
            .map(|(ranking, count)| {
                let class_index = ranking.index(partition);

                let num_alternatives_before = ranking
                    .order_type()
                    .into_iter()
                    .take(class_index)
                    .sum::<usize>();

                let score = self.alternatives - 1 - num_alternatives_before;

                // let special_score = profile.extra_votes.get(ranking);

                match extra_vote {
                    Some(extra_vote) => {
                        let extra_score = extra_vote.get(ranking);
                        match extra_score {
                            Some(s) => {
                                let new_count = count.add(s);
                                new_count.mul(Int::from_i64(ctx, score as i64))
                            }
                            None => count.mul(Int::from_i64(ctx, score as i64)),
                        }
                    }
                    None => count.mul(Int::from_i64(ctx, score as i64)),
                }
            })
            .collect::<Vec<_>>();

        let sub_scores_refs: Vec<&Int> = sub_scores.iter().collect();
        let total_score = Int::add(ctx, &sub_scores_refs);

        total_score.to_real()
    }

    fn all_partitions(&self) -> BTreeSet<Partition> {
        self.partitions.clone()
    }

    fn name() -> &'static str
    where
        Self: Sized,
    {
        "borda"
    }
}
