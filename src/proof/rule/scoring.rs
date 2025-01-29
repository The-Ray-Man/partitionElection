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
use z3::Context;

use super::rule_trait::VotingRule;
use itertools::Itertools;
pub struct Scoring<'b> {
    pub partitions: BTreeSet<Partition>,
    pub alternatives: usize,
    pub num_candidates: usize,
    scores: BTreeMap<Vec<usize>, Vec<Real<'b>>>,
}

impl<'a> Scoring<'a> {
    pub fn create(m: usize, types: BTreeSet<Vec<usize>>, ctx: &'a Context) -> Self {
        let partitions = Partition::all(m);
        let num_partitions = partitions.len();
        let mut scores = BTreeMap::new();
        for t in types {
            let mut score = Vec::new();
            for i in 0..t.len() {
                score.push(Real::fresh_const(
                    ctx,
                    &format!(
                        "score_{}_{}",
                        t.iter().map(|x| { x.to_string() }).join("-"),
                        i
                    ),
                ));
            }
            scores.insert(t, score);
        }

        Scoring {
            partitions,
            alternatives: num_partitions,
            num_candidates: m,
            scores,
        }
    }

    pub fn get_score_vector(&self, order_type: &Vec<usize>) -> &Vec<Real<'a>> {
        self.scores.get(order_type).unwrap()
    }
}

impl<'a> VotingRule<'a> for Scoring<'a> {
    fn new(m: usize) -> Self
    where
        Self: Sized,
    {
        todo!()
    }

    fn name() -> &'static str
    where
        Self: Sized,
    {
        "scoring"
    }

    fn all_partitions(&self) -> BTreeSet<Partition> {
        self.partitions.clone()
    }

    fn score(
        &self,
        partition: &Partition,
        profile: &Profile<'a>,
        extra_votes: Option<&BTreeMap<Ranking, Int<'a>>>,
    ) -> Real<'a> {
        let ctx = profile.get_ctx();

        let mut score = profile
            .votes
            .iter()
            .map(|(ranking, var)| {
                let order_type = ranking.order_type();
                let score_vector = self.scores.get(&order_type).unwrap();
                let index = ranking.index(partition);
                let score = &score_vector[index];
                score.mul(var.to_real())
            })
            .collect::<Vec<_>>();

        let score_extra = if let Some(extra_votes) = extra_votes {
            extra_votes
                .iter()
                .map(|(ranking, var)| {
                    let order_type = ranking.order_type();
                    let score_vector = self.scores.get(&order_type).unwrap();
                    let index = ranking.index(partition);
                    let score = &score_vector[index];
                    score.mul(var.to_real())
                })
                .collect::<Vec<_>>()
        } else {
            vec![]
        };

        score.extend(score_extra);

        let score = Real::add(ctx, &score.iter().collect::<Vec<_>>());

        score
    }
}
