/*
 * @Author: MoeMoeFish moemoefish@qq.com
 * @Date: 2023-01-10 11:04:19
 * @LastEditors: MoeMoeFish moemoefish@qq.com
 * @LastEditTime: 2023-01-29 13:55:54
 * @Description: 
 */

mod solution1;

pub(crate) fn demo() {
    let nums1 = vec![1,3];
    let nums2 = vec![1,3];
    println!("q0004 solution1: {}", solution1::Solution::find_median_sorted_arrays(nums1.clone(), nums2.clone()));
}
