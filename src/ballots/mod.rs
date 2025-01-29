pub(crate) mod ballot_trait;

mod cs;
mod fcs;
mod fcw;
mod fp;
// mod new_ballot;
mod pa;
mod ps;
mod utils;

pub use ballot_trait::Ballot;
pub use cs::Cs;
pub use fcs::Fcs;
pub use fcw::Fcw;
pub use fp::Fp;
// pub use new_ballot::MyBallot;
pub use pa::Pa;
pub use ps::Ps;

use crate::proof::profile::Profile;
use std::path::Path;
use z3::Context;

macro_rules! create_profile {
    ([$($ballot:ty),*]) => {

        /// Returns a list of all ballot names.
        pub fn all_ballot_names() -> Vec<(String, String)> {

            let mut all = vec![];
            $(
                all.push((<$ballot>::get_name(), <$ballot>::get_full_name()));
            )*

            return all
        }

        /// Returns the full name of a ballot given its name.
        pub fn name_to_full_name(name: &str) -> String {

            $(if name.eq_ignore_ascii_case(&<$ballot>::get_name()) {
                return <$ballot>::get_full_name();
            })*

            eprintln!("Ballot {} not found", name);
            std::process::exit(1)
        }

        /// Returns the profile of a ballot given its name.
        pub fn get_profile<'a>(m: usize, ballot: &str, ctx: &'a Context) -> Profile<'a> {
            $(if ballot.eq_ignore_ascii_case(&<$ballot>::get_name()) || ballot.eq_ignore_ascii_case(&<$ballot>::get_full_name()) {
                return Profile::from_ballot::<$ballot>(m, ctx)
            })*

            eprintln!("Ballot {} not found", ballot);
            std::process::exit(1)
        }


        /// Creates a profile file for a given ballot.
        pub fn create_profile_file(m: usize, ballot: &str) -> () {
            let mut ballot_name : String = "".to_string();
            $(if ballot.eq_ignore_ascii_case(&<$ballot>::get_name()) || ballot.eq_ignore_ascii_case(&<$ballot>::get_full_name()) {
                ballot_name  = <$ballot>::get_name()
            })*

            if ballot_name.is_empty() {
                eprintln!("Ballot {} not found", ballot_name);
            }

            let filename = format!("{}_{}", m, ballot_name);
            let path = format!("logs/rankings/{}.txt", filename);
            if Path::new(&path).exists() {
                eprintln!("Profile already exists");
                return;
            }

            $(if ballot.eq_ignore_ascii_case(&<$ballot>::get_name()) || ballot.eq_ignore_ascii_case(&<$ballot>::get_full_name()) {
                <$ballot>::generate_ranking_file(m)
            })*
        }
    };
}

create_profile!([
    Cs,
    Fcs,
    Fcw,
    Fp,
    Pa,
    Ps
    // , MyBallot
]);
