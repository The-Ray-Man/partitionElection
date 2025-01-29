use prettytable::{format, row, Cell, Row, Table};
use std::fs;
use z3::{Config, Context};

use crate::ballots::{name_to_full_name, *};

struct ProfileMetadata {
    candidate_size: u8,
    ballot_name: String,
    num_rankings: usize,
}

pub fn list() {
    let paths = fs::read_dir("logs/rankings");

    if paths.is_err() {
        eprintln!("Could not read files in logs/rankings");
        std::process::exit(1);
    }
    let paths = paths.unwrap();

    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);

    table.set_titles(row![
        "Name",
        "Candidate Size",
        "Ballot Name",
        "Number of Rankings"
    ]);

    let mut profiles = paths
        .into_iter()
        .filter_map(|path| {
            let path = &path.unwrap().path();
            if path.is_file() {
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let file_name = file_name.split('.').next().unwrap();
                let file_name = file_name.split('_').collect::<Vec<&str>>();
                let candidate_size = file_name[0];
                let ballot_name = file_name[1];
                let num_rankings = fs::read_to_string(path).unwrap().lines().count();
                return Some(ProfileMetadata {
                    candidate_size: candidate_size.parse().unwrap(),
                    ballot_name: ballot_name.to_string(),
                    num_rankings,
                });
            }
            None
        })
        .collect::<Vec<_>>();

    profiles.sort_by(|a, b| match a.candidate_size.cmp(&b.candidate_size) {
        std::cmp::Ordering::Equal => a.ballot_name.cmp(&b.ballot_name),
        other => other,
    });

    for profile in profiles {
        table.add_row(Row::new(vec![
            Cell::new(format!("{}_{}", profile.candidate_size, profile.ballot_name).as_str()),
            Cell::new(&profile.candidate_size.to_string()),
            Cell::new(&name_to_full_name(&profile.ballot_name)),
            Cell::new(&profile.num_rankings.to_string()),
        ]));
    }

    println!("{}", table);
}

pub fn list_profile(profile_name: &str) {
    let info = profile_name.split('_').collect::<Vec<&str>>();
    let m = info[0].parse::<usize>();
    if m.is_err() {
        eprintln!("Failed to parse name");
        std::process::exit(1);
    }
    let m = m.unwrap();
    let ballot_name = info[1];
    let ctx = Context::new(&Config::new());
    let ranking = get_profile(m, ballot_name, &ctx);
    for (i, (rank, _)) in ranking.votes.iter().enumerate() {
        println!("{i} - {}", rank.to_string());
    }
}
