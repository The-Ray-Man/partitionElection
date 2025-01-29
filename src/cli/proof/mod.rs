use std::{fs, io};

use clap::Parser;
use log::warn;
use simplelog::{ConfigBuilder, LevelFilter, WriteLogger};

use crate::ballots::get_profile;
use crate::proof::Proof;
use crate::proof::{
    axiom::{
        add_axiom, all_axiom_names, check_iteratively, get_axiom_full_name, get_axiom_short_name,
    },
    rule::{get_rule, get_rule_name},
};

#[derive(Parser, Debug)]
pub struct Args {
    /// Number of Candidates
    #[arg(short, long)]
    pub candidates: u8,

    /// Voting rule
    #[arg(short, long)]
    pub rule: String,

    /// Ballot type
    #[arg(short, long)]
    pub ballot: String,

    /// Axiom to check
    #[arg(short, long, num_args = 0..)]
    pub axiom: Vec<String>,

    /// Search for counterexample (only possible for one axiom at a time)
    #[arg(short, long)]
    pub iteratively: bool,

    /// Show intermediate results
    #[arg(short, long, default_value = "false")]
    pub verbose: bool,

    /// To file
    #[arg(short, long)]
    pub output: bool,
}

fn setup_logging(args: &Args) {
    let mut level = if args.verbose || args.output {
        LevelFilter::Info
    } else {
        LevelFilter::Warn
    };
    let mut builder = ConfigBuilder::new();
    builder.set_time_level(LevelFilter::Off);
    builder.set_max_level(LevelFilter::Warn);
    let config = builder.build();

    if args.output {
        let rule_name = get_rule_name(&args.rule);
        let axioms_names = if args.axiom.is_empty() {
            level = LevelFilter::Warn;
            all_axiom_names()
                .into_iter()
                .map(|(short, _)| short)
                .collect::<Vec<_>>()
        } else if args.axiom.len() == 1 {
            Vec::from([get_axiom_full_name(&args.axiom[0])])
        } else {
            level = LevelFilter::Warn;
            args.axiom
                .iter()
                .map(|name| get_axiom_short_name(name))
                .collect::<Vec<_>>()
        };
        let axiom_names = axioms_names.join("-");
        let path = format!("logs/proofs/{}/", rule_name);
        let filename = format!("{}{}_{}.log", args.candidates, args.ballot, axiom_names);
        fs::create_dir_all(path.clone()).unwrap();
        let file = std::fs::File::create(format!("{}/{}", path, filename)).unwrap();
        let _ = WriteLogger::init(level, config, file);
    } else {
        let _ = WriteLogger::init(level, config, io::stdout());
    };
}

pub fn run(args: &Args) {
    let mut axioms: Vec<String> = args.axiom.clone();
    if axioms.is_empty() {
        axioms = all_axiom_names()
            .into_iter()
            .map(|(short, _)| short.to_string())
            .collect::<Vec<String>>();
        eprintln!("No axiom provided. We will take all axioms");
    }

    if axioms.len() > 1 && args.iteratively {
        eprintln!("Iteratively checking multiple axioms is not supported");
        std::process::exit(1);
    }

    let ctx = z3::Context::new(&z3::Config::new());
    let rule = get_rule(args.candidates as usize, &args.rule);
    let rule = Box::leak(rule);
    let rule = rule as &dyn crate::proof::rule::VotingRule;
    let profile = get_profile(args.candidates as usize, &args.ballot, &ctx);
    let mut proof = Proof::new(args.candidates as usize, rule, profile);
    setup_logging(args);

    log::info!("Starting proof");

    if args.iteratively {
        let axiom_name = args.axiom.first().unwrap();
        let result = check_iteratively(axiom_name, &mut proof);
        log::warn!("FINAL RESULT: {:?}", result);
        return;
    }

    for axiom in axioms.iter() {
        warn!("Adding axiom {}", axiom);
        add_axiom(axiom, &mut proof);
    }

    let (result, _) = proof.check();
    log::warn!("FINAL RESULT: {:?}", result);
}

// pub fn create_proof<'a : 'b, 'b>(
//     candidates: u8,
//     rule: &'b str,
//     ballot: &'b str,
//     ctx: &'a Context,
// ) -> Proof<'a> {
//     let rule = get_rule(candidates as usize, rule);
//     let profile = get_profile(candidates as usize, ballot, ctx);
//     let proof= Proof::new(candidates as usize, rule, profile);
//     proof
// }
