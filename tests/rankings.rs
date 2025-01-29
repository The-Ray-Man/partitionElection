#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use partitionElection::ballots::{Ballot, Fp, Pa};
    use partitionElection::structures::Structure;
    use partitionElection::structures::{Candidate, Coalition, Partition, Ranking};

    #[test]
    fn test_split1() {
        let m = 3;
        let rankings = Ranking::all(m);
        let coalitions = Coalition::all(m);
        let allowed_coalitions = coalitions
            .iter()
            .filter(|x| x.members.len() > 1 && x.members.len() < m)
            .collect::<BTreeSet<_>>();
        for ranking in rankings {
            for coalition in allowed_coalitions.iter() {
                let strict_split = ranking.is_strict_split(coalition);
                let weak_split = ranking.is_weakly_split(coalition);
                assert!(
                    !(weak_split && !strict_split),
                    "Weak split but not strict split"
                );
            }
        }
    }

    #[test]
    fn test_split2() {
        let m = 3;
        let rankings = Fp::all_rankings(m);
        let coalitions = Coalition::all(m);
        let allowed_coalitions = coalitions
            .iter()
            .filter(|x| x.members.len() > 1 && x.members.len() < m)
            .collect::<BTreeSet<_>>();

        for ranking in rankings {
            for coalition in &allowed_coalitions {
                let is_strict_split = ranking.is_strict_split(coalition);
                assert!(!is_strict_split)
            }
        }
    }

    #[test]
    fn test_split3() {
        let m = 3;
        let member = Candidate {
            name: "a".to_string(),
        };
        let coalition = Coalition {
            members: (BTreeSet::from([member])),
        };
        let partitions = Partition::all(m);
        let ranking = Pa::new(partitions, 3).get_ranking();

        assert!(ranking.is_weakly_split(&coalition))
    }
}
