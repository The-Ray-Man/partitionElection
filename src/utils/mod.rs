pub mod io;
pub mod structures;
pub mod unordered_pair;

/// Returns the Bell number for a given number of candidates.
/// The Bell number is the number of partitions of a set.
pub fn bell(m: usize) -> usize {
    let mut bell = vec![vec![0; m]; m];
    bell[0][0] = 1;
    for i in 1..m {
        bell[i][0] = bell[i - 1][i - 1];
        for j in 1..=i {
            bell[i][j] = bell[i][j - 1] + bell[i - 1][j - 1];
        }
    }
    bell[m - 1][m - 1]
}
