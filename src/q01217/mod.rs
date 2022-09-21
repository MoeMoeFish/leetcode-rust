/*
 * @Author: MoeMoeFish moemoefish@qq.com
 * @Date: 2022-07-08 10:38:50
 * @LastEditors: MoeMoeFish moemoefish@qq.com
 * @LastEditTime: 2022-09-21 13:59:13
 * @Description: 
 */
pub struct Solution;

impl Solution {
    pub fn min_cost_to_move_chips(position: Vec<i32>) -> i32 {
        let mut odd = 0;
        let mut even = 0;

        for num in position {
            if num % 2 == 0 {
                even += 1;
            } else {
                odd += 1;
            }
        }

        return if odd < even { odd } else { even };
    }
}

pub fn demo() {
    Solution::min_cost_to_move_chips(vec![1,2,3]);
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {
        let position = vec![1,2,3];
        let answer = 1;
        let ret = super::Solution::min_cost_to_move_chips(position);
        assert_eq!(ret, answer)
    }

    #[test]
    fn test2() {
        let position = vec![2,2,2,3,3];
        let answer = 2;
        let ret = super::Solution::min_cost_to_move_chips(position);
        assert_eq!(ret, answer)
    }
    #[test]
    fn test3() {
        let position = vec![1,1000000000];
        let answer = 1;
        let ret = super::Solution::min_cost_to_move_chips(position);
        assert_eq!(ret, answer)
    }
}