#[inline(always)]
pub fn levenshtein_distance(s1: &str, s2: &str) -> u32 {
    let n = s1.len();
    let m = s2.len();
    let mut d: Vec<Vec<u32>> = vec![vec![0; m + 1]; n + 1];

    if n == 0 {
        return m as u32;
    } else if m == 0 {
        return n as u32;
    }

    for i in 0..=n {
        for j in 0..=m {
            if i == 0 {
                d[i][j] = j as u32;
            } else if j == 0 {
                d[i][j] = i as u32;
            } else {
                d[i][j] = *[
                    d[i - 1][j] + 1, // deletion
                    d[i][j - 1] + 1, // insertion
                    d[i - 1][j - 1]
                        + (s1.chars().nth(i - 1).unwrap() != s2.chars().nth(j - 1).unwrap()) as u32, // substitution
                ]
                .iter()
                .min()
                .unwrap();
            }
        }
    }

    d[n][m]
}

#[inline(always)]
pub fn levenshtein_similarity_ratio(s1: &str, s2: &str) -> f64 {
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
