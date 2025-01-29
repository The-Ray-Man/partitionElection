
# Partition Election  
This is an automated axiom-testing framework using Z3. It can verify whether an axiom holds for a voting rule in small election sizes.  

## CLI  
This project includes a command-line interface. After compilation, several commands are available:  

```shell
# Manage ballot types and generate a ranking database
partitionElection profile {...}
```

```shell
# Verify an axiom or multiple axioms for a given voting rule
partitionElection proof {...} 
```

```shell
# Check whether a scoring system exists that satisfies a set of axioms
partitionElection score {...} 
```

```shell
# Provide an overview of the available ballots, rules, and axioms
partitionElection overview    
```

## Rules  
Two rules are implemented: the Borda and Copeland rules. To add a new rule, follow these steps:  

1. In `src/proof/rule/new_rule.rs`, implement the TODOs. For the scoring function: The alternative with the highest score is elected.  
2. Rename the file and rule, and update all imports accordingly. The necessary imports are already present but commented out. Ensure you also add the new rule to the macro list in the `mod.rs` file.  

## Ballot Types  
Six ballot types are implemented. To add a new ballot, follow these steps:  

1. In `src/ballot/new_ballot.rs`, implement the TODOs. Do not change the function declarations, except for the new function, where you may modify the arguments.  
    - `fn generate_all_rankings(m: usize) -> BTreeSet<Ranking>`: This function must return all possible rankings for that ballot.  
2. Rename the file and ballot, and update all imports accordingly. The necessary imports are already present but commented out. Ensure you also add the new ballot to the macro list in the `mod.rs` file.  

## Axioms  
Axioms are more complex. Two types of axioms are supported:  

- **Forall Axioms**: The generated formula must hold for all possible numbers of votes.  
- **Exists Axioms**: The generated formula must hold for at least one number of votes.  

One function needs to be implemented: `condition_generator`. This function must return an iterator over formulas. The conditions that must hold for these formulas are:  

- **Forall Axioms**: The axiom holds if and only if the conjunction of all these formulas is satisfied for every profile.  
- **Exists Axioms**: The axiom holds if and only if, for each formula, there exists a profile in which it is satisfied.  

This is an automated axiom-testing framework using Z3. It can verify whether an axiom holds for a voting rule in small election sizes.  
