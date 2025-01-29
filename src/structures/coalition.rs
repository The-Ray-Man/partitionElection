use std::collections::BTreeSet;

use itertools::Itertools;

use crate::{
    structures::candidate::Candidate,
    utils::{structures::powerset, unordered_pair::UnorderedPair},
};

use super::Structure;

#[derive(Eq, Hash, Ord, PartialEq, PartialOrd, Clone, Debug)]
pub struct Coalition {
    pub members: BTreeSet<Candidate>,
}

impl Coalition {
    /// Returns true if the coalition contains all the candidate from ```other```.
    /// This means that the coalition ```self``` is a superset of the coalition ```other```.
    pub fn contains_weak(&self, other: &Coalition) -> bool {
        self.members.is_superset(&other.members)
    }

    /// Returns true if the coalition contains exactly the candidate from ```other```.
    pub fn contains(&self, other: &Candidate) -> bool {
        self.members.contains(other)
    }

    /// Returns true if the coalition contains exactly the candidate from ```other```.
    pub fn contains_pair(&self, pair: &(&Candidate, &Candidate)) -> bool {
        self.members.contains(pair.0) && self.members.contains(pair.1)
    }

    /// Returns all pairs of candidates that are in the coalition.
    pub fn all_pairs(&self) -> BTreeSet<UnorderedPair<Candidate>> {
        self.members
            .clone()
            .into_iter()
            .tuples()
            .map(|(a, b)| UnorderedPair::new(a, b))
            .collect()
    }
}

impl Structure for Coalition {
    fn all(m: usize) -> BTreeSet<Self>
    where
        Self: Sized,
    {
        let all_candidates = Candidate::all(m);
        powerset(&all_candidates.into_iter().collect())
            .into_iter()
            .filter(|x| !x.is_empty())
            .map(|x| Self {
                members: x.into_iter().cloned().collect::<BTreeSet<_>>(),
            })
            .collect()
    }

    fn is_legal(&self, m: usize) -> bool
    where
        Self: Sized,
    {
        !self.members.is_empty()
            && self.members.len() <= m
            && self.members.iter().all(|x| x.is_legal(m))
    }
}
