use std::collections::BTreeSet;

use z3::ast::Bool;

use crate::structures::Coalition;

use crate::proof::{profile::Profile, rule::VotingRule};

use super::{Axiom, AxiomType};

enum SplitType {
    Weak,
    Strict,
}

enum WinningType {
    All,
    Some,
}

struct Split {}

impl Split {
    fn profile_weak_split<'a: 'b, 'b>(
        profile: &'b Profile<'a>,
        rule: &'b Box<&'b (dyn VotingRule<'a> + 'b)>,
        coalition: Coalition,
    ) -> Bool<'a> {
        let ctx = profile.get_ctx();
        let conditions = profile
            .votes
            .iter()
            .filter_map(|(ranking, _)| {
                if ranking.is_weakly_split(&coalition) {
                    None
                } else {
                    Some(rule.zero_votes(ranking, profile, None))
                }
            })
            .collect::<Vec<_>>();
        let conditions = conditions.iter().collect::<Vec<_>>();
        Bool::and(ctx, &conditions)
    }

    fn profile_strict_split<'a: 'b, 'b>(
        profile: &'b Profile<'a>,
        rule: &'b Box<&'b (dyn VotingRule<'a> + 'b)>,
        coalition: Coalition,
    ) -> Bool<'a> {
        let ctx = profile.get_ctx();
        let conditions = profile
            .votes
            .iter()
            .filter_map(|(ranking, _)| {
                if ranking.is_strict_split(&coalition) {
                    None
                } else {
                    Some(rule.zero_votes(ranking, profile, None))
                }
            })
            .collect::<Vec<_>>();
        let conditions = conditions.iter().collect::<Vec<_>>();
        Bool::and(ctx, &conditions)
    }

    fn winner_all_split<'a: 'b, 'b>(
        profile: &'b Profile<'a>,
        rule: &'b Box<&'b (dyn VotingRule<'a> + 'b)>,
        coalition: Coalition,
    ) -> Bool<'a> {
        let ctx = profile.get_ctx();
        let non_split = profile
            .partitions
            .iter()
            .filter(|partition| !partition.is_split(&coalition))
            .collect::<BTreeSet<_>>();

        let non_winning_conditions = non_split
            .iter()
            .map(|partition| rule.not_winner(partition, profile, None))
            .collect::<Vec<_>>();

        let conditions = non_winning_conditions.iter().collect::<Vec<_>>();
        Bool::and(ctx, &conditions)
    }

    fn winner_some_split<'a: 'b, 'b>(
        profile: &'b Profile<'a>,
        rule: &'b Box<&'b (dyn VotingRule<'a> + 'b)>,
        coalition: Coalition,
    ) -> Bool<'a> {
        let ctx = profile.get_ctx();
        let split = profile
            .partitions
            .iter()
            .filter(|partition| partition.is_split(&coalition))
            .collect::<BTreeSet<_>>();

        let winning_conditions = split
            .iter()
            .map(|partition| rule.winner(partition, profile, None))
            .collect::<Vec<_>>();

        let conditions = winning_conditions.iter().collect::<Vec<_>>();
        Bool::or(ctx, &conditions)
    }

    fn condition_generator<'a: 'b, 'b>(
        split_type: SplitType,
        winning_type: WinningType,
        profile: &'b Profile<'a>,
        rule: &'b Box<&'b (dyn VotingRule<'a> + 'b)>,
    ) -> impl Iterator<Item = Bool<'a>> + 'b {
        let _ctx = profile.get_ctx();

        profile.coalitions.iter().filter_map(move |coalition| {
            log::info!("Checking for Split {}", coalition.to_string());
            if coalition.members.is_empty() || coalition.members.len() == profile.num_candidates {
                return None;
            }
            let precondition = match split_type {
                SplitType::Weak => Split::profile_weak_split(profile, rule, coalition.clone()),
                SplitType::Strict => Split::profile_strict_split(profile, rule, coalition.clone()),
            };
            let winner_condition = match winning_type {
                WinningType::All => Split::winner_all_split(profile, rule, coalition.clone()),
                WinningType::Some => Split::winner_some_split(profile, rule, coalition.clone()),
            };
            let formula = precondition.implies(&winner_condition);
            Some(formula)
        })
    }
}

