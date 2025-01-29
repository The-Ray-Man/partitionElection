use clap::{Parser, Subcommand};

use crate::ballots::create_profile_file;
mod utils;

#[derive(Subcommand, Debug)]
pub enum ProfileCommands {
    /// List all profiles
    List,

    /// Print a profile
    Print {
        /// Profile name
        #[arg(short, long)]
        name: String,
    },

    /// Add a profile
    Create {
        /// Ballot name
        #[arg(short, long)]
        ballot: String,

        /// Candidate number
        #[arg(short, long, default_value = "3")]
        m: u8,
    },
}

#[derive(Parser, Debug)]
pub struct Args {
    #[clap(subcommand)]
    pub command: ProfileCommands,
}

pub fn run(args: &Args) {
    match &args.command {
        ProfileCommands::List => {
            utils::list();
        }
        ProfileCommands::Print { name } => {
            utils::list_profile(name);
        }
        ProfileCommands::Create { ballot, m } => {
            create_profile_file(*m as usize, ballot);
        }
    }
}
