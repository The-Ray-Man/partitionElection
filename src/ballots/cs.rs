use core::panic;
use std::collections::{BTreeMap, BTreeSet, HashMap};

use super::ballot_trait;
use super::utils::ranking_from_scores;
use crate::structures::Structure;
use crate::structures::{Candidate, Partition};
use crate::structures::{Coalition, Ranking};

use itertools::Itertools;
use z3::ast::{Ast, Real};
use z3::{Config, Context, Solver};

type Scores = HashMap<Coalition, i32>;

#[derive(Eq, PartialEq, Clone)]
pub struct Cs {
    pub ballot: Scores,
    ranking: Ranking,
}

impl Cs {
    /// Creates a new Cs ballot.
    pub fn new(ballot: &Scores, m: usize) -> Cs {
        let partitions = Partition::all(m);

        let mut scores = BTreeMap::new();

        partitions.into_iter().for_each(|partition| {
            let sum = partition
                .coalitions
                .iter()
                .fold(0 as usize, |acc, coalition| {
                    acc + (*ballot.get(coalition).unwrap_or(&0) as usize)
                });
            scores.insert(partition, sum);
        });

        let ranking = ranking_from_scores(scores, true);
        Cs {
            ballot: ballot.clone(),
            ranking,
        }
    }

    /// Returns true, if the ranking is possible with the given ballot.
    fn possible_ranking(m: usize, ranking: Ranking) -> bool {
        let _candidates = Candidate::all(m);
        let partitions = Partition::all(m);
        let coalitions = Coalition::all(m);

        let config = Config::new();
        let context = Context::new(&config);

        let mut variables = HashMap::new();
        for coalition in &coalitions {
            let var = Real::new_const(&context, coalition.to_string());
            variables.insert(coalition.clone(), var);
        }

        let mut scores = HashMap::new();
        for partition in &partitions {
            let containing_pairs = coalitions
                .iter()
                .filter(|x| partition.contains(x))
                .collect::<Vec<_>>();
            let vars = containing_pairs
                .iter()
                .map(|x| variables.get(x).unwrap()) // This is safe because we know that the partition contains the coalition
                .collect::<Vec<_>>();
            if vars.is_empty() {
                let zero = z3::ast::Int::from_i64(&context, 0);
                scores.insert(partition, Real::from_int(&zero));
            } else {
                let score = Real::add(&context, &vars);
                scores.insert(partition, score);
            }
        }

        let solver = Solver::new(&context);
        for (partition_a, partition_b) in partitions.iter().tuple_combinations() {
            let score_a = scores.get(partition_a).unwrap(); // This is safe because the partition is in the hashmap
            let score_b = scores.get(partition_b).unwrap(); // This is safe because the partition is in the hashmap
            if ranking.is_strictly_preferred(partition_a, partition_b) {
                let constraint = score_a.gt(score_b);
                solver.assert(&constraint);
            } else if ranking.is_strictly_preferred(partition_b, partition_a) {
                let constraint = score_b.gt(score_a);
                solver.assert(&constraint);
            } else {
                let constraint = score_a._eq(score_b);
                solver.assert(&constraint);
            }
        }

        let result = solver.check();
        match result {
            z3::SatResult::Sat => true,
            z3::SatResult::Unsat => false,
            z3::SatResult::Unknown => panic!("Z3 solver returned unknown"),
        }
    }
}

impl ballot_trait::Ballot for Cs {
    fn get_ranking(&self) -> Ranking {
        self.ranking.clone()
    }

    fn get_name() -> String {
        "CS".to_string()
    }

    fn get_full_name() -> String {
        "Coalition Scores".to_string()
    }

    fn generate_all_rankings(m: usize) -> BTreeSet<Ranking>
    where
        Self: Sized,
    {
        let all_rankings = Ranking::all(m);

        all_rankings
            .into_iter()
            .filter(|x| Cs::possible_ranking(m, x.clone()))
            .collect::<BTreeSet<_>>()
    }
}