pub struct WeakSomeSplit {}

impl Axiom for WeakSomeSplit {
    fn condition_generator<'a: 'b, 'b>(
        profile: &'b Profile<'a>,
        rule: &'b Box<&'b (dyn VotingRule<'a> + 'b)>,
    ) -> impl Iterator<Item = Bool<'a>> + 'b {
        Split::condition_generator(SplitType::Weak, WinningType::Some, profile, rule)
    }

    fn get_type() -> AxiomType {
        AxiomType::Forall
    }

    fn condition<'a: 'b, 'b>(
        profile: &'b Profile<'a>,
        rule: &'b Box<&'b (dyn VotingRule<'a> + 'b)>,
    ) -> Vec<Bool<'a>> {
        WeakSomeSplit::condition_generator(profile, rule).collect()
    }
    fn short_name() -> &'static str {
        "splitws"
    }

    fn full_name() -> &'static str {
        "weakly-some-split"
    }
}

pub struct WeakAllSplit {}

impl Axiom for WeakAllSplit {
    fn condition_generator<'a: 'b, 'b>(
        profile: &'b Profile<'a>,
        rule: &'b Box<&'b (dyn VotingRule<'a> + 'b)>,
    ) -> impl Iterator<Item = Bool<'a>> + 'b {
        Split::condition_generator(SplitType::Weak, WinningType::All, profile, rule)
    }

    fn get_type() -> AxiomType {
        AxiomType::Forall
    }

    fn condition<'a: 'b, 'b>(
        profile: &'b Profile<'a>,
        rule: &'b Box<&'b (dyn VotingRule<'a> + 'b)>,
    ) -> Vec<Bool<'a>> {
        WeakAllSplit::condition_generator(profile, rule).collect()
    }
    fn short_name() -> &'static str {
        "splitwa"
    }

    fn full_name() -> &'static str {
        "weakly-all-split"
    }
}

pub struct StrictSomeSplit {}

impl Axiom for StrictSomeSplit {
    fn condition_generator<'a: 'b, 'b>(
        profile: &'b Profile<'a>,
        rule: &'b Box<&'b (dyn VotingRule<'a> + 'b)>,
    ) -> impl Iterator<Item = Bool<'a>> + 'b {
        Split::condition_generator(SplitType::Strict, WinningType::Some, profile, rule)
    }

    fn get_type() -> AxiomType {
        AxiomType::Forall
    }

    fn condition<'a: 'b, 'b>(
        profile: &'b Profile<'a>,
        rule: &'b Box<&'b (dyn VotingRule<'a> + 'b)>,
    ) -> Vec<Bool<'a>> {
        StrictSomeSplit::condition_generator(profile, rule).collect()
    }
    fn short_name() -> &'static str {
        "splitss"
    }

    fn full_name() -> &'static str {
        "strictly-some-split"
    }
}

pub struct StrictAllSplit {}

impl Axiom for StrictAllSplit {
    fn condition_generator<'a: 'b, 'b>(
        profile: &'b Profile<'a>,
        rule: &'b Box<&'b (dyn VotingRule<'a> + 'b)>,
    ) -> impl Iterator<Item = Bool<'a>> + 'b {
        Split::condition_generator(SplitType::Strict, WinningType::All, profile, rule)
    }

    fn get_type() -> AxiomType {
        AxiomType::Forall
    }

    fn condition<'a: 'b, 'b>(
        profile: &'b Profile<'a>,
        rule: &'b Box<&'b (dyn VotingRule<'a> + 'b)>,
    ) -> Vec<Bool<'a>> {
        StrictAllSplit::condition_generator(profile, rule).collect()
    }
    fn short_name() -> &'static str {
        "splitsa"
    }

    fn full_name() -> &'static str {
        "strictly-all-split"
    }
}
