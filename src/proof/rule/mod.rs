mod borda;
mod copeland;
mod rule_trait;
mod scoring;
// mod new_rule;
pub use borda::Borda;
pub use copeland::Copeland;
pub use rule_trait::VotingRule;
pub use scoring::Scoring;
// pub use new_rule::MyRule;

macro_rules! get_rule {
    ([$($rule:ty),*]) => {

        pub fn all_rule_names() -> Vec<&'static str> {

            let mut all = vec![];
            $(
                all.push(<$rule>::name());
            )*

            return all
        }

        pub fn get_rule_name(rule: &str) -> String {
            let lowercase = rule.to_ascii_lowercase();
            let name = lowercase.as_str();
            let mut _obj: Box<dyn VotingRule>;
            $(if name.eq_ignore_ascii_case(<$rule>::name()) {
                return <$rule>::name().to_string();
            }
             )*
            eprintln!("Rule {} not found", name);
            std::process::exit(1)

        }


        pub fn get_rule<'a :'b, 'b>(m: usize, rule: &'b str) -> Box<dyn VotingRule<'a> + 'b> {
            let lowercase = rule.to_ascii_lowercase();
            let name = lowercase.as_str();
            let obj: Box<dyn VotingRule>;
            $(if name == <$rule>::name() {
                obj = Box::new(<$rule>::new(m));
                return obj;
            }
             )*
            eprintln!("Rule {} not found", name);
            std::process::exit(1)

        }
    };
}

get_rule!([
    Borda,
    Copeland
    // ,MyRule
    ]);
