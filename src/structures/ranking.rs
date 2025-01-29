use itertools::Itertools;

use crate::{
    structures::partition::Partition,
    utils::{bell, structures::divide_into_classes},
};
use std::collections::BTreeSet;

use super::{Candidate, Coalition, Structure};

#[derive(Eq, Hash, Ord, PartialEq, PartialOrd, Clone, Debug)]
enum IndiffTypes {
    Split,
    Mixed,
    NotSplit,
    Failed,
}

#[derive(Eq, Hash, Ord, PartialEq, PartialOrd, Clone, Debug)]
pub struct Ranking {
    pub ranking: Vec<BTreeSet<Partition>>,
}

impl Ranking {
    /// Returns the number of indifference classes in the ranking.
    pub fn num_indifference_class(&self) -> usize {
        self.ranking.len()
    }

    /// Returns the order type of the ranking i.e. the size of each indifference class.
    pub fn order_type(&self) -> Vec<usize> {
        self.ranking.iter().map(|x| x.len()).collect()
    }

    /// Returns the index of the indifference class in the ranking, which contains the partition ```partition```.
    pub fn index(&self, partition: &Partition) -> usize {
        for (i, equiv) in self.ranking.iter().enumerate() {
            if equiv.contains(partition) {
                return i;
            }
        }
        panic!("Partition not found in ranking");
    }

    /// Returns true, if in the ranking ```self``` the partition ```partition1``` is strictly preferred over the partition ```partition2```.
    pub fn is_strictly_preferred(&self, partition1: &Partition, partition2: &Partition) -> bool {
        let index1 = self.index(partition1);
        let index2 = self.index(partition2);
        index1 < index2
    }

    /// Returns true, if in the ranking ```self``` the partition ```partition1``` is preferred over the partition ```partition2```.
    pub fn is_preferred(&self, partition1: &Partition, partition2: &Partition) -> bool {
        let index1 = self.index(partition1);
        let index2 = self.index(partition2);
        index1 <= index2
    }

    /// Returns true, if in the ranking ```self``` the partition set ```partitions``` is the same as the indifference class in the ranking at index ```class_index``` .
    pub fn is_exacty_equiv_class(
        &self,
        class_index: usize,
        partitions: &BTreeSet<&Partition>,
    ) -> bool {
        self.ranking[class_index]
            .iter()
            .collect::<BTreeSet<_>>()
            .eq(partitions)
    }

    /// Returns true, if the ranking ```self``` contains a partition ```partition``` in the indifference class at index ```index```, which contains the coalition ```coalition```.
    pub fn contains_coalition_in_indiff_class(
        &self,
        class_index: usize,
        coalition: &Coalition,
    ) -> bool {
        self.ranking[class_index]
            .iter()
            .any(|x| x.contains(coalition))
    }

    /// Returns true, if the ranking ```self``` contains a pair of candidates ```pair``` in the indifference class at index ```index```.
    pub fn contains_pair_in_indiff_class(
        &self,
        class_index: usize,
        pair: &(&Candidate, &Candidate),
    ) -> bool {
        self.ranking[class_index]
            .iter()
            .any(|x| x.contains_pair(pair))
    }

    /// Returns true, if every partition in the indifference class at index ```index``` contains the pair ```pair```.
    pub fn every_partition_contains_pair(
        &self,
        class_index: usize,
        pair: &(&Candidate, &Candidate),
    ) -> bool {
        self.ranking[class_index]
            .iter()
            .all(|x| x.contains_pair(pair))
    }

    /// Returns true, if every partition in the indifference class at index ```index``` contains the coalition ```coalition```.
    pub fn all_coalition_in_indiff_class(
        &self,
        indiff_index: usize,
        coalition: &Coalition,
    ) -> bool {
        self.ranking[indiff_index]
            .iter()
            .all(|x| x.contains(coalition))
    }

    fn split_helper(&self, coalition: &Coalition) -> Vec<IndiffTypes> {
        self.ranking
            .iter()
            .filter_map(|indiff_class| {
                if indiff_class.is_empty() {
                    return None;
                }

                if indiff_class.iter().all(|x| x.is_split(coalition)) {
                    Some(IndiffTypes::Split)
                } else if indiff_class.iter().all(|x| !x.is_split(coalition)) {
                    Some(IndiffTypes::NotSplit)
                } else {
                    Some(IndiffTypes::Mixed)
                }
            })
            .collect::<Vec<_>>()
    }

    /// Returns true if the ranking is a strict split for the coalition ```coalition```.
    pub fn is_strict_split(&self, coalition: &Coalition) -> bool {
        if self.ranking.iter().filter(|x| !x.is_empty()).count() == 1 {
            return false;
        }

        let helper = self.split_helper(coalition);
        let result = helper.iter().reduce(|acc, curr| match (acc, curr) {
            (IndiffTypes::Split, IndiffTypes::Split) => &IndiffTypes::Split,
            (IndiffTypes::NotSplit, IndiffTypes::NotSplit) => &IndiffTypes::NotSplit,
            (IndiffTypes::Split, IndiffTypes::NotSplit) => &IndiffTypes::NotSplit,
            _ => &IndiffTypes::Failed,
        });
        *result.unwrap() != IndiffTypes::Failed
    }

    /// Returns true if the ranking is a weak split for the coalition ```coalition```.
    pub fn is_weakly_split(&self, coalition: &Coalition) -> bool {
        let _non_empty = self
            .ranking
            .iter()
            .filter(|x| !x.is_empty())
            .collect::<Vec<_>>();
        if self.ranking.iter().filter(|x| !x.is_empty()).count() == 1 {
            return true;
        }
        let helper = self.split_helper(coalition);
        let result = helper.iter().reduce(|acc, curr| match (acc, curr) {
            (IndiffTypes::Split, IndiffTypes::Split) => &IndiffTypes::Split,
            (IndiffTypes::NotSplit, IndiffTypes::NotSplit) => &IndiffTypes::NotSplit,
            (IndiffTypes::Split, IndiffTypes::NotSplit) => &IndiffTypes::NotSplit,
            (IndiffTypes::Split, IndiffTypes::Mixed) => &IndiffTypes::Mixed,
            (IndiffTypes::Mixed, IndiffTypes::NotSplit) => &IndiffTypes::NotSplit,
            _ => &IndiffTypes::Failed,
        });
        *result.unwrap() != IndiffTypes::Failed
    }
}

impl Structure for Ranking {
    fn all(m: usize) -> BTreeSet<Ranking> {
        // 5 partitions
        let partitions = Partition::all(m);
        let mut result = BTreeSet::new();

        for num_equiv in 1..=partitions.len() {
            let equiv_classes = divide_into_classes(&partitions, num_equiv)
                .into_iter()
                .collect::<Vec<_>>();

            let permutations = equiv_classes
                .iter()
                .flat_map(|classes| {
                    classes
                        .iter()
                        .permutations(classes.len())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();

            let rankings = permutations
                .into_iter()
                .map(|x| Ranking {
                    ranking: x.into_iter().cloned().collect::<Vec<_>>(),
                })
                .collect::<BTreeSet<_>>();

            result.extend(rankings);
        }
        result
    }

    fn is_legal(&self, m: usize) -> bool
    where
        Self: Sized,
    {
        self.ranking
            .iter()
            .all(|equiv| equiv.iter().all(|partition| partition.is_legal(m)))
            && self.ranking.iter().map(|x| x.len()).sum::<usize>() == bell(m)
    }
}
