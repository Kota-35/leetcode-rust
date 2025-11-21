#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        Self::check_all_substring(s)
    }

    fn check_all_substring(s: String) -> String {
        let chars: Vec<char> = s.chars().collect();

        let check = |i: usize, j: usize| -> bool {
            let mut left = i;
            let mut right = j - 1;

            while left < right {
                if chars[left] != chars[right] {
                    return false;
                }

                left += 1;
                right -= 1;
            }

            true
        };

        for length in (1..=chars.len()).rev() {
            for start in 0..(chars.len() - length + 1) {
                if check(start, start + length) {
                    return chars[start..(start + length)]
                        .iter()
                        .collect();
                }
            }
        }

        return String::new();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check<F: Fn(String) -> String>(f: F) {
        assert_eq!(f("babad".to_string()), "bab".to_string());
        assert_eq!(f("cbbd".to_string()), "bb".to_string());
    }

    #[test]
    fn test_answer() {
        check(|s| Solution::check_all_substring(s));
    }
}
