/*
 * @Author: MoeMoeFish moemoefish@qq.com
 * @Date: 2022-09-28 15:13:17
 * @LastEditors: MoeMoeFish moemoefish@qq.com
 * @LastEditTime: 2022-09-29 14:01:05
 * @Description: 
 */


pub struct Solution;

impl Solution {
    pub fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = heights.len();
        let m = heights[0].len();

        let mut result1: Vec<Vec<bool>> = vec![vec![false;m];n];
        let mut result2: Vec<Vec<bool>> = vec![vec![false;m];n];

        for i in 0usize..m {
            Solution::find_can_enter(&heights, 0, i, n, m, heights[0][i], &mut result1);
            Solution::find_can_enter(&heights, n - 1, i, n, m, heights[n - 1][i], &mut result2);
        }

        for i in 0usize..n {
            Solution::find_can_enter(&heights, i, 0, n, m, heights[i][0], &mut result1);
            Solution::find_can_enter(&heights, i, m - 1, n, m, heights[i][m - 1], &mut result2);
        }

        let mut result: Vec<Vec<i32>> =  vec![];
        for i in 0..n {
            for j in 0..m {
                if result1[i][j] && result2[i][j] {
                    result.push(vec![i as i32, j as i32])
                }
            }
        }

        result
    }

    pub fn find_can_enter(heights: &Vec<Vec<i32>>, y: usize, x: usize, max_y: usize, max_x: usize, pre_height: i32, result: &mut Vec<Vec<bool>>) {
        if y >= max_y || x >= max_x {
            return
        }

        if result[y][x] {
            return
        }

        let height = heights[y][x];
        if height < pre_height {
            return
        }

        result[y][x] = true;


        Solution::find_can_enter(heights, y + 1, x, max_y, max_x, height, result);
        if y > 0 {
            Solution::find_can_enter(heights, y - 1, x, max_y, max_x, height, result);
        }
        Solution::find_can_enter(heights, y, x + 1, max_y, max_x, height, result);
        if x > 0 {
            Solution::find_can_enter(heights, y, x - 1, max_y, max_x, height, result);
        }

    }
}