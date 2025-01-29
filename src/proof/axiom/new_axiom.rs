use std::collections::BTreeSet;

use z3::ast::{Ast, Bool, Int};

use crate::proof::{profile::Profile, rule::VotingRule};
use crate::utils::structures::powerset_generator;

use super::{Axiom, AxiomType};

pub struct MyAxiom {}

impl Axiom for MyAxiom {
    fn condition_generator<'a: 'b, 'b>(
        profile: &'b Profile<'a>,
        rule: &'b Box<&'b (dyn VotingRule<'a> + 'b)>,
    ) -> impl Iterator<Item = Bool<'a>> + 'b {
        [0..1].iter().map(|_|{
            todo!()
        }
        )
    }

    fn get_type() -> AxiomType {
        AxiomType::Forall
    }
    fn condition<'a: 'b, 'b>(
        profile: &'b Profile<'a>,
        rule: &'b Box<&'b (dyn VotingRule<'a> + 'b)>,
    ) -> Vec<Bool<'a>> {
        let result = MyAxiom::condition_generator(profile, rule).collect::<Vec<_>>();
        result
    }

    fn short_name() -> &'static str {
        "myaxiom"
    }

    fn full_name() -> &'static str {
        "My Axiom"
    }
}
