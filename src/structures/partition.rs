use std::collections::BTreeSet;

use super::{Candidate, Structure};
use crate::{
    structures::coalition::Coalition,
    utils::{structures::partition, unordered_pair::UnorderedPair},
};
use itertools::Itertools;

#[derive(Eq, Hash, Ord, PartialEq, PartialOrd, Clone, Debug)]
pub struct Partition {
    pub coalitions: BTreeSet<Coalition>,
}

impl Partition {
    /// Returns true if the partition contains exactly the coalition ```coalition```.
    pub fn contains(&self, coalition: &Coalition) -> bool {
        self.coalitions.contains(coalition)
    }

    /// Returns true if the partition contains the coalition ```coalition``` or a coalition that is a superset of ```coalition```.
    pub fn contains_weak(&self, coalition: &Coalition) -> bool {
        self.coalitions.iter().any(|x| x.contains_weak(coalition))
    }

    /// Returns true if the partition contains the pair ```pair```.
    pub fn contains_pair(&self, pair: &(&Candidate, &Candidate)) -> bool {
        self.coalitions.iter().any(|x| x.contains_pair(pair))
    }

    /// Returns all pairs of candidates that are in the partition.
    pub fn all_pairs(&self) -> BTreeSet<UnorderedPair<Candidate>> {
        self.coalitions.iter().flat_map(|x| x.all_pairs()).collect()
    }

    // Returns true if the partition is ```coalition```-split
    pub fn is_split(&self, coalition: &Coalition) -> bool {
        if self.coalitions.is_empty() {
            return true;
        }

        let candidates = self
            .coalitions
            .iter()
            .flat_map(|x| x.members.clone())
            .collect::<BTreeSet<_>>();
        assert!(
            !coalition.members.is_empty() && coalition.members.len() < candidates.len(),
            "The split must not be trivial"
        );

        let other = candidates
            .iter()
            .filter(|x| !coalition.contains(x))
            .collect::<BTreeSet<_>>();
        coalition
            .members
            .iter()
            .cartesian_product(other.iter())
            .all(|x| !self.contains_pair(&(x.0, x.1)))
    }

    /// Returns all partitions that are one editing distance away from the partition.
    pub fn distance_one(&self) -> BTreeSet<Partition> {
        let mut result = BTreeSet::new();

        let candidates = self
            .coalitions
            .iter()
            .flat_map(|x| x.members.clone())
            .collect::<Vec<_>>();

        for candidate in candidates {
            let new_coalition = self
                .coalitions
                .iter()
                .filter_map(|x| {
                    if x.contains(&candidate) {
                        let mut new_coalition = x.clone();
                        new_coalition.members.remove(&candidate);
                        if new_coalition.members.is_empty() {
                            None
                        } else {
                            Some(new_coalition)
                        }
                    } else {
                        Some(x.clone())
                    }
                })
                .collect::<Vec<_>>();

            let mut partition_singelton = new_coalition.clone();
            partition_singelton.push(Coalition {
                members: vec![candidate.clone()].into_iter().collect(),
            });
            result.insert(Partition {
                coalitions: partition_singelton.into_iter().collect(),
            });

            for i in 0..new_coalition.len() {
                let mut partition = new_coalition.clone();
                partition[i].members.insert(candidate.clone());
                result.insert(Partition {
                    coalitions: partition.into_iter().collect(),
                });
            }
        }

        result
    }
}

impl Structure for Partition {
    fn all(m: usize) -> BTreeSet<Self>
    where
        Self: Sized,
    {
        let mut candidates = Candidate::all(m);
        partition(&mut candidates)
            .into_iter()
            .map(|x| Self {
                coalitions: x.into_iter().map(|y| Coalition { members: y }).collect(),
            })
            .collect()
    }

    fn is_legal(&self, m: usize) -> bool
    where
        Self: Sized,
    {
        !self.coalitions.is_empty()
            && self.coalitions.len() <= m
            && self.coalitions.iter().all(|x| x.is_legal(m))
            && self
                .coalitions
                .iter()
                .map(|x| x.members.len())
                .sum::<usize>()
                == m
    }
}
