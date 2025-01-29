use crate::{
    structures::{Ranking, Structure},
    utils::{io::read_from_file, io::write_rankings_to_file},
};
use std::collections::BTreeSet;

use std::path::Path;

pub trait Ballot {
    /// Returns the short name of the ballot
    fn get_name() -> String;

    /// Returns the full name of the ballot
    fn get_full_name() -> String;

    /// Returns all rankings possible with this ballot type.
    /// It reads them from a file. If the file does not exist, it will panic.
    /// The file can be generated with the `generate_ranking_file` method.
    fn all_rankings(m: usize) -> BTreeSet<Ranking> {
        let path = format!("logs/rankings/{}_{}.txt", m, Self::get_name());

        let lines = match read_from_file(&path) {
            Ok(lines) => lines,
            Err(err) => {
                eprintln!("Error: {:#?}", err);
                eprintln!(
                    "Maybe rankings for {} with {} candidates were not created yet.",
                    Self::get_name(),
                    m
                );
                std::process::exit(1)
            }
        };

        lines
            .into_iter()
            .enumerate()
            .map(|(i, x)| match x.parse::<Ranking>() {
                Ok(ranking) => {
                    if ranking.is_legal(m) {
                        ranking
                    } else {
                        eprintln!("A Parsing error in file {path} occurred on line: {}", i + 1);
                        std::process::exit(1)
                    }
                }
                Err(err) => {
                    eprintln!("Error: {:#?}", err);
                    eprintln!("A Parsing error in file {path} occurred on line: {}", i + 1);
                    std::process::exit(1)
                }
            })
            .collect::<BTreeSet<_>>()
    }

    /// Returns the induced ranking of the ballot
    fn get_ranking(&self) -> Ranking;

    /// Generates all possible rankings for a given number of candidates
    fn generate_all_rankings(m: usize) -> BTreeSet<Ranking>
    where
        Self: Sized;

    /// Writes all possible rankings for a given number of candidates to a file
    fn generate_ranking_file(m: usize)
    where
        Self: Sized,
    {
        let filename = format!("{}_{}.txt", m, Self::get_name());
        let path_str = format!("logs/rankings/{}.txt", filename);
        let path = Path::new(&path_str);

        if path.exists() {
            eprintln!("File {} already exists.", path_str);
            std::process::exit(1);
        }
        let rankings = Self::generate_all_rankings(m);

        write_rankings_to_file(rankings, &filename);
    }
}
