/*
 * @Author: MoeMoeFish moemoefish@qq.com
 * @Date: 2023-01-10 11:04:19
 * @LastEditors: MoeMoeFish moemoefish@qq.com
 * @LastEditTime: 2023-02-01 20:12:45
 * @Description: 
 */

mod solution1;
mod solution2;
mod solution1b;
mod solution2b;

pub(crate) fn demo() {
    let nums1 = vec![1,3];
    let nums2 = vec![1,3];
    println!("q0004 solution1: {}", solution1::Solution::find_median_sorted_arrays(nums1.clone(), nums2.clone()));
    println!("q0004 solution2: {}", solution2::Solution::find_median_sorted_arrays(nums1.clone(), nums2.clone()));
    println!("q0004 solution1b: {}", solution1b::Solution::find_median_sorted_arrays(nums1.clone(), nums2.clone()));
    println!("q0004 solution2b: {}", solution2b::Solution::find_median_sorted_arrays(nums1.clone(), nums2.clone()));
}

#[cfg(test)]
mod tests {
    // use super::solution1;
    // use super::solution2;
    use super::solution1b;
    use super::solution2b;

    fn inner_test(nums1: &Vec<i32>, nums2: &Vec<i32>, answer: f64) {
        // let ret = solution1::Solution::find_median_sorted_arrays(nums1.clone(), nums2.clone());
        // assert_eq!(ret, answer);

        // let ret2 = solution2::Solution::find_median_sorted_arrays(nums1.clone(), nums2.clone());
        // assert_eq!(ret2, answer);

        let ret3 = solution1b::Solution::find_median_sorted_arrays(nums1.clone(), nums2.clone());
        assert_eq!(ret3, answer);

        let ret3 = solution2b::Solution::find_median_sorted_arrays(nums1.clone(), nums2.clone());
        assert_eq!(ret3, answer);
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

    #[test]
    fn test03() {
        let nums1 = vec![];
        let nums2 = vec![1];
        let answer = 1.0;

        inner_test(&nums1, &nums2, answer);

    }

    #[test]
    fn test04() {
        let nums1 = vec![1, 3, 5];
        let nums2 = vec![2, 4];
        let answer = 3.0;

        inner_test(&nums1, &nums2, answer);

    }

    #[test]
    fn test05() {
        let nums1 = vec![1, 5];
        let nums2 = vec![2, 3, 4];
        let answer = 3.0;

        inner_test(&nums1, &nums2, answer);

    }

    #[test]
    fn test06() {
        let nums1 = vec![1];
        let nums2 = vec![2, 3, 4, 5];
        let answer = 3.0;

        inner_test(&nums1, &nums2, answer);
    }

    #[test]
    fn test07() {
        let nums1 = vec![];
        let nums2 = vec![1, 2, 3, 4, 5];
        let answer = 3.0;

        inner_test(&nums1, &nums2, answer);
    }

    #[test]
    fn test08() {
        let nums1 = vec![1, 2, 3, 4, 5];
        let nums2 = vec![];
        let answer = 3.0;

        inner_test(&nums1, &nums2, answer);
    }

    #[test]
    fn test09() {
        let nums1 = vec![2, 3, 4, 5];
        let nums2 = vec![1];
        let answer = 3.0;

        inner_test(&nums1, &nums2, answer);
    }

    #[test]
    fn test10() {
        let nums1 = vec![5,6,7,8];
        let nums2 = vec![];
        let answer = 6.5;

        inner_test(&nums1, &nums2, answer);
    }

    #[test]
    fn test11() {
        let nums1 = vec![6,7,8];
        let nums2 = vec![5];
        let answer = 6.5;

        inner_test(&nums1, &nums2, answer);
    }
}
