impl Solution {
    pub fn remove_kdigits(num: String, k: i32) -> String {
        let mut result = String::new();
        let mut left = k;
        let mut index: i32 = 0;
        let len = num.len() as i32;

        for char in num.chars() {
            if left == 0 {
                break;
            }
            if (len - index) <= left {
                result.push(char);
                left = left - 1;
            } else {
                
            }

            index += 1;
        }

        result
    }
}

pub (crate) struct Solution;