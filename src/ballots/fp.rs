use std::collections::{BTreeMap, BTreeSet, VecDeque};

use super::ballot_trait;
use super::utils::ranking_from_scores;
use crate::structures::Partition;
use crate::structures::Ranking;
use crate::structures::Structure;

#[derive(Eq, Hash, Ord, PartialEq, PartialOrd, Clone)]
pub struct Fp {
    pub favorite: Partition,
    ranking: Ranking,
}

impl Fp {
    /// Creates a new Fp ballot.
    pub fn new(favorite: Partition, _m: usize) -> Fp {
        let distances = Fp::bfs(favorite.clone());

        let ranking = ranking_from_scores(distances, false);

        Fp { favorite, ranking }
    }

    /// Breadth-first search to calculate the distance of each partition to the favorite partition.
    fn bfs(start: Partition) -> BTreeMap<Partition, usize> {
        let mut visited = BTreeMap::new();
        let mut queue = VecDeque::new();
        queue.push_back(start.clone());
        visited.insert(start.clone(), 0);

        while let Some(current) = queue.pop_front() {
            let distance = visited[&current];
            for neighbor in current.distance_one() {
                if !visited.contains_key(&neighbor) {
                    visited.insert(neighbor.clone(), distance + 1);
                    queue.push_back(neighbor);
                }
            }
        }

        visited
    }
}

impl ballot_trait::Ballot for Fp {
    fn get_ranking(&self) -> Ranking {
        self.ranking.clone()
    }

    fn get_name() -> String {
        "FP".to_string()
    }

    fn get_full_name() -> String {
        "Favorite Partition".to_string()
    }

    fn generate_all_rankings(m: usize) -> BTreeSet<Ranking>
    where
        Self: Sized,
    {
        let partitions = Partition::all(m);

        partitions
            .into_iter()
            .map(|x| Fp::new(x, m))
            .map(|x| x.ranking)
            .collect()
    }
}
