use std::collections::BTreeSet;

use super::ballot_trait;
use crate::structures::Partition;
use crate::structures::Structure;
use crate::structures::{Coalition, Ranking};

#[derive(Eq, Hash, Ord, PartialEq, PartialOrd, Clone)]
pub struct MyAxiom {
    pub coalition: Coalition,
    ranking: Ranking,
}

impl MyAxiom {
    /// Creates a new Fcw ballot.
    pub fn new(coalition: &Coalition, m: usize) -> Fcw {
        let partition = Partition::all(m);

        let (good, bad) =
            partition
                .into_iter()
                .fold((BTreeSet::new(), BTreeSet::new()), |(mut a, mut b), x| {
                    if x.contains_weak(coalition) {
                        a.insert(x.clone());
                    } else {
                        b.insert(x.clone());
                    }
                    (a, b)
                });

        let ranking = Ranking {
            ranking: vec![good, bad],
        };

        Fcw {
            coalition: coalition.clone(),
            ranking,
        }
    }
}

impl ballot_trait::Ballot for Fcw {
    fn get_ranking(&self) -> Ranking {
        self.ranking.clone()
    }

    fn get_name() -> String {
        "FCW".to_string()
    }

    fn get_full_name() -> String {
        "Favorite Coalition Weak".to_string()
    }

    fn generate_all_rankings(m: usize) -> BTreeSet<Ranking>
    where
        Self: Sized,
    {
        let coalitoins = Coalition::all(m);
        coalitoins
            .into_iter()
            .map(|x| Fcw::new(&x, m))
            .map(|x| x.ranking)
            .collect::<BTreeSet<_>>()
    }
}
