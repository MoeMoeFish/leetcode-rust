use std::collections::HashMap;

/*
 * @Author: MoeMoeFish moemoefish@qq.com
 * @Date: 2022-10-18 12:12:55
 * @LastEditors: MoeMoeFish moemoefish@qq.com
 * @LastEditTime: 2022-10-18 13:32:45
 * @Description: 
 */
pub struct Solution;

impl Solution {
    pub fn most_frequent(nums: Vec<i32>, key: i32) -> i32 {
        let mut number_map = HashMap::<i32, i32>::new();
        for i in 0..(nums.len() - 1) {
            if nums[i] == key {
                let t = nums[i + 1];
                let mut count = 1;
                if number_map.contains_key(&t) {
                    count = number_map.get(&t).unwrap() + 1;
                }
                number_map.insert(t, count);
            }
        }
        
        let mut max = 0;
        let mut target = 0;
        for v in number_map.keys() {
            let number = number_map.get(v).unwrap().to_owned();
            if number > max {
                max = number;
                target = v.to_owned();
            }
        }

        target
    }
}

pub(crate) fn demo() {
    let nums = vec![1,100,200,1,100];
    let key = 1;

    let answer = Solution::most_frequent(nums.clone(), key);
    println!("q02190 answer: {:?}", answer);
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test01() {
        let nums = vec![1,100,200,1,100,200];
        let key = 1;
        let output = 100;

        let answer = Solution::most_frequent(nums.clone(), key);
        assert_eq!(output, answer);
    }

    #[test]
    fn test02() {
        let nums = vec![2,2,2,2,3];
        let key = 2;
        let output = 2;

        let answer = Solution::most_frequent(nums.clone(), key);
        assert_eq!(output, answer);
    }
}
