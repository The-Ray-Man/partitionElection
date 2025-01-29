#[cfg(test)]
mod tests {
    use partitionElection::ballots::{Ballot, Fp};
    use partitionElection::ballots::{Fcs, Fcw, Pa};

    #[test]
    fn test_pa() {
        let m = 3;
        let num_partitions = 5;

        let rankings = Pa::all_rankings(m);

        assert_eq!(rankings.len(), (2usize).pow(num_partitions as u32) - 1);
    }

    #[test]
    fn test_fcs() {
        let m = 3;

        let rankings = Fcs::all_rankings(m);

        assert_eq!(rankings.len(), (2usize).pow(m as u32) - 1);
    }

    #[test]
    fn test_fcw() {
        let m = 3;

        let rankings = Fcw::all_rankings(m);

        assert_eq!(rankings.len(), 5);
    }

    #[test]
    fn test_fp() {
        let m = 3;

        let rankings = Fp::all_rankings(m);

        assert_eq!(rankings.len(), 5);
    }

    #[test]
    fn test_expressive() {
        let m = 3;
        let rankings_pa = Pa::all_rankings(m);
        let rankings_fcs = Fcs::all_rankings(m);
        let rankings_fcw = Fcw::all_rankings(m);
        let rankings_fp = Fp::all_rankings(m);

        assert!(rankings_pa.is_superset(&rankings_fcs));
        assert!(rankings_pa.is_superset(&rankings_fcw));
        assert!(rankings_fcs.intersection(&rankings_fcw).count() > 0);
        assert!(!rankings_fcs.is_subset(&rankings_fcw));
        assert!(!rankings_fcw.is_subset(&rankings_fcs));
        assert!(!rankings_pa.is_subset(&rankings_fp));
        assert!(!rankings_fp.is_subset(&rankings_pa));
    }
}
