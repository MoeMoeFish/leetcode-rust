/**
 * solution1 的复习
 */
use std::cmp;

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let len1 = nums1.len();
        let len2 = nums2.len();
        let total_len = len1 + len2;

        let ret1 = Solution::find_by_leading_index(&nums1, 0, len1, &nums2, 0, len2, (total_len - 1) / 2);
        let ret2 = Solution::find_by_leading_index(&nums1, 0, len1, &nums2, 0, len2, total_len / 2);

        let ret = (ret1 + ret2) as f64 / 2.0;

        return ret;
    }

    /*
     * index 以 0 开始
     * start, end 前闭后开
     */
    pub fn find_leading_index(nums1: &Vec<i32>, start1: usize, end1: usize, nums2: &Vec<i32>, start2: usize, end2: usize, k: usize) -> i32 {
        let len1 = end1 - start1;
    }
}
