mod consistency;
mod fullcoalitionsupport;
mod majority;
mod nonimposition;
mod participation;
mod split;
mod strategyproof;
mod strongnonimposition;
mod strongpairsupport;
mod unanimity;
mod weakpairsupport;
// mod new_axiom;

pub use consistency::Consistency;
pub use fullcoalitionsupport::FullCoalitionSupport;
pub use majority::Majority;
pub use nonimposition::Nonimposition;
pub use participation::Participation;
pub use split::{StrictAllSplit, StrictSomeSplit, WeakAllSplit, WeakSomeSplit};
pub use strategyproof::Strategyproof;
pub use strongnonimposition::Strongnonimposition;
pub use strongpairsupport::StrongPairSupport;
pub use unanimity::Unanimity;
pub use weakpairsupport::WeakPairSupport;
// pub use new_axiom::MyAxiom;

use z3::SatResult;

use z3::ast::Bool;

use crate::proof::{profile::Profile, rule::VotingRule, Proof};

pub enum AxiomType {
    Forall,
    Exists,
}

pub trait Axiom {
    /// Creates the Z3 conditions for the axiom.
    /// If the axiom type is forall, then all conditions must be true for the axiom to hold.
    /// If the axiom type is exists, then every condition must be satisfiable for the axiom to hold.
    fn condition<'a: 'b, 'b>(
        profile: &'b Profile<'a>,
        rule: &'b Box<&'b (dyn VotingRule<'a> + 'b)>,
    ) -> Vec<Bool<'a>>;

    /// Returns the type of the axiom.
    fn get_type() -> AxiomType;

    /// Generates the conditions for the axiom as an iterator.
    fn condition_generator<'a: 'b, 'b>(
        profile: &'b Profile<'a>,
        rule: &'b Box<&'b (dyn VotingRule<'a> + 'b)>,
    ) -> impl Iterator<Item = Bool<'a>> + 'b;

    /// Returns the short name of the axiom.
    fn short_name() -> &'static str;

    /// Returns the full name of the axiom.
    fn full_name() -> &'static str;
}

macro_rules! case_distinction {
    ($func:ident, $proof:expr, [$($type:ty),*], $name:expr) => {
        $(
            if $name.eq_ignore_ascii_case(<$type>::short_name()) || $name.eq_ignore_ascii_case(<$type>::full_name()){
                return $proof.$func::<$type>();
            }

        )*
    }
}

macro_rules! all_names {
    ([$($axiom:ty),*]) => {

        pub fn all_axiom_names() -> Vec<(&'static str, &'static str)> {

            let mut all = vec![];
            $(
                all.push((<$axiom>::short_name(), <$axiom>::full_name()));
            )*

            return all
        }
    }
}

macro_rules! get_axiom_name {
    ([$($axiom:ty),*]) => {
        /// Returns the short name of an axiom given its name.
        pub fn get_axiom_short_name(name :&str) -> &str {

            $(
                if name.eq_ignore_ascii_case(<$axiom>::short_name()) || name.eq_ignore_ascii_case(<$axiom>::full_name()){
                    return <$axiom>::short_name();
                }
            )*

            eprintln!("Axiom {} not found", name);
            std::process::exit(1)
        }

        /// Returns the full name of an axiom given its name.
        pub fn get_axiom_full_name(name :&str) -> &str {

            $(
                if name.eq_ignore_ascii_case(<$axiom>::short_name()) || name.eq_ignore_ascii_case(<$axiom>::full_name()){
                    return <$axiom>::full_name();
                }
            )*

            eprintln!("Axiom {} not found", name);
            std::process::exit(1)
        }
    }
}

macro_rules! create_functions {
    ([$(($func:ident, $ret:ty)),*], $type:tt) => {

        all_names!($type);

        get_axiom_name!($type);

$(pub fn $func(axiom: &str, proof: &mut Proof) -> $ret {

    case_distinction!($func, proof, $type, axiom);

    eprintln!("Axiom {} not found", axiom);
    std::process::exit(1)
    }
)*
}
}

create_functions!(
    [(check_iteratively, SatResult), (add_axiom, ())],
    [
        Unanimity,
        Consistency,
        FullCoalitionSupport,
        Majority,
        Nonimposition,
        Participation,
        Strategyproof,
        Strongnonimposition,
        StrictSomeSplit,
        StrictAllSplit,
        WeakSomeSplit,
        WeakAllSplit,
        WeakPairSupport,
        StrongPairSupport
        // ,MyAxiom
    ]
);
