use std::hash::Hash;

#[derive(Eq, Clone, Hash, PartialOrd, Ord)]
pub struct UnorderedPair<T>(T, T);

impl<T: PartialEq + Ord> UnorderedPair<T> {
    pub fn new(a: T, b: T) -> Self {
        // Always store in a consistent order
        if a <= b {
            UnorderedPair(a, b)
        } else {
            UnorderedPair(b, a)
        }
    }
}

impl<T: PartialEq> PartialEq for UnorderedPair<T> {
    fn eq(&self, other: &Self) -> bool {
        (self.0 == other.0 && self.1 == other.1) || (self.0 == other.1 && self.1 == other.0)
    }
}
