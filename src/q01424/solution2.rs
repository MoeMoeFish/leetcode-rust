/*
 * @Author: MoeMoeFish moemoefish@qq.com
 * @Date: 2022-11-24 13:55:00
 * @LastEditors: MoeMoeFish moemoefish@qq.com
 * @LastEditTime: 2022-11-25 15:36:59
 * @Description: 
 */

use std::cmp::Ordering;

pub struct Solution;

/**
 * 实现逻辑
 */
impl Solution {
    pub fn find_diagonal_order(nums: Vec<Vec<i32>>) -> Vec<i32> {
        let mut temp = Vec::<(usize, usize, i32)>::new();

        for i in 0..nums.len() {
            for j in 0..nums[i].len() {
                temp.push((i + j, j, nums[i][j]));
            }
        }

        temp.sort_by(|x, y| {
            if x.0 == y.0 && x.1 == y.1 {
                return Ordering::Equal
            }

            if x.0 > y.0 || x.0 == y.0 && x.1 > y.1 {
                return Ordering::Greater
            }

            Ordering::Less
        });

        let ret = temp.iter().map(|x| x.2).collect();
        return ret;
    }
}
