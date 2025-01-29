use z3::{
    ast::{exists_const, forall_const, Ast, Bool},
    Model, SatResult, Solver,
};

use crate::proof::{
    axiom::{Axiom, AxiomType},
    profile::Profile,
    rule::VotingRule,
};

#[derive(Debug)]
pub enum ProofResult {
    Sat(Vec<String>),
    Unsat(Vec<String>),
    Unknown,
}

pub struct Proof<'ctx> {
    profile: Profile<'ctx>,
    forall_conditions: Vec<Bool<'ctx>>,
    exists_conditions: Vec<Bool<'ctx>>,
    rule: Box<&'ctx dyn VotingRule<'ctx>>,
}

impl<'ctx> Proof<'ctx> {
    /// Creates a new proof with the given profile and rule.
    pub fn new(_m: usize, rule: &'ctx dyn VotingRule<'ctx>, profile: Profile<'ctx>) -> Self {
        let forall_conditions: Vec<Bool<'ctx>> = Vec::new();
        let exists_conditions: Vec<Bool<'ctx>> = Vec::new();
        let rule = Box::new(rule);
        Proof {
            profile,
            forall_conditions,
            exists_conditions,
            rule,
        }
    }

    /// Adds an axiom to the proof.
    pub fn add_axiom<A: Axiom>(&mut self) {
        let condition = A::condition(&self.profile, &self.rule);
        let ax_type = A::get_type();
        match ax_type {
            AxiomType::Forall => self.forall_conditions.extend(condition),
            AxiomType::Exists => self.exists_conditions.extend(condition),
        }
    }

    /// Returns the number of votes per ranking given the model.
    fn get_profile(&self, model: &Model) -> Vec<String> {
        self.profile
            .votes
            .iter()
            .filter_map(|(ranking, var)| {
                let ranking = ranking.to_string();
                let val = model.get_const_interp(var);
                match val {
                    None => Some(format!("? -> {}", ranking)),
                    Some(val) => {
                        if val.as_i64().unwrap() == 0 {
                            None
                        } else {
                            Some(format!("{} -> {}", val, ranking))
                        }
                    }
                }
            })
            .collect::<Vec<_>>()
    }

    /// Prints the profile given the model.
    fn print_profile(&self, model: &Model) {
        let ranking = self.get_profile(model);
        ranking.iter().for_each(|ranking| {
            log::warn!("{}", ranking.to_string());
        });
    }

    /// Searches for a witness that satisfies the axiom.
    fn check_iteratively_exists_axiom<A: Axiom>(&mut self) -> SatResult {
        log::info!("Start checking Axiom {}", A::full_name());
        let ctx = self.profile.get_ctx();
        let non_negative = self.profile.vars_nonnegative();
        let sum_positive = self.profile.vars_sum_positive();

        for condition in A::condition_generator(&self.profile, &self.rule) {
            let solver = Solver::new(ctx);
            solver.assert(&condition);
            solver.assert(&non_negative);
            solver.assert(&sum_positive);
            let result = solver.check();
            match result {
                SatResult::Sat => {
                    log::info!("Sat with witness:");
                    let model = solver.get_model();
                    match model {
                        None => log::info!("Failed to get Model"),
                        Some(model) => self.print_profile(&model),
                    }
                }
                SatResult::Unknown => {
                    log::warn!("Unknown");
                    return SatResult::Unknown;
                }
                SatResult::Unsat => {
                    log::warn!("Unsat - Could not find a witness");
                    return SatResult::Unsat;
                }
            }
        }
        SatResult::Sat
    }

    /// Searches for a counterexample that falsifies the axiom.
    fn check_iteratively_forall_axiom<A: Axiom>(&mut self) -> SatResult {
        log::warn!("Start checking Axiom {}", A::full_name());
        let ctx = self.profile.get_ctx();
        let non_negative = self.profile.vars_nonnegative();
        let sum_positive = self.profile.vars_sum_positive();

        for condition in A::condition_generator(&self.profile, &self.rule) {
            let solver = Solver::new(ctx);
            solver.assert(&condition.not());
            solver.assert(&non_negative);
            solver.assert(&sum_positive);
            let result = solver.check();
            match result {
                SatResult::Sat => {
                    log::warn!("Unsat, Counterexample:");
                    let model = solver.get_model();
                    match model {
                        None => log::error!("Failed to get Model"),
                        Some(model) => self.print_profile(&model),
                    }
                    return SatResult::Unsat;
                }
                SatResult::Unknown => return SatResult::Unknown,
                _ => {}
            }
        }
        SatResult::Sat
    }

    /// Checks if the profile satisfies this axiom. An early termination is possible if the axiom is not satisfied.
    pub fn check_iteratively<A: Axiom>(&mut self) -> SatResult {
        let axiom_type = A::get_type();
        match axiom_type {
            AxiomType::Forall => self.check_iteratively_forall_axiom::<A>(),
            AxiomType::Exists => self.check_iteratively_exists_axiom::<A>(),
        }
    }

    /// Checks if the profile satisfies the axiom previously added.
    pub fn check(&mut self) -> (SatResult, Option<Model>) {
        let ctx = self.profile.get_ctx();
        let solver = Solver::new(ctx);
        let all_forall_conditions = self.forall_conditions.iter().collect::<Vec<_>>();
        let all_forall_conditions: Bool<'_> = Bool::and(ctx, &all_forall_conditions);

        let vars = self
            .profile
            .all_vars()
            .into_iter()
            .map(|v| v as &dyn Ast)
            .collect::<Vec<_>>();

        let non_negative = self.profile.vars_nonnegative();
        let sum_positive = self.profile.vars_sum_positive();
        let preconditions = Bool::and(ctx, &[&non_negative, &sum_positive]);
        let formula = preconditions.implies(&all_forall_conditions);

        let forall_quantor = forall_const(ctx, &vars, &[], &formula);

        let exists_formulas = self
            .exists_conditions
            .iter()
            .map(|formula| {
                let formula_with_assumptions =
                    Bool::and(ctx, &[&non_negative, &sum_positive, formula]);
                exists_const(ctx, &vars, &[], &formula_with_assumptions)
            })
            .collect::<Vec<_>>();

        solver.assert(&forall_quantor);
        for exist_formula in exists_formulas {
            solver.assert(&exist_formula);
        }
        log::info!("Start Checking");
        let result = solver.check();

        (result, solver.get_model())
    }
}
