#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn length_of_strong_substring(s: String) -> i32 {
        Solution::sliding_window(s)
    }

    fn sliding_window(s: String) -> i32 {
        let mut max_length = 0;
        let mut last_index = [0; 128];
        let mut start = 0;

        for (end, ch) in s.chars().enumerate() {
            start = start.max(last_index[ch as usize]);
            max_length = max_length.max(end - start + 1);
            last_index[ch as usize] = end + 1;
        }

        max_length as i32
    }
}

#[cfg(test)]
mod test {

    use super::*;

    fn check<F: Fn(String) -> i32>(f: F) {
        assert_eq!(f("abcabcbb".to_string()), 3);
        assert_eq!(f("bbbbb".to_string()), 1);
        assert_eq!(f("pwwkew".to_string()), 3);
    }

    #[test]
    fn test_answer() {
        check(|s| Solution::sliding_window(s));
    }
}
