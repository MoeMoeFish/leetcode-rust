/*
 * @Author: MoeMoeFish moemoefish@qq.com
 * @Date: 2023-01-31 13:49:42
 * @LastEditors: MoeMoeFish moemoefish@qq.com
 * @LastEditTime: 2023-02-15 13:30:14
 * @Description: 在两个数组中分别定义两个点，根据中位数的定义有 i + j = (m - i) + (n - j)
 */

use std::cmp;

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let m = nums1.len();
        let n = nums2.len();
        let len = m + n;

        if m > n {
            return Solution::find_median_sorted_arrays(nums2, nums1);
        }

        if m == 0 {
            if len % 2 == 0 {
                return (nums2[n / 2 - 1] + nums2[n / 2]) as f64 / 2.0;
            } else {
                return nums2[n / 2] as f64;
            }
        }

        let mut left = 0;
        let mut right = m;
        loop {
            let i = (left + right) / 2;
            let j = (len + 1) / 2 - i;
            

            if i > 0 && j < n && nums1[i - 1] > nums2[j] {
                right = i - 1;
            } else if i < m && j > 0 && nums2[j - 1] > nums1[i] {
                left = i + 1;
            } else {
                let ret_left = if i == 0 { nums2[j - 1] } else if j == 0 { nums1[i - 1] } else { cmp::max(nums1[i - 1], nums2[j - 1]) };
                let ret_right = if i == m { nums2[j] } else if j == n { nums1[i] } else { cmp::min(nums1[i], nums2[j]) };

                if len % 2 == 1 {
                    return ret_left as f64;
                } else {
                    return (ret_left + ret_right) as f64 / 2.0;
                }
            }
        }

    }
}

pub struct Solution {}
