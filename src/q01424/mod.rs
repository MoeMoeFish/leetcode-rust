/*
 * @Author: MoeMoeFish moemoefish@qq.com
 * @Date: 2022-11-23 20:23:10
 * @LastEditors: MoeMoeFish moemoefish@qq.com
 * @LastEditTime: 2023-01-29 13:59:57
 * @Description: 
 */

pub struct Solution;
mod solution2;

/**
 * 暴力计算，按照对角线的走向逐一遍历数组中的元素，遇到没有元素的位置，直接跳过。
 * 直到某个对角线一个元素也没有就停止
 * 
 * 优势：如果二位数组中的元素较满，则每个元素只会遍历一次，时间复杂度接近 O(n)
 * 
 * 缺陷：整体繁杂度是 O(n^2), 如果有一个极端的情况，一行元素非常多，其他行只有一个元素的情况（厂字型），
 * 会计算大量无用位置， 
 */
impl Solution {
    pub fn find_diagonal_order(nums: Vec<Vec<i32>>) -> Vec<i32> {
        let mut ret = Vec::<i32>::new();


        let mut head = (0, 0_usize);
        loop {
            let mut no_ele = true;  

            let mut i = head.0;
            let mut j = head.1;
            loop {
                if nums[i].len() >= j + 1 {
                    ret.push(nums[i][j]);
                    no_ele = false;
                }

                if i == 0 {
                    break;
                }

                i = i - 1;
                j = j + 1;
            }

            if no_ele {
                break;
            }

            if head.0 < nums.len() - 1 {
                head.0 = head.0 + 1;
            } else {
                head.1 = head.1 + 1;
            }
        }

        return ret;
    }
}

pub(crate) fn demo() {
    let nums = vec![vec![1,2,3], vec![4,5,6], vec![7,8,9]];
    let answer = Solution::find_diagonal_order(nums.clone());
    let answer2 = solution2::Solution::find_diagonal_order(nums.clone());
    println!("q01424 Diagonal Traverse II. Answer: {:?}, {:?}", answer, answer2);
}

#[cfg(test)]
mod test {
    use super::Solution;
    use super::solution2;

    fn run_test(nums: Vec<Vec<i32>>, expected: Vec<i32>) {
        assert_eq!(expected, Solution::find_diagonal_order(nums.clone()));
    }

    fn run_test2(nums: Vec<Vec<i32>>, expected: Vec<i32>) {
        assert_eq!(expected, solution2::Solution::find_diagonal_order(nums.clone()));
    }

    #[test]
    fn test1() {
        let nums = vec![vec![1,2,3],vec![4,5,6],vec![7,8,9]];
        let expected = vec![1,4,2,7,5,3,8,6,9];

        run_test(nums, expected);
    }

    #[test]
    fn test2() {
        let nums = vec![vec![1,2,3,4,5],vec![6,7],vec![8],vec![9,10,11],vec![12,13,14,15,16]];
        let expected = vec![1,6,2,8,7,3,9,4,12,10,5,13,11,14,15,16];

        for _ in 0..10000 {
            run_test(nums.clone(), expected.clone());
        }
    }

    #[test]
    fn test22() {
        let nums = vec![vec![1,2,3,4,5],vec![6,7],vec![8],vec![9,10,11],vec![12,13,14,15,16]];
        let expected = vec![1,6,2,8,7,3,9,4,12,10,5,13,11,14,15,16];

        for _ in 0..10000 {
            run_test2(nums.clone(), expected.clone());
        }
    }

}
