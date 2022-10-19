/*
 * @Author: MoeMoeFish moemoefish@qq.com
 * @Date: 2022-10-18 13:45:23
 * @LastEditors: MoeMoeFish moemoefish@qq.com
 * @LastEditTime: 2022-10-19 14:06:18
 * @Description: 
 */
pub struct Solution;

impl Solution {
    fn given_set(digits: &Vec::<i32>, quotient: i32, max: i32, count: i32) -> i32 {
        
        0
    }

    pub fn at_most_n_given_digit_set(digits: Vec<String>, n: i32) -> i32 {
        if n == 0 {
            return 0
        }

        let numDigits = digits.into_iter().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<_>>();
        let lenN = n.to_string().len();

        let zeroCount = if lenN > 1 {
            numDigits.len().pow((lenN - 1) as u32) as i32
        } else {
            0
        };

        let num = n % 10;
        let count = Solution::given_set(&numDigits, n, num, 0);


        return zeroCount + count;
    }
}

pub(crate) fn demo() {
    let nums = vec!["1","4","9"].into_iter().map(String::from).collect();
    let n = 1000000000;

    println!("q00902 answer: {:?}", Solution::at_most_n_given_digit_set(nums, n));
}