use std::collections::BTreeSet;

use super::ballot_trait;
use crate::structures::Partition;
use crate::structures::Structure;
use crate::structures::{Coalition, Ranking};

#[derive(Eq, Hash, Ord, PartialEq, PartialOrd, Clone)]
pub struct Fcs {
    pub coalition: Coalition,
    ranking: Ranking,
}

impl Fcs {
    /// Creates a new Fcs ballot.
    pub fn new(coalition: &Coalition, m: usize) -> Fcs {
        let partition = Partition::all(m);

        let (good, bad) =
            partition
                .into_iter()
                .fold((BTreeSet::new(), BTreeSet::new()), |(mut a, mut b), x| {
                    if x.contains(coalition) {
                        a.insert(x.clone());
                    } else {
                        b.insert(x.clone());
                    }
                    (a, b)
                });

        let ranking = Ranking {
            ranking: vec![good, bad],
        };

        Fcs {
            coalition: coalition.clone(),
            ranking,
        }
    }
}

impl ballot_trait::Ballot for Fcs {
    fn get_ranking(&self) -> Ranking {
        self.ranking.clone()
    }

    fn get_name() -> String {
        "FCS".to_string()
    }

    fn get_full_name() -> String {
        "Favorite Coalition Strict".to_string()
    }

    fn generate_all_rankings(m: usize) -> BTreeSet<Ranking>
    where
        Self: Sized,
    {
        let coalition = Coalition::all(m);

        coalition
            .into_iter()
            .map(|x| Fcs::new(&x, m))
            .map(|x| x.ranking)
            .collect()
    }
}
