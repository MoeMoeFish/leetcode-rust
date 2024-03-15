/**
 * solution1 的复习
 */
use std::cmp;

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let len1 = nums1.len();
        let len2 = nums2.len();
        let total_len = len1 + len2;

        let ret1 = Solution::find_leading_index(&nums1, 0, len1, &nums2, 0, len2, (total_len + 1) / 2);
        let ret2 = Solution::find_leading_index(&nums1, 0, len1, &nums2, 0, len2, (total_len + 2) / 2);

        let ret = (ret1 + ret2) as f64 / 2.0;

        return ret;
    }

    /*
     * start, end 前闭后开
     * k 表示从 1 开始的位数, k = 2 表示找到第 2 个元素 arr[1]
     */
    pub fn find_leading_index(nums1: &Vec<i32>, start1: usize, end1: usize, nums2: &Vec<i32>, start2: usize, end2: usize, k: usize) -> i32 {
        let len1 = end1 - start1;
        let len2 = end2 - start2;

        if len1 < len2 {
            return Solution::find_leading_index(nums2, start2, end2, nums1, start1, end1, k);
        }

        if len2 == 0 {
            return nums1[start1 + k - 1];
        }

        if k == 1 {
            return cmp::min(nums1[start1], nums2[start2]);
        }

        let removed_count = cmp::min(k / 2, len2) ;

        if nums1[start1 + removed_count - 1] < nums2[start2 + removed_count - 1] {
            return Solution::find_leading_index(nums1, start1 + removed_count, end1, nums2, start2, end2, k - removed_count);
        }
         
        Solution::find_leading_index(nums1, start1, end1, nums2, start2 + removed_count, end2, k - removed_count)
    }
}

pub struct Solution {}
