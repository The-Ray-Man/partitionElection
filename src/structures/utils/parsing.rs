use std::{collections::BTreeSet, str::FromStr};

use regex::Regex;

use crate::structures::{Candidate, Coalition, Partition, Ranking};

impl FromStr for Candidate {
    type Err = std::string::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            name: s.trim().to_string(),
        })
    }
}

impl FromStr for Coalition {
    type Err = std::string::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let members = s
            .trim()
            .trim_start_matches('{')
            .trim_end_matches('}')
            .split(", ")
            .map(|x| Candidate::from_str(x).unwrap())
            .collect::<BTreeSet<_>>();
        Ok(Self { members })
    }
}

impl FromStr for Partition {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.trim().chars();
        let re = Regex::new(r"\{[a-zA-Z, ]*\}").unwrap(); // Save, the regex is correct
        chars.next();
        chars.next_back();
        let coalitions = re
            .captures_iter(chars.as_str())
            .map(|capture| {
                let coalition_string = capture.get(0).unwrap().as_str();
                Coalition::from_str(coalition_string).unwrap() // Safe, Can never happen
            })
            .collect::<Vec<_>>();

        Ok(Self {
            coalitions: coalitions.into_iter().collect(),
        })
    }
}

impl FromStr for Ranking {
    type Err = std::string::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ranking = s
            .trim()
            .split('>')
            .map(|x| {
                if x.is_empty() {
                    BTreeSet::new()
                } else {
                    x.split('~')
                        .map(|y| Partition::from_str(y).unwrap())
                        .collect::<BTreeSet<_>>()
                }
            })
            .collect::<Vec<_>>();
        Ok(Self { ranking })
    }
}
