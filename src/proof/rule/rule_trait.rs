use std::collections::{BTreeMap, BTreeSet};

use z3::ast::{Ast, Real};
use z3::ast::{Bool, Int};

use crate::{
    proof::profile::Profile,
    structures::{Partition, Ranking},
};

pub trait VotingRule<'a> {
    /// Creates a new instance of the rule.
    fn new(m: usize) -> Self
    where
        Self: Sized;

    /// Returns the name of the rule.
    fn name() -> &'static str
    where
        Self: Sized;

    /// Returns all possible partitions.
    fn all_partitions(&self) -> BTreeSet<Partition>;

    /// Returns the score of ```partition``` in the profile.
    fn score(
        &self,
        partition: &Partition,
        profile: &Profile<'a>,
        extra_votes: Option<&BTreeMap<Ranking, Int<'a>>>,
    ) -> Real<'a>;

    /// Returns the condition for ```partition``` to be the only winner.
    fn only_winner(
        &self,
        partition: &Partition,
        profile: &Profile<'a>,
        extra_votes: Option<&BTreeMap<Ranking, Int<'a>>>,
    ) -> Bool<'a> {
        let ctx = profile.get_ctx();
        let score = self.score(partition, profile, extra_votes);
        let scores = self
            .all_partitions()
            .into_iter()
            .filter_map(|x| {
                if &x == partition {
                    None
                } else {
                    let other_score = self.score(&x, profile, extra_votes);
                    let condition = score.gt(&other_score);
                    Some(condition)
                }
            })
            .collect::<Vec<_>>();

        let scores_vec: Vec<_> = scores.iter().collect();
        Bool::and(ctx, scores_vec.as_slice())
    }

    // Returns the condition for ```partition``` to not be a winner.
    fn not_winner(
        &self,
        partition: &Partition,
        profile: &Profile<'a>,
        extra_votes: Option<&BTreeMap<Ranking, Int<'a>>>,
    ) -> Bool<'a> {
        let winner_condition = self.winner(partition, profile, extra_votes);
        winner_condition.not()
    }

    /// Returns the condition for ```partition``` to be a winner.
    fn winner(
        &self,
        partition: &Partition,
        profile: &Profile<'a>,
        extra_votes: Option<&BTreeMap<Ranking, Int<'a>>>,
    ) -> Bool<'a> {
        let ctx = profile.get_ctx();
        let score = self.score(partition, profile, extra_votes);
        let scores: Vec<Bool<'a>> = self
            .all_partitions()
            .into_iter()
            .map(|x| {
                let other_score = self.score(&x, profile, extra_votes);
                let condition = score.ge(&other_score);
                condition
            })
            .collect::<Vec<_>>();

        let scores_vec: Vec<&Bool<'a>> = scores.iter().collect();
        Bool::and(ctx, scores_vec.as_slice())
    }

    /// Returns the condition for ```winning_set``` to be winners (there may be additional winners).
    fn winner_set(
        &self,
        profile: &Profile<'a>,
        winning_set: &BTreeSet<&Partition>,
        extra_votes: Option<&BTreeMap<Ranking, Int<'a>>>,
    ) -> Bool<'a> {
        let ctx = profile.get_ctx();
        let winning_set_are_winners: Vec<Bool<'a>> = winning_set
            .iter()
            .map(|winner| {
                let condition = self.winner(winner, profile, extra_votes);
                condition
            })
            .collect::<Vec<_>>();

        Bool::and(ctx, &winning_set_are_winners.iter().collect::<Vec<_>>())
    }

    /// Returns the condition for ```winning_set``` to be the only winners.
    fn exact_winner_set(
        &self,
        winning_set: &BTreeSet<&Partition>,
        profile: &Profile<'a>,
        extra_votes: Option<&BTreeMap<Ranking, Int<'a>>>,
    ) -> Bool<'a> {
        let ctx = profile.get_ctx();
        let winning_set_are_winners: Vec<Bool<'a>> = winning_set
            .iter()
            .map(|winner| {
                let condition = self.winner(winner, profile, extra_votes);
                condition
            })
            .collect::<Vec<_>>();

        let loosers = self
            .all_partitions()
            .into_iter()
            .filter(|x| !winning_set.contains(x))
            .collect::<Vec<_>>();
        let winning_set_are_only_winners: Vec<Bool<'a>> = loosers
            .iter()
            .map(|looser| self.not_winner(looser, profile, extra_votes))
            .collect::<Vec<_>>();

        let all_conditions: Vec<&Bool<'a>> = winning_set_are_winners
            .iter()
            .chain(winning_set_are_only_winners.iter())
            .collect::<Vec<_>>();

        let condition: Bool<'a> = Bool::and(ctx, &all_conditions);
        condition
    }

    /// Returns the condition for ```partition_a``` to be tied with ```partition_b```.
    fn tied(
        &'a self,
        partition_a: &Partition,
        partition_b: &Partition,
        profile: &Profile<'a>,
        extra_votes: Option<&BTreeMap<Ranking, Int<'a>>>,
    ) -> Bool<'a> {
        let score_a = self.score(partition_a, profile, extra_votes);
        let score_b = self.score(partition_b, profile, extra_votes);
        let condition = score_a._eq(&score_b);
        condition
    }

    /// Returns the condition for ```ranking``` to receive zero votes.
    fn zero_votes(
        &self,
        ranking: &Ranking,
        profile: &Profile<'a>,
        extra_votes: Option<&BTreeMap<Ranking, Int<'a>>>,
    ) -> Bool<'a> {
        let ctx = profile.get_ctx();
        let variable = profile
            .votes
            .get(ranking)
            .unwrap_or_else(|| panic!("The ranking {:?} is not in the profile", ranking,));
        match extra_votes {
            Some(extra_votes) => {
                if let Some(extra_vote) = extra_votes.get(ranking) {
                    let total_votes = z3::ast::Int::<'_>::add(ctx, &[extra_vote, variable]);
                    total_votes._eq(&Int::from_i64(ctx, 0))
                } else {
                    variable._eq(&Int::from_i64(ctx, 0))
                }
            }
            None => {
                let condition = variable._eq(&Int::from_i64(ctx, 0));
                condition
            }
        }
    }
}
