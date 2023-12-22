//! This module provides functions for approximate string matching.

/// Calculates the Levenshtein distance between two strings.
///
/// The Levenshtein distance is a measure of the difference between two strings.
/// It is defined as the minimum number of single-character edits (insertions, deletions, or substitutions)
/// required to change one string into the other. This function is designed to be used with
/// the ASCII-only strings in the siz crate.
///
/// # Arguments
///
/// * `s1` - The first string.
/// * `s2` - The second string.
///
/// # Returns
///
/// The Levenshtein distance between `s1` and `s2`.
///
/// # Examples
///
/// ```
/// use siz::fuzzy::levenshtein_distance;
///
/// let distance = levenshtein_distance("kitten", "sitting");
/// assert_eq!(distance, 3);
/// ```
#[inline(always)]
pub fn levenshtein_distance(s1: &str, s2: &str) -> usize {
    let s1_chars: Vec<char> = s1.chars().collect();
    let s2_chars: Vec<char> = s2.chars().collect();
    let len_s1 = s1_chars.len();
    let len_s2 = s2_chars.len();
    if len_s1 == 0 {
        return len_s2;
    }
    if len_s2 == 0 {
        return len_s1;
    }
    let mut d: [Vec<usize>; 2] = [vec![0; len_s2 + 1], vec![0; len_s2 + 1]];
    for j in 0..=len_s2 {
        d[0][j] = j;
    }
    for i in 1..=len_s1 {
        d[i % 2][0] = i;
        for j in 1..=len_s2 {
            let cost = if s1_chars[i - 1] != s2_chars[j - 1] {
                1
            } else {
                0
            };
            d[i % 2][j] = std::cmp::min(
                std::cmp::min(d[(i - 1) % 2][j] + 1, d[i % 2][j - 1] + 1),
                d[(i - 1) % 2][j - 1] + cost,
            );
        }
    }
    d[len_s1 % 2][len_s2]
}

///
/// The Levenshtein similarity ratio is a measure of the similarity between two strings.
/// It is defined as the ratio of the length of the combined strings minus the Levenshtein distance,
/// divided by the length of the combined strings.
///
/// # Arguments
///
/// * `s1` - The first string.
/// * `s2` - The second string.
///
/// # Returns
///
/// The Levenshtein similarity ratio between `s1` and `s2`.
///
/// # Examples
///
/// ```
/// use siz::fuzzy::levenshtein_similarity_ratio;
///
/// let similarity = levenshtein_similarity_ratio("kitten", "sitting");
/// assert_eq!(similarity, 0.7692307692307693);
/// ```
#[inline(always)]
pub fn levenshtein_similarity_ratio(s1: &str, s2: &str) -> f64 {
    if s1 == s2 {
        return 1.0;
    }

    let distance = levenshtein_distance(s1, s2);
    let len_sum = s1.len() + s2.len();

    (len_sum as f64 - distance as f64) / len_sum as f64
}

#[cfg(test)]
mod tests {
    use super::*;

    use approx::assert_abs_diff_eq;

    #[test]
    fn test_levenshtein_distance_same() {
        assert_eq!(levenshtein_distance("xyz", "xyz"), 0);
    }

    #[test]
    fn test_levenshtein_distance_zero_length_left() {
        assert_eq!(levenshtein_distance("", "xyz"), 3);
    }

    #[test]
    fn test_levenshtein_distance_zero_length_right() {
        assert_eq!(levenshtein_distance("xyz", ""), 3);
    }

    #[test]
    fn test_levenshtein_distance_insertion() {
        assert_eq!(levenshtein_distance("xyz", "xyza"), 1);
    }

    #[test]
    fn test_levenshtein_distance_deletion() {
        assert_eq!(levenshtein_distance("xyz", "xy"), 1);
    }

    #[test]
    fn test_levenshtein_distance_substitution() {
        assert_eq!(levenshtein_distance("xyz", "xzz"), 1);
    }

    #[test]
    fn test_levenshtein_distance_sitting_kitten() {
        assert_eq!(levenshtein_distance("sitting", "kitten"), 3);
    }

    #[test]
    fn test_similarity_ratio_same() {
        assert_abs_diff_eq!(levenshtein_similarity_ratio("xyz", "xyz"), 1.0);
    }

    #[test]
    fn test_similarity_ratio_zero_length_left() {
        assert_abs_diff_eq!(levenshtein_similarity_ratio("", "xyz"), 0.0);
    }

    #[test]
    fn test_similarity_ratio_zero_length_right() {
        assert_abs_diff_eq!(levenshtein_similarity_ratio("xyz", ""), 0.0);
    }

    #[test]
    fn test_similarity_ratio_insertion() {
        assert_abs_diff_eq!(
            levenshtein_similarity_ratio("xyz", "xyza"),
            0.8571428571428571
        );
    }

    #[test]
    fn test_similarity_ratio_insertion_reversed() {
        assert_abs_diff_eq!(
            levenshtein_similarity_ratio("xyza", "xyz"),
            0.8571428571428571
        );
    }

    #[test]
    fn test_similarity_ratio_deletion() {
        assert_abs_diff_eq!(levenshtein_similarity_ratio("xyz", "xy"), 0.8);
    }

    #[test]
    fn test_similarity_ratio_deletion_reversed() {
        assert_abs_diff_eq!(levenshtein_similarity_ratio("xy", "xyz"), 0.8);
    }

    #[test]
    fn test_similarity_ratio_substitution() {
        assert_abs_diff_eq!(
            levenshtein_similarity_ratio("xyz", "xzz"),
            0.8333333333333334
        );
    }

    #[test]
    fn test_similarity_ratio_substitution_reversed() {
        assert_abs_diff_eq!(
            levenshtein_similarity_ratio("xzz", "xyz"),
            0.8333333333333334
        );
    }

    #[test]
    fn test_similarity_ratio_sitting_kitten() {
        assert_abs_diff_eq!(
            levenshtein_similarity_ratio("sitting", "kitten"),
            0.7692307692307693
        );
    }

    #[test]
    fn test_similarity_ratio_sitting_kitten_reversed() {
        assert_abs_diff_eq!(
            levenshtein_similarity_ratio("kitten", "sitting"),
            0.7692307692307693
        );
    }
}
