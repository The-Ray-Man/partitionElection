#[cfg(test)]
mod tests {
    use partitionElection::proof::profile::Profile;
    use partitionElection::structures::Structure;
    use std::collections::{BTreeMap, BTreeSet};

    use partitionElection::proof::rule::Borda;
    use partitionElection::proof::rule::VotingRule;
    use partitionElection::structures::{Partition, Ranking};

    use z3::ast::Ast;

    #[test]
    fn test_borda_score_1() {
        let m = 3;
        let partitions = Partition::all(m);
        let num_partitions = partitions.len();
        let ranked = partitions
            .clone()
            .into_iter()
            .map(|x| {
                let mut set = BTreeSet::new();
                set.insert(x);
                set
            })
            .collect::<Vec<_>>();
        let ranking = Ranking { ranking: ranked };
        let ctx = z3::Context::new(&z3::Config::new());

        let mut votes = BTreeMap::new();
        votes.insert(ranking, 2);

        let profile = Profile::from_custom(3, &ctx, votes);

        let borda = Borda::new(3);

        for (i, partition) in partitions.iter().enumerate() {
            let score: z3::ast::Real<'_> = borda.score(partition, &profile, None);
            assert_eq!(
                score.simplify().to_string(),
                ((num_partitions - 1 - i) * 2).to_string()
            );
        }
        // assert!(false);
    }

    #[test]
    fn test_borda_score_2() {
        let m = 3;
        let partitions = Partition::all(m);
        let num_partitions = partitions.len();
        let mut ranked = vec![];
        ranked.push(partitions.clone());

        let ranking = Ranking { ranking: ranked };
        let ctx = z3::Context::new(&z3::Config::new());

        let mut votes = BTreeMap::new();
        votes.insert(ranking, 2);

        let profile = Profile::from_custom(3, &ctx, votes);

        let borda = Borda::new(3);

        for (_i, partition) in partitions.iter().enumerate() {
            let score: z3::ast::Real<'_> = borda.score(partition, &profile, None);
            assert_eq!(
                score.simplify().to_string(),
                ((num_partitions - 1) * 2).to_string()
            );
        }
    }
}
