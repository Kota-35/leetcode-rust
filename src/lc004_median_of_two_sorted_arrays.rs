#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find_median_sorted_arrays(
        nums1: Vec<i32>,
        nums2: Vec<i32>,
    ) -> f64 {
        Solution::answer(nums1, nums2)
    }

    fn answer(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let mut merged_array = [nums1, nums2].concat();

        merged_array.sort_unstable();

        let len = merged_array.len();
        if len == 0 {
            return 0.0;
        }

        if len % 2 == 1 {
            merged_array[len / 2] as f64
        } else {
            let mid1 = merged_array[len / 2 - 1] as f64;
            let mid2 = merged_array[len / 2] as f64;

            (mid1 + mid2) / 2.0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check<F: Fn(Vec<i32>, Vec<i32>) -> f64>(f: F) {
        assert_eq!(f(vec![1, 3], vec![2]), 2.00000);
        assert_eq!(f(vec![1, 2], vec![3, 4]), 2.50000);
    }

    #[test]
    fn test_answer() {
        check(|nums1, nums2| Solution::answer(nums1, nums2));
    }
}
