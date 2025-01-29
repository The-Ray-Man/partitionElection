use crate::structures::{Partition, Ranking};
use std::collections::{BTreeMap, BTreeSet};

/// Creates a ranking from a map of scores. If reversed is true, a higher score is better.
pub fn ranking_from_scores(scores: BTreeMap<Partition, usize>, reversed: bool) -> Ranking {
    // Reverse the map.
    let reversed_map = scores.iter().fold(
        BTreeMap::new(),
        |mut acc: BTreeMap<usize, BTreeSet<Partition>>, (partition, score)| {
            let vec = acc.get_mut(score);
            match vec {
                Some(x) => {
                    let _ = x.insert(partition.clone());
                }
                None => {
                    let _ = acc.insert(*score, BTreeSet::from([partition.clone()]));
                }
            }
            acc
        },
    );

    // sort by distances
    let mut ranking = reversed_map.into_iter().collect::<Vec<_>>();
    ranking.sort_by(|(a, _), (b, _)| a.cmp(b));
    let mut ranking = ranking
        .into_iter()
        .map(|(_, x)| x.into_iter().collect())
        .collect::<Vec<_>>();

    if reversed {
        ranking.reverse();
    }
    Ranking { ranking }
}
