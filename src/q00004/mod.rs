/*
 * @Author: MoeMoeFish moemoefish@qq.com
 * @Date: 2023-01-10 11:04:19
 * @LastEditors: MoeMoeFish moemoefish@qq.com
 * @LastEditTime: 2023-01-30 13:37:33
 * @Description: 
 */

mod solution1;

pub(crate) fn demo() {
    let nums1 = vec![1,3];
    let nums2 = vec![1,3];
    println!("q0004 solution1: {}", solution1::Solution::find_median_sorted_arrays(nums1.clone(), nums2.clone()));
}

#[cfg(test)]
mod tests {
    use super::solution1;

    fn inner_test(nums1: &Vec<i32>, nums2: &Vec<i32>, answer: f64) {
        let ret = solution1::Solution::find_median_sorted_arrays(nums1.clone(), nums2.clone());
        assert_eq!(ret, answer)
    }

    #[test]
    fn test01() {
        let nums1 = vec![1,3];
        let nums2 = vec![2];
        let answer = 2.0;

        inner_test(&nums1, &nums2, answer);
    }

    #[test]
    fn test02() {
        let nums1 = vec![1,2];
        let nums2 = vec![3,4];
        let answer = 2.5;

        inner_test(&nums1, &nums2, answer);
    }
}
