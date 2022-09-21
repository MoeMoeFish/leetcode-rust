/*
 * @Author: MoeMoeFish moemoefish@qq.com
 * @Date: 2022-09-21 13:55:21
 * @LastEditors: MoeMoeFish moemoefish@qq.com
 * @LastEditTime: 2022-09-21 14:12:27
 * @Description: 
 */
pub struct Solution;

impl Solution {
    pub fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        panic!("not impl")
    }
}

pub(crate) fn demo() {
    let heights = vec![vec![1,2,2,3,5],vec![3,2,3,4,4],vec![2,4,5,3,1],vec![6,7,1,4,5],vec![5,1,1,2,4]];
    let answer = Solution::pacific_atlantic(heights);
    
    println!("q00417 answer: {:?}", answer);
}

#[cfg(test)]
mod tests {
    // use super::Solution;

    // #[test]
    // fn test01() {
    //     let heights =[[1,2,2,3,5],[3,2,3,4,4],[2,4,5,3,1],[6,7,1,4,5],[5,1,1,2,4]];
    // }
    
}