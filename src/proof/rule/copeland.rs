use super::rule_trait::VotingRule;
use crate::structures::Ranking;
use crate::structures::Structure;
use crate::{proof::profile::Profile, structures::Partition};
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::ops::Sub;
use z3::ast::Int;
use z3::ast::Real;

pub struct Copeland {
    pub partitions: BTreeSet<Partition>,
    pub alternatives: usize,
    pub num_candidates: usize,
}

impl<'a> VotingRule<'a> for Copeland
where
    Self: Sized,
{
    fn new(m: usize) -> Self {
        let partitions = Partition::all(m);
        let alternatives = partitions.len();
        let num_candidates = m;

        Copeland {
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

        let scores = profile
            .partitions
            .iter()
            .map(|other| {
                let mut pro_variables = Vec::new();
                let mut con_variables = Vec::new();
                profile.votes.iter().for_each(|(ranking, var)| {
                    if ranking.is_strictly_preferred(partition, other) {
                        pro_variables.push(var);

                        if let Some(extra_votes) = extra_vote {
                            if let Some(extra_vote) = extra_votes.get(ranking) {
                                pro_variables.push(extra_vote);
                            }
                        }
                    } else if ranking.is_strictly_preferred(other, partition) {
                        con_variables.push(var);
                        if let Some(extra_votes) = extra_vote {
                            if let Some(extra_vote) = extra_votes.get(ranking) {
                                con_variables.push(extra_vote);
                            }
                        }
                    }
                });

                let pro_sum = if pro_variables.is_empty() {
                    Int::from_i64(ctx, 0)
                } else {
                    Int::add(ctx, &pro_variables)
                };

                let con_sum = if con_variables.is_empty() {
                    Int::from_i64(ctx, 0)
                } else {
                    Int::add(ctx, &con_variables)
                };

                let score = pro_sum.sub(&con_sum);
                let maj_win = score.gt(&Int::from_i64(ctx, 0));
                maj_win.ite(&Int::from_i64(ctx, 1), &Int::from_i64(ctx, 0))
            })
            .collect::<Vec<_>>();
        let scores = scores.iter().collect::<Vec<_>>();
        Int::add(ctx, &scores).to_real()
    }

    fn all_partitions(&self) -> BTreeSet<Partition> {
        self.partitions.clone()
    }

    fn name() -> &'static str
    where
        Self: Sized,
    {
        "copeland"
    }
}
