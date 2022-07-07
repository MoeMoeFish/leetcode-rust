/*
 * @Author: ytyu
 * @Date: 2022-07-01 21:42:00
 * @LastEditors: MoeMoeFish moemoefish@qq.com
 * @LastEditTime: 2022-07-07 16:09:06
 * @Description: 
 */
pub struct Solution {}

impl Solution {
    pub fn diff_ways_to_compute(expression: String) -> Vec<i32> {
        Solution::diff_ways_to_compute_str(&expression)
    }

    pub fn diff_ways_to_compute_str(express: &str) -> Vec<i32> {
        let mut ret: Vec<i32> = vec![];
        let mut prefix_length = 0;
        let mut hasSplit = false;

        for  c in express.chars() {
            if c == '+' || c == '-' || c == '*' {
                let l_vec = Solution::diff_ways_to_compute_str(&express[..prefix_length]);
                let r_vec = Solution::diff_ways_to_compute_str(&express[(prefix_length + c.len_utf8())..]);
                
                let result_vec = Solution::calculate_result(c, l_vec, r_vec);

                for num in result_vec.into_iter() {
                    ret.push(num);
                }

                hasSplit = true;
            }

            prefix_length += c.len_utf8();
        }

        if !hasSplit {
            ret.push(express.parse().unwrap())
        }

        ret
    }

    pub fn calculate_result(c: char, l_vec: Vec<i32>, r_vec: Vec<i32>) -> Vec<i32> {
        let mut ret: Vec<i32> = vec![];

        for l in &l_vec[..] {
            for r in &r_vec[..] {
                match c {
                    '+' => ret.push(l + r),
                    '-' => ret.push(l - r),
                    '*' => ret.push(l * r) ,
                    _ => (),
                }
            }
        }

        ret
    }


}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test() {
        let ret = Solution::diff_ways_to_compute(String::from("2-1-1"));
        assert_eq!(ret, [0,2])
    }
}


