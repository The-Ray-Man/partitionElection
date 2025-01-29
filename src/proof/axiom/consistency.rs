use std::collections::BTreeSet;

use z3::ast::{Ast, Bool, Int};

use crate::proof::{profile::Profile, rule::VotingRule};
use crate::utils::structures::powerset_generator;

use super::{Axiom, AxiomType};

pub struct Consistency {}

impl Axiom for Consistency {
    fn condition_generator<'a: 'b, 'b>(
        profile: &'b Profile<'a>,
        rule: &'b Box<&'b (dyn VotingRule<'a> + 'b)>,
    ) -> impl Iterator<Item = Bool<'a>> + 'b {
        let ctx = profile.get_ctx();
        // Create the subprofiles
        let sub_profile1 = profile.create_new("'");
        let sub_profile2 = profile.create_new("''");

        // The sum of the votes of the subprofiles should be equal to the votes of the original profile
        let sum_correct = profile
            .votes
            .iter()
            .map(|(ranking, var)| {
                let var1 = sub_profile1.votes.get(ranking).unwrap(); // Save the subprofile contains every ranking which is contained in the profile
                let var2 = sub_profile2.votes.get(ranking).unwrap(); // Save the subprofile contains every ranking which is contained in the profile
                let sum = Int::add(ctx, &[var1, var2]);
                let equal = var._eq(&sum);
                equal
            })
            .collect::<Vec<_>>();
        let sum_condition = Bool::and(ctx, &sum_correct.iter().collect::<Vec<_>>());

        // Both subprofiles should be nonnegative and non-empty
        let non_negative1 = sub_profile1.vars_nonnegative();
        let non_negative2 = sub_profile2.vars_nonnegative();
        let sum_pos1 = sub_profile1.vars_sum_positive();
        let sum_pos2 = sub_profile2.vars_sum_positive();

        // Combining the preconditions.
        let precondition_formulas = vec![
            &non_negative1,
            &non_negative2,
            &sum_pos1,
            &sum_pos2,
            &sum_condition,
        ];
        let sub_profiles_preconditions = Bool::and(ctx, &precondition_formulas);

        // For every possible winning set
        powerset_generator(&profile.partitions).filter_map(move |winning_set| {
            if winning_set.is_empty() {
                return None;
            }
            log::info!(
                "Checking for winning set {}",
                winning_set
                    .iter()
                    .map(|x| { x.to_string() })
                    .collect::<Vec<_>>()
                    .join(", ")
            );
            // The winning set is the intersection of the winning sets of the subprofiles.
            let winner_set1 = rule.winner_set(&sub_profile1, &winning_set, None);
            let winner_set2 = rule.winner_set(&sub_profile2, &winning_set, None);

            // The winning set should be exact the intersection, therefore every partition not in the winning set must loose in either subprofile.
            let partitions = profile.partitions.iter().collect::<BTreeSet<_>>();
            let looser = partitions.difference(&winning_set).collect::<Vec<_>>();
            let condition = looser
                .into_iter()
                .map(|partition| {
                    let winner1 = rule.winner(partition, &sub_profile1, None);
                    let winner2 = rule.winner(partition, &sub_profile2, None);
                    let cond = Bool::and(ctx, &[&winner1, &winner2]);
                    cond.not()
                })
                .collect::<Vec<_>>();
            let exact_intersection = Bool::and(ctx, &condition.iter().collect::<Vec<_>>());

            // Whenever this holds, then the intersection (the winning set) should be the winner set of the original profile.
            let precondition = Bool::and(ctx, &[&winner_set1, &winner_set2, &exact_intersection]);
            let postcondition = rule.winner_set(profile, &winning_set, None);

            // The formula is (sub_profiles_preconditions and precondition) implies postcondition.
            let formula = Bool::and(ctx, &[&sub_profiles_preconditions, &precondition])
                .implies(&postcondition);
            Some(formula)
        })
    }

    fn get_type() -> AxiomType {
        AxiomType::Forall
    }
    fn condition<'a: 'b, 'b>(
        profile: &'b Profile<'a>,
        rule: &'b Box<&'b (dyn VotingRule<'a> + 'b)>,
    ) -> Vec<Bool<'a>> {
        let result = Consistency::condition_generator(profile, rule).collect::<Vec<_>>();
        result
    }

    fn short_name() -> &'static str {
        "cons"
    }

    fn full_name() -> &'static str {
        "consistency"
    }
}
