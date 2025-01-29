use std::collections::BTreeSet;

use clap::Parser;
use z3::{Model, SatResult};

use crate::{
    ballots::get_profile,
    proof::{axiom::add_axiom, rule::Scoring, Proof},
};

#[derive(Parser, Debug)]
pub struct Args {
    /// Number of Candidates
    #[arg(short, long)]
    pub candidates: u8,

    /// Ballot type
    #[arg(short, long)]
    pub ballot: String,

    /// Axiom to check
    #[arg(short, long, num_args = 0..)]
    pub axiom: Vec<String>,
}

fn print_scoring_system(order_types: BTreeSet<Vec<usize>>, model: Model, rule: &Scoring) {
    for order_type in order_types {
        let score = rule.get_score_vector(&order_type);
        let values = score
            .iter()
            .map(|x| {
                let s = model.get_const_interp(x).unwrap();
                let (a, b) = s.as_real().unwrap();
                format!("{}/{}", a, b)
            })
            .collect::<Vec<_>>();

        println!("{:?} -> {:?}", order_type, values);
    }
}

pub fn run(args: &Args) {
    let ctx = z3::Context::new(&z3::Config::new());

    let m = args.candidates as usize;

    let profile = get_profile(m, &args.ballot, &ctx);

    let order_types = profile
        .votes
        .iter()
        .map(|(ranking, _)| ranking.order_type())
        .collect::<BTreeSet<_>>();

    let rule = Scoring::create(args.candidates as usize, order_types.clone(), &ctx);

    let mut proof = Proof::new(m, &rule, profile);

    for axiom in &args.axiom {
        println!("Adding axiom: {}", axiom);
        add_axiom(axiom, &mut proof);
    }

    println!("Checking proof");
    let (result, model) = proof.check();

    match result {
        SatResult::Sat => {
            let model = model.unwrap();
            print_scoring_system(order_types, model, &rule);
        }
        _ => {
            println!("{:?}", result);
        }
    }
}
