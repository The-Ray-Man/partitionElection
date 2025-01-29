#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use partitionElection::utils::structures::{divide_into_classes, partition};

    #[test]
    fn test_partitions() {
        let m = 3;
        let mut vec = (0..m).collect::<BTreeSet<_>>();
        let partitions = partition(&mut vec);
        assert!(partitions.len() == 5);

        let m = 4;
        let mut vec = (0..m).collect::<BTreeSet<_>>();
        let partitions = partition(&mut vec);
        assert!(partitions.len() == 15);
    }

    #[test]
    fn test_partitions_fixed_size() {
        let m = 3;
        let size = 1;

        let mut vec = (0..m).collect::<BTreeSet<_>>();
        let partitions = divide_into_classes(&mut vec, size);
        assert!(partitions.len() == 1);

        let m = 4;
        let mut vec = (0..m).collect::<BTreeSet<_>>();
        let partitions = divide_into_classes(&mut vec, 2);

        assert!(partitions.len() == 7);
    }
}
