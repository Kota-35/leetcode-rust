use std::collections::HashMap;

#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        Self::answer(nums, target)
    }

    fn answer(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut result: Vec<i32> = vec![];

        for i in 0..(nums.len() - 1) {
            for j in (i + 1)..(nums.len()) {
                if nums[i] + nums[j] == target {
                    result.push(i as i32);
                    result.push(j as i32);
                }
            }
        }

        result
    }

    /// **Approach 1: Brute Force**
    ///
    /// `nums`の要素`x`を全て探索し、 `target - x`と等しいものを見つける
    /// O(n^2)
    fn two_sum_brute_force(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for i in 0..(nums.len()) {
            for j in (i + 1)..nums.len() {
                if nums[j] == target - nums[i] {
                    return vec![i as i32, j as i32];
                }
            }
        }

        vec![]
    }

    /// **Approach 2: Two-pass Hash Table**
    ///
    ///
    fn two_sum_two_pass_hash_table(
        nums: Vec<i32>,
        target: i32,
    ) -> Vec<i32> {
        let mut hash: HashMap<i32, i32> = HashMap::new();

        // keyに num を value にそのindexを代入
        for i in 0..nums.len() {
            hash.insert(nums[i], i as i32);
        }

        for i in 0..nums.len() {
            let complement = target - nums[i];

            match hash.get(&complement) {
                Some(value) if *value != i as i32 => {
                    return vec![i as i32, *value];
                }
                _ => continue,
            }
        }

        vec![]
    }

    /// *Approach 3: One-pass Hash Table*
    ///
    fn two_sum_one_pass_hash_table(
        nums: Vec<i32>,
        target: i32,
    ) -> Vec<i32> {
        let mut hash: HashMap<i32, i32> = HashMap::new();

        for i in 0..nums.len() {
            let complement = target - nums[i];
            if let Some(value) = hash.get(&complement) {
                return vec![*value, i as i32];
            };
            hash.insert(nums[i], i as i32);
        }

        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    fn check<F: Fn(Vec<i32>, i32) -> Vec<i32>>(f: F) {
        assert_eq!(f(vec![2, 7, 11, 15], 9), vec![0, 1]);
        assert_eq!(f(vec![3, 2, 4], 6), vec![1, 2]);
        assert_eq!(f(vec![3, 3], 6), vec![0, 1]);
    }

    #[test]
    fn test_answer() {
        check(|nums, target| Solution::answer(nums, target));
    }

    #[test]
    fn test_brute_force() {
        check(|nums, target| Solution::two_sum_brute_force(nums, target));
    }

    #[test]
    fn test_two_pass_hash_table() {
        check(|nums, target| {
            Solution::two_sum_two_pass_hash_table(nums, target)
        });
    }

    #[test]
    fn test_one_pass_hash_table() {
        check(|nums, target| {
            Solution::two_sum_one_pass_hash_table(nums, target)
        });
    }
}
