use std::collections::BTreeSet;

/// Returns the power set of a set.
pub fn powerset<T>(s: &BTreeSet<T>) -> BTreeSet<BTreeSet<&T>>
where
    T: Clone + Ord,
{
    powerset_generator(s).collect()
}

/// Returns an iterator over the power set of a set.
pub fn powerset_generator<T>(s: &BTreeSet<T>) -> impl Iterator<Item = BTreeSet<&T>>
where
    T: Clone + Ord,
{
    (0..2usize.pow(s.len() as u32)).map(move |i| {
        s.iter()
            .enumerate()
            .filter(move |&(t, _)| (i >> t) % 2 == 1)
            .map(move |(_, element)| element)
            .collect()
    })
}

/// Returns all possible partitions over a set.
pub fn partition<T>(s: &mut BTreeSet<T>) -> BTreeSet<BTreeSet<BTreeSet<T>>>
where
    T: Clone + Ord,
{
    if s.len() == 1 {
        let mut inner_set = BTreeSet::new();
        let element = s.pop_first().unwrap(); // This is safe because we know that the set is not empty
        inner_set.insert(element);
        let mut outer_set = BTreeSet::new();
        outer_set.insert(inner_set);
        let mut result = BTreeSet::new();
        result.insert(outer_set);
        return result;
    }

    let current_element = s.pop_first().unwrap(); // This is safe because we know that the set is not empty

    let mut singelton_partition = BTreeSet::new();
    singelton_partition.insert(current_element.clone());

    let small_partition = partition(s);

    let mut result = BTreeSet::new();
    for partition in &small_partition {
        let mut new_partition = partition.clone();
        new_partition.insert(singelton_partition.clone());
        result.insert(new_partition);

        for set in partition.clone() {
            let mut new_partition = partition.clone();
            new_partition.remove(&set);
            let mut new_set = set.clone();
            new_set.insert(current_element.clone());
            new_partition.insert(new_set);
            result.insert(new_partition);
        }
    }
    result
}

/// Returns all possible partitions over a set with a given number of coalitions.
pub fn divide_into_classes<T>(s: &BTreeSet<T>, m: usize) -> BTreeSet<BTreeSet<BTreeSet<T>>>
where
    T: Clone + Ord,
{
    let result: BTreeSet<BTreeSet<BTreeSet<T>>> = s.iter().fold(BTreeSet::new(), |mut acc, x| {
        // acc : set of possible partitions
        if acc.is_empty() {
            let mut class = BTreeSet::new();
            class.insert(x.clone());

            let mut partition = BTreeSet::new();
            partition.insert(class);
            acc.insert(partition);
            acc
        } else {
            let mut new_partitions = BTreeSet::new();
            for classes in &acc {
                if classes.len() < m {
                    let mut classes = classes.clone();
                    let mut new_class = BTreeSet::new();
                    new_class.insert(x.clone());
                    classes.insert(new_class);
                    new_partitions.insert(classes);
                }

                for class in classes {
                    let mut new_classes = classes.clone();
                    new_classes.remove(class);
                    let mut new_class = class.clone();
                    new_class.insert(x.clone());
                    new_classes.insert(new_class);
                    new_partitions.insert(new_classes);
                }
            }
            new_partitions
        }
    });
    result.into_iter().filter(|x| x.len() == m).collect()
}
