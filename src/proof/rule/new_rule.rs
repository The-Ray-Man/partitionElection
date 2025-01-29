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

pub struct MyRule {
    pub partitions: BTreeSet<Partition>,
    pub alternatives: usize,
    pub num_candidates: usize,
}

impl<'a> VotingRule<'a> for MyRule {
    fn new(m: usize) -> Self {
        let partitions = Partition::all(m);
        let alternatives = partitions.len();
        let num_candidates = m;

        MyRule {
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
        todo!()
    }

    fn all_partitions(&self) -> BTreeSet<Partition> {
        self.partitions.clone()
    }

    fn name() -> &'static str
    where
        Self: Sized,
    {
        "myrule"
    }
}
