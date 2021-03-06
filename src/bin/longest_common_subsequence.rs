struct Solution;

impl Solution {
    pub fn index_of(bytes: &[u8], byte: u8) -> Option<usize> {
        for i in 0..bytes.len() {
            if bytes[i] == byte {
                return Some(i);
            }
        }
        None
    }

    pub fn helper_simpler(
        a: &[u8],
        b: &[u8],
        answers: &mut Vec<Vec<Option<i32>>>,
        i: usize,
        j: usize
    ) -> i32 {
        if i >= a.len() || j >= b.len() {
            return 0;
        }
        if answers[i][j].is_none() {
            let answer =
                if a[i] == b[j] {
                    1 + Solution::helper_simpler(a, b, answers, i+1, j+1)
                } else {
                    std::cmp::max(
                        Solution::helper_simpler(a, b, answers, i, j+1),
                        Solution::helper_simpler(a, b, answers, i+1, j)
                    )
                };
            answers[i][j] = Some(answer);
        }
        return answers[i][j].unwrap();
    }

    #[allow(unused)]
    pub fn helper(
        a: &[u8],
        b: &[u8],
        answers: &mut Vec<Vec<Option<i32>>>,
        i: usize,
        j: usize) -> i32 {
        if a.len() == 0 || b.len() == 0 {
            return 0;
        }
        if answers[i][j].is_none() {
            let in_len =
                if let Some(index) = Solution::index_of(b, a[0])
                { Solution::helper(&a[1..], &b[index..], answers, i + 1, j + index) + 1 } else { 0 };
            let out_len = Solution::helper(&a[1..], b, answers, i + 1, j);
            answers[i][j] = Some(std::cmp::max(in_len, out_len));
        }
        return answers[i][j].unwrap();
    }

    #[allow(unused)]
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let text1_bytes = text1.as_bytes();
        let text2_bytes = text2.as_bytes();
        let mut answers = vec![vec![None; text2_bytes.len()]; text1_bytes.len()];
        return
            Solution::helper_simpler(
                text1_bytes,
                text2_bytes,
                &mut answers,
                0, 0);
    }
}

#[cfg(test)]
pub mod tests {
    use crate::Solution;

    #[test]
    pub fn simple_test_001() {
        let input_a = String::from("");
        let input_b = String::from("");
        let expected = 0;
        let actual = Solution::longest_common_subsequence(input_a, input_b);
        assert_eq!(actual, expected);
    }

    #[test]
    pub fn simple_test_002() {
        let input_a = String::from("a");
        let input_b = String::from("");
        let expected = 0;
        let actual = Solution::longest_common_subsequence(input_a, input_b);
        assert_eq!(actual, expected);
    }

    #[test]
    pub fn simple_test_003() {
        let input_a = String::from("");
        let input_b = String::from("b");
        let expected = 0;
        let actual = Solution::longest_common_subsequence(input_a, input_b);
        assert_eq!(actual, expected);
    }

    #[test]
    pub fn simple_test_004() {
        let input_a = String::from("b");
        let input_b = String::from("b");
        let expected = 1;
        let actual = Solution::longest_common_subsequence(input_a, input_b);
        assert_eq!(actual, expected);
    }

    #[test]
    pub fn leetcode_test_001() {
        let input_a = String::from("abcde");
        let input_b = String::from("ace");
        let expected = 3;
        let actual = Solution::longest_common_subsequence(input_a, input_b);
        assert_eq!(actual, expected);
    }

    #[test]
    pub fn leetcode_test_002() {
        let input_a = String::from("abc");
        let input_b = String::from("abc");
        let expected = 3;
        let actual = Solution::longest_common_subsequence(input_a, input_b);
        assert_eq!(actual, expected);
    }

    #[test]
    pub fn leetcode_test_003() {
        let input_a = String::from("abc");
        let input_b = String::from("def");
        let expected = 0;
        let actual = Solution::longest_common_subsequence(input_a, input_b);
        assert_eq!(actual, expected);
    }
}

pub fn main() {
    println!("longest_common_subsequence");
}