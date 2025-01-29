use std::collections::BTreeSet;

use super::Structure;

#[derive(Eq, Hash, Ord, PartialEq, PartialOrd, Clone, Debug)]
pub struct Candidate {
    pub name: String,
}

impl Structure for Candidate {
    fn all(m: usize) -> BTreeSet<Self>
    where
        Self: Sized,
    {
        let alphabet: Vec<char> = ('a'..='z').collect();

        let mut result: BTreeSet<Self> = BTreeSet::new();

        for mut i in 0..m {
            let mut name = String::new();
            loop {
                let index = i % 26;
                name = format!("{}{}", alphabet[index], name);
                if i < 26 {
                    break;
                }
                i = (i - index) / 26;
            }
            result.insert(Self { name });
        }
        result
    }

    fn is_legal(&self, _m: usize) -> bool
    where
        Self: Sized,
    {
        self.name.chars().all(|x| x.is_ascii_alphanumeric())
    }
}
