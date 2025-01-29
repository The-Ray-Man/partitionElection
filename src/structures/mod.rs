pub mod candidate;
pub mod coalition;
pub mod partition;
pub mod ranking;
mod utils;

pub use candidate::Candidate;
pub use coalition::Coalition;
pub use partition::Partition;
pub use ranking::Ranking;

use std::collections::BTreeSet;

pub trait Structure {
    /// Returns all possible structures of this type and size ```m```.
    fn all(m: usize) -> BTreeSet<Self>
    where
        Self: Sized;

    /// Checks if `self` is a legal structure.
    /// For example: A ranking is legal if it contains all partitions
    fn is_legal(&self, m: usize) -> bool
    where
        Self: Sized;
}
