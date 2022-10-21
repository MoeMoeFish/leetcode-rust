/*
 * @Author: MoeMoeFish moemoefish@qq.com
 * @Date: 2022-10-18 13:45:23
 * @LastEditors: MoeMoeFish moemoefish@qq.com
 * @LastEditTime: 2022-10-21 15:28:31
 * @Description: 
 */
pub struct Solution;

impl Solution {
    fn digit_set(digits: &Vec<i32>, digits_len: i32, n_arr: &Vec<i32>, deep: usize) -> i32 {
        let current_n = n_arr[deep];
        
        let mut count = 0;
        for i in digits {
            let u = i.clone();
            if u < current_n {
                if deep <= 0 as usize {
                    count += 1;
                } else {
                    count += Solution::digit_set(digits, digits_len, n_arr,  deep - 1);
                }
            } else if u == current_n {
                if deep <= 0 as usize {
                    count += 1
                } else {
                    count += digits_len.pow(deep as u32);
                }
            }
        }

        count
    }

    pub fn at_most_n_given_digit_set(digits: Vec<String>, n: i32) -> i32 {
        if n == 0 {
            return 0
        }

        let mut n_arr: Vec<i32> = vec![];
        let mut num = n;
        while num > 0 {
            let rem = num % 10;
            num = num / 10;
            n_arr.push(rem)
        }
        n_arr.reverse();

        let num_digits = digits.into_iter().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<_>>();
        let len_n = n_arr.len();

        let mut zero_count = 0;
        if len_n > 1 {
            for i in 0..(len_n - 1) {
                zero_count += num_digits.len().pow((i + 1) as u32)
            }
        }


        let count = Solution::digit_set(&num_digits, num_digits.len() as i32, &n_arr, len_n - 1);
        println!("zero_count: {:?}, count: {:?}", zero_count, count);
        return zero_count as i32 + count;
    }
}

pub(crate) fn demo() {
    let nums = vec!["1","4","9"].into_iter().map(String::from).collect();
    let n = 1000000000;

    println!("q00902 answer: {:?}", Solution::at_most_n_given_digit_set(nums, n));
}

#[cfg(test)]
mod test {
    use super::Solution;

    fn run_test(nums_arr: Vec<&str>, n: i32, out: i32) {
        let nums  = nums_arr.into_iter().map(String::from).collect::<Vec<_>>();

        assert_eq!(out, Solution::at_most_n_given_digit_set(nums.clone(), n.clone()), "solution 1");
    }
    #[test]
    fn test1() {
        run_test(vec!["1","4","9"], 1000000000, 29523);
    }

    #[test]
    fn test2() {
        run_test(vec!["1","3","5","7"], 100, 20);
    }

    #[test]
    fn test3() {
        run_test(vec!["3","4","5","6"], 64, 18);
    }
}
