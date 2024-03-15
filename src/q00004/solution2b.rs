// i + j = m - i + n - j
// 2j = m + n - 2i
// j = (m + n)/2 - i
// j = (m + n) / 2; if i == 0 and m <= n, so (m + n) / 2 <= n; 
// j = (m + n) / 2 - m = (n - m) / 2; if i == m and m <=n; so (n - m) / 2 >= 0


// i,j 表示什么： 表示的是在自己数组中自然数(base = 1) 的位置，也就是前开后闭中的后闭

// 为什么奇数选择左侧的

use std::cmp;

impl Solution {
    // 编写左右区间时，区间都是前闭后开
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let m = nums1.len();
        let n = nums2.len();
        let total = m + n;

        if m > n {
            return Solution::find_median_sorted_arrays(nums2, nums1);
        }

        if m == 0 {
            return (nums2[n / 2] + nums2[(n - 1) / 2]) as f64 / 2.0;
        }

        let mut left: usize = 0;
        let mut right = m;

        loop {
            let i = (left + right) / 2;
            let j  = (total + 1) / 2 - i;

            if i > 0 && j < n && nums1[i - 1] > nums2[j] {
                right = i - 1;
                continue;
            }

            if i < m && j > 0 && nums1[i] < nums2[j - 1] {
                left = i + 1;
                continue;
            }

            let lv = if i == 0 { nums2[j - 1] } else if j == 0 { nums1[i - 1]} else { cmp::max(nums1[i - 1], nums2[j - 1]) };
            let rv = if i == m { nums2[j] } else if j == m { nums1[i] } else { cmp::min(nums1[i], nums2[j]) };

            if total % 2 == 0 {
                return (lv + rv) as f64 / 2.0;
            } else {
                return lv as f64;
            }
        }
    }
}


pub struct Solution {}