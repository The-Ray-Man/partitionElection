use std::collections::BTreeSet;

use super::ballot_trait;
use crate::structures::Partition;
use crate::structures::Ranking;
use crate::structures::Structure;
use crate::utils::structures::powerset;

#[derive(Eq, Hash, Ord, PartialEq, PartialOrd, Clone)]
pub struct Pa {
    pub approved: BTreeSet<Partition>,
    ranking: Ranking,
}

impl Pa {
    /// Creates a new Pa ballot.
    pub fn new(approved: BTreeSet<Partition>, m: usize) -> Pa {
        let partition = Partition::all(m);

        let (good, bad) =
            partition
                .into_iter()
                .fold((BTreeSet::new(), BTreeSet::new()), |(mut a, mut b), x| {
                    if approved.contains(&x) {
                        a.insert(x.clone());
                    } else {
                        b.insert(x.clone());
                    }
                    (a, b)
                });

        let ranking = Ranking {
            ranking: vec![good, bad],
        };

        Pa { approved, ranking }
    }
}

impl ballot_trait::Ballot for Pa {
    fn get_ranking(&self) -> Ranking {
        self.ranking.clone()
    }

    fn get_name() -> String {
        "PA".to_string()
    }

    fn get_full_name() -> String {
        "Partition Approval".to_string()
    }

    fn generate_all_rankings(m: usize) -> BTreeSet<Ranking>
    where
        Self: Sized,
    {
        let partitions = Partition::all(m);

        let possible_approvals = powerset(&partitions)
            .into_iter()
            .filter(|x| !x.is_empty())
            .collect::<Vec<_>>();

        possible_approvals
            .into_iter()
            .map(|x| Pa::new(x.into_iter().cloned().collect::<BTreeSet<_>>(), m))
            .map(|x| x.ranking)
            .collect()
    }
}
