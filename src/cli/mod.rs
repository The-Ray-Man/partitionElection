use clap::Parser;
use clap::Subcommand;

use crate::ballots::all_ballot_names;
use crate::proof::axiom::all_axiom_names;
use crate::proof::rule::all_rule_names;
pub mod profile;
pub mod proof;
pub mod score;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Manage profiles
    Profile(profile::Args),

    /// Axiom checking
    Proof(proof::Args),

    /// General Scoring Rule
    Score(score::Args),

    /// Shows all Ballots, Rules and Axioms
    Overview,
}

pub fn run(args: Cli) {
    match args.command {
        Commands::Profile(args) => profile::run(&args),
        Commands::Proof(args) => proof::run(&args),
        Commands::Score(args) => score::run(&args),
        Commands::Overview => overview(),
    }
}

pub fn overview() {
    let all_ballot_names = all_ballot_names();
    let all_rule_names = all_rule_names();
    let all_axiom_names = all_axiom_names();

    println!("There are {} Ballots:", all_ballot_names.len());

    for (short, long) in all_ballot_names {
        println!("\t{}\t{}", short, long);
    }

    println!("\nThrere are {} Rules:", all_rule_names.len());
    for rule in all_rule_names {
        println!("\t{rule}")
    }

    println!("\nThere are {} Axioms:", all_axiom_names.len());
    for (short, long) in all_axiom_names {
        println!("\t{}\t{}", short, long)
    }
}
