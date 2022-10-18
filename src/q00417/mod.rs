use std::{ collections::HashMap };
mod solution2;
mod solution3;

/*
 * @Author: MoeMoeFish moemoefish@qq.com
 * @Date: 2022-09-21 13:55:21
 * @LastEditors: MoeMoeFish moemoefish@qq.com
 * @LastEditTime: 2022-10-12 13:15:05
 * @Description: 
 */
pub struct Solution;


#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct Position {
    x: i32,
    y: i32,
    height: i32
}

impl Position {
    pub fn new(x: i32, y: i32, height: i32) -> Self {
        Self { x, y, height }
    }
}

#[derive(Clone, Debug)]
struct FlowResult {
    pacific: i32,
    atlantic: i32
}

/**
 * 流入的结果
 * -1: 不确定，0: 无法流入，1: 可以流入
 */
impl FlowResult {
    pub fn new() -> Self {
        Self {
            pacific: -1,
            atlantic: -1,
        }
    }

    pub fn init(pacific: i32, atlantic: i32) -> Self {
        Self {
            pacific,
            atlantic,
        }
    }

    pub fn or(&self, other: &FlowResult) -> FlowResult {
        FlowResult {
            pacific: Self::or_for_value(self.pacific, other.pacific),
            atlantic: Self::or_for_value(self.atlantic, other.atlantic),
        }
    }

    pub fn or_for_value(left: i32, right: i32) -> i32 {
        if left == 1 || right == 1 {
            return 1;
        }

        if left == -1 || right == -1 {
            return -1;
        }

        0
    }

    pub fn flow_to_both(&self) -> bool {
        self.pacific == 1 && self.atlantic == 1
    }

    pub fn confirm_value(&mut self) {
        if self.pacific == -1 {
            self.pacific = 0
        }
        
        if self.atlantic == -1 {
            self.atlantic = 0
        }
    }
}

impl Solution {
    pub fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        if heights.len() == 0 {
            return vec![vec![]];
        }
        let n = heights.len(); // 高度，y 轴
        let m = heights.get(0).unwrap().len(); // 宽度, x 轴

        let mut ret = vec![];

        let mut data_map = HashMap::<Position, FlowResult>::new();

        for  i in 0..n {
            let row = heights.get(i).unwrap();
            for j in 0..m {
                let position = &Position::new(j as i32, i as i32, row.get(j).unwrap().clone());
                let mut track_map = HashMap::<Position, FlowResult>::new();
                let mut result = Solution::calculate_block(position, m  as i32, n as i32, &mut data_map, &mut track_map, &heights);
                result.confirm_value();

                if result.flow_to_both() {
                    let ret_block = vec![position.y, position.x];
                    ret.push(ret_block);
                }

                data_map.insert(position.clone(), result);
            }

        }

        ret
    }

    fn calculate_block(position: &Position, m: i32, n: i32, data_map: &mut HashMap<Position, FlowResult>, track_map: &mut HashMap<Position, FlowResult>, heights: &Vec<Vec<i32>>) -> FlowResult {
        if data_map.contains_key(position) {
            return data_map.get(position).unwrap().to_owned();
        }

        if track_map.contains_key(position) {
            return FlowResult::new();
        }

        track_map.insert(position.to_owned(), FlowResult::new());

        let result_top = if position.y == 0 {
            FlowResult::init(1, 0)
        } else {
            Solution::calculate_around_block(position, position.x, position.y - 1, m, n, data_map, track_map, heights)
        };

        
        let result_right = if position.x == (m - 1) {
            FlowResult::init(0, 1)
        } else {
            Solution::calculate_around_block(position, position.x + 1, position.y, m, n, data_map, track_map, heights)
        };

        let result_bottom = if position.y == (n - 1) {
            FlowResult::init(0, 1)
        } else {
            Solution::calculate_around_block(position, position.x, position.y + 1, m, n, data_map, track_map, heights)
        };

        let result_left = if position.x == 0 {
            FlowResult::init(1, 0)
        } else {
            Solution::calculate_around_block(position, position.x - 1, position.y, m, n, data_map, track_map, heights)
        };

        let result = result_top.or(&result_right).or(&result_bottom).or(&result_left);

        if (result.atlantic != -1) && (result.pacific != -1) {
            data_map.insert(position.to_owned(), result.clone());
        }

        result
    }

    fn calculate_around_block(position: &Position, x: i32, y: i32,  m: i32, n: i32, data_map: &mut HashMap<Position, FlowResult>, track_map: &mut HashMap<Position, FlowResult>, heights: &Vec<Vec<i32>>) -> FlowResult {
        let height = heights.get(y as usize).unwrap().get(x as usize).unwrap().to_owned();
        let new_position = Position::new(x, y, height);

        if new_position.height <= position.height {
            return Solution::calculate_block(&new_position, m, n, data_map, track_map, heights);
        }

        FlowResult::init(0, 0)
    }


}

pub(crate) fn demo() {
    let heights = vec![vec![1,2,2,3,5],vec![3,2,3,4,4],vec![2,4,5,3,1],vec![6,7,1,4,5],vec![5,1,1,2,4]];
    let answer = Solution::pacific_atlantic(heights.clone());

    let answer2 = solution2::Solution::pacific_atlantic(heights.clone());

    let answer3 = solution3::Solution::pacific_atlantic(heights.clone());
    
    println!("q00417 answer: {:?}, {:?}, {:?}", answer, answer2, answer3);
}

#[cfg(test)]
mod tests {
    use crate::q00417::solution2;

    use super::Solution;

    // 1 2 2 3 5
    // 3 2 3 4 4
    // 2 4 5 3 1
    // 6 7 1 4 5
    // 5 1 1 2 4
    #[test]
    fn test01() {
        let heights = vec![vec![1,2,2,3,5],vec![3,2,3,4,4],vec![2,4,5,3,1],vec![6,7,1,4,5],vec![5,1,1,2,4]];
        let expected = vec![vec![0,4],vec![1,3],vec![1,4],vec![2,2],vec![3,0],vec![3,1],vec![4,0]];

        let ret = Solution::pacific_atlantic(heights.clone());

        assert_eq!(ret, expected);

        let ret2 = solution2::Solution::pacific_atlantic(heights.clone());
        assert_eq!(ret2, expected);

        let ret3 = solution2::Solution::pacific_atlantic(heights.clone());
        assert_eq!(ret3, expected);
    }
    
    #[test]
    fn test02() {
        let heights = vec![vec![1]];
        let expected = vec![vec![0, 0]];

        let ret = Solution::pacific_atlantic(heights.clone());
        assert_eq!(ret, expected);

        let ret2 = solution2::Solution::pacific_atlantic(heights.clone());
        assert_eq!(ret2, expected);

        let ret3 = solution2::Solution::pacific_atlantic(heights.clone());
        assert_eq!(ret3, expected);
    }
    
}
