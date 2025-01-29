use std::collections::BTreeSet;

use std::collections::BTreeMap;

use z3::ast::Ast;
use z3::ast::{Bool, Int};
use z3::Context;

use crate::ballots::Ballot;

use crate::structures::Candidate;
use crate::structures::Coalition;
use crate::structures::Structure;
use crate::structures::{Partition, Ranking};

#[derive(Debug)]
pub struct Profile<'ctx> {
    pub votes: BTreeMap<Ranking, Int<'ctx>>,
    pub partitions: BTreeSet<Partition>,
    pub coalitions: BTreeSet<Coalition>,
    pub candidates: BTreeSet<Candidate>,
    pub num_candidates: usize,
}

impl<'ctx> Profile<'ctx> {
    pub fn from_custom(m: usize, ctx: &'ctx Context, votes: BTreeMap<Ranking, usize>) -> Self {
        let partitions = Partition::all(m);
        let coalitions = Coalition::all(m);
        let candidates = Candidate::all(m);
        let num_partitions = partitions.len();

        if votes.is_empty() {
            eprintln!("Error: No votes provided");
            std::process::exit(1)
        }

        let votes = {
            votes
                .into_iter()
                .map(move |(ranking, count)| {
                    let var = Int::from_i64(ctx, count as i64);
                    (ranking, var)
                })
                .collect::<BTreeMap<_, _>>()
        };

        Profile {
            votes,
            partitions,
            coalitions,
            candidates,
            num_candidates: num_partitions,
        }
    }

    /// Creates a profile with the rankings possible with the given ballot type.
    pub fn from_ballot<T: Ballot>(m: usize, ctx: &'ctx Context) -> Self {
        let partitions = Partition::all(m).into_iter().collect();
        let coalitions = Coalition::all(m);
        let candidates = Candidate::all(m);
        let rankings = T::all_rankings(m);
        let mut votes = BTreeMap::new();
        for (i, ranking) in rankings.into_iter().enumerate() {
            let var = Int::new_const(ctx, format!("k_{}", i));
            votes.insert(ranking, var);
        }

        Profile {
            votes,
            partitions,
            coalitions,
            candidates,
            num_candidates: m,
        }
    }

    /// Returns the Z3 context of the profile.
    pub fn get_ctx(&self) -> &'ctx Context {
        self.votes.iter().next().unwrap().1.get_ctx() // The profile can never be empty.
    }

    /// Returns all variables in the profile.
    pub fn all_vars(&self) -> Vec<&Int<'ctx>> {
        self.votes.values().collect::<Vec<_>>()
    }

    /// Returns the Z3 condition that all variables are non-negative.
    pub fn vars_nonnegative(&self) -> Bool<'ctx> {
        let ctx = self.get_ctx();
        let zero = Int::from_i64(ctx, 0);
        let vars = self
            .all_vars()
            .iter()
            .map(|var| var.ge(&zero))
            .collect::<Vec<_>>();
        let vars = vars.iter().collect::<Vec<_>>();
        Bool::and(ctx, &vars)
    }

    /// Returns the Z3 condition that the sum of all variables is positive i.e. there are some votes.
    pub fn vars_sum_positive(&self) -> Bool<'ctx> {
        let ctx = self.get_ctx();
        let zero = Int::from_i64(ctx, 0);

        let vars = self.all_vars();
        let sum = Int::add(ctx, &vars);
        sum.gt(&zero)
    }

    /// Creates a new profile with the same rankings, however different variables.
    /// The extra votes are discarded.
    /// ```s``` is the prefix of the variables
    pub fn create_new(&self, s: &str) -> Self {
        let partitions = self.partitions.clone();
        let coalitions = self.coalitions.clone();
        let candidates = self.candidates.clone();
        let num_candidates = self.num_candidates;

        let mut votes = BTreeMap::new();
        let ctx = self.get_ctx();

        for (i, (ranking, _)) in self.votes.iter().enumerate() {
            let var = Int::new_const(ctx, format!("k_{}{}", i, s));
            votes.insert(ranking.clone(), var);
        }
        Profile {
            votes,
            partitions,
            coalitions,
            candidates,
            num_candidates,
        }
    }
}
