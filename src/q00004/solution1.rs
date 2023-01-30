/*
 * @Author: MoeMoeFish moemoefish@qq.com
 * @Date: 2023-01-29 13:45:52
 * @LastEditors: MoeMoeFish moemoefish@qq.com
 * @LastEditTime: 2023-01-30 13:36:51
 * @Description: 00004 solution1
 */

use std::cmp;
pub struct Solution {}

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let len1 = nums1.len();
        let len2 = nums2.len();
        let len = len1 + len2;

        let ret1 = Solution::find_by_leading_index(&nums1, &nums2, 0, len1 - 1, 0, len2, (len / 2) + 1);
        let ret2 = Solution::find_by_leading_index(&nums1, &nums2, 0, len1 - 1, 0, len2, (len + 1) / 2);
        let ret = (ret1 + ret2) as f64 / 2.0;

        ret
    }

    fn find_by_leading_index(nums1: &Vec<i32>, nums2: &Vec<i32>, start1: usize, end1: usize, start2: usize, end2: usize, pos: usize) -> i32 {
        let len1 = end1 - start1 + 1;
        let len2 = end2 - start2 + 1;

        if len1 > len2 {
            return Solution::find_by_leading_index(nums2, nums1, start2, end2, start1, end1, pos);
        }

        if pos == 1 {
            return cmp::min(nums1[start1], nums2[start2]);
        }

        let k = pos / 2;

        if len1 < start1 + k - 1 {
            return nums2[start2 + pos - 1 - len1]
        }

        if nums1[start1 + k - 1] < nums2[start2 + k - 1] {
            return Solution::find_by_leading_index(nums1, nums2, start1 + k, end1, start2, end2, pos - k);
        } else {
            return Solution::find_by_leading_index(nums1, nums2, start1, end1, start2 + k, end2, pos - k);
        }
    }
}