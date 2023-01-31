/*
 * @Author: MoeMoeFish moemoefish@qq.com
 * @Date: 2023-01-31 13:49:42
 * @LastEditors: MoeMoeFish moemoefish@qq.com
 * @LastEditTime: 2023-01-31 14:16:48
 * @Description: 在两个数组中分别定义两个点，根据中位数的定义有 i + j = (m - i) + (n - j)
 */

use std::cmp;

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let m = nums1.len();
        let n = nums2.len();
        let len = m + n;

        if m < n {
            return Solution::find_median_sorted_arrays(nums2, nums1);
        }

        let i = 0;
        let j = (m + n) / 2 - i;

        loop {
            if (nums1[i] < nums2[j + 1]) && (nums2[j] < nums1[i + 1]) {
                return (cmp::max(nums1[i], nums2[j]) + cmp::min(nums1[i + 1], nums2[j + 1])) as f64 / 2.0;
            } 
        }

        todo!("");

    }
}

pub struct Solution {}