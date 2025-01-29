use crate::structures::{Candidate, Coalition, Partition, Ranking};

impl ToString for Candidate {
    fn to_string(&self) -> String {
        self.name.clone()
    }
}

impl ToString for Coalition {
    fn to_string(&self) -> String {
        let result = self
            .members
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(", ");
        format!("{{{}}}", result)
    }
}

impl ToString for Partition {
    fn to_string(&self) -> String {
        let result = self
            .coalitions
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(", ");
        format!("{{{}}}", result)
    }
}

impl ToString for Ranking {
    fn to_string(&self) -> String {
        let result = self
            .ranking
            .iter()
            .map(|x| {
                x.iter()
                    .map(|y| y.to_string())
                    .collect::<Vec<String>>()
                    .join(" ~ ")
            })
            .collect::<Vec<String>>()
            .join(" > ");
        result.to_string()
    }
}
