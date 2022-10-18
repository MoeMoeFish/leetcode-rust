/*
 * @Author: MoeMoeFish moemoefish@qq.com
 * @Date: 2022-10-18 13:45:23
 * @LastEditors: MoeMoeFish moemoefish@qq.com
 * @LastEditTime: 2022-10-18 13:51:06
 * @Description: 
 */
pub struct Solution;

impl Solution {
    pub fn at_most_n_given_digit_set(digits: Vec<String>, n: i32) -> i32 {
        0
    }
}

pub(crate) fn demo() {
    let nums = vec!["1","4","9"].into_iter().map(String::from).collect();
    let n = 1000000000;

    println!("q00902 answer: {:?}", Solution::at_most_n_given_digit_set(nums, n));
}