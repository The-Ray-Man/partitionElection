use std::collections::BTreeSet;

use super::ballot_trait;
use crate::structures::Partition;
use crate::structures::Structure;
use crate::structures::{Coalition, Ranking};

#[derive(Eq, Hash, Ord, PartialEq, PartialOrd, Clone)]
pub struct MyBallot {
    pub coalition: Coalition,
    ranking: Ranking,
}

impl MyBallot {
    /// Creates a new MyBallot ballot.
    pub fn new(coalition: &Coalition, m: usize) -> Self {
        todo!()
    }
}

impl ballot_trait::Ballot for MyBallot {
    fn get_ranking(&self) -> Ranking {
        self.ranking.clone()
    }

    fn get_name() -> String {
        "MyBallot".to_string()
    }

    fn get_full_name() -> String {
        "My Ballot".to_string()
    }

    fn generate_all_rankings(m: usize) -> BTreeSet<Ranking>
    where
        Self: Sized,
    {
        todo!()
    }
}
