/*
 * @Author: MoeMoeFish moemoefish@qq.com
 * @Date: 2023-02-01 20:18:10
 * @LastEditors: MoeMoeFish moemoefish@qq.com
 * @LastEditTime: 2023-02-08 13:32:53
 * @Description: 
 */

 impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        if s == p || p == ".*".to_string() {
            return true;
        }

        let s_len = s.len();
        let p_len = p.len();

        let mut buffer: Vec<Vec<Option<bool>>> = vec![vec![None; p_len + 1]; s_len + 1];

        Solution::is_match_backtrace_dp(s.as_str(), p.as_str(), 0, s_len, 0, p_len, &mut buffer)
    }

    fn is_match_backtrace_dp(s: &str, p:&str, source_index: usize, source_len: usize, pattern_index: usize, pattern_len: usize, buffer:  &mut Vec<Vec<Option<bool>>>) -> bool {
        let ret = buffer[source_index][pattern_index];

        let new_ret = match ret {
            None => Solution::is_match_backtrace(s, p, source_index, source_len, pattern_index, pattern_len, buffer),
            _ => ret.unwrap(),
        };

        buffer[source_index][pattern_index] = Some(new_ret);
        
        return new_ret;
    }

    pub fn is_match_backtrace(s: &str, p:&str, source_index: usize, source_len: usize, pattern_index: usize, pattern_len: usize, buffer:  &mut Vec<Vec<Option<bool>>>) -> bool {
        if source_index == source_len {
            if pattern_index == pattern_len {
                return true
            }

            if pattern_index + 1 < pattern_len && (&p[pattern_index + 1..pattern_index + 2] == "*") {
                return Solution::is_match_backtrace_dp(s, p, source_index, source_len, pattern_index + 2, pattern_len, buffer);
            }

            return false;
        }

        if pattern_index == pattern_len {
            return false;
        }

        let s_curr = &s[source_index..source_index+1];
        let p_curr = &p[pattern_index..pattern_index + 1];

        if pattern_index + 1 < pattern_len && &p[pattern_index + 1..pattern_index + 2] == "*" {
            let res = Solution::is_match_backtrace_dp(s, p, source_index, source_len, pattern_index + 2, pattern_len, buffer);
            if res {
                return true;
            }

            if Solution::is_char_match(s_curr, p_curr) {
                return Solution::is_match_backtrace_dp(s, p,  source_index + 1, source_len,
                    pattern_index, pattern_len, buffer);
            }
        } else {
            if Solution::is_char_match(s_curr, p_curr) {
                let ret = Solution::is_match_backtrace_dp(s, p, source_index + 1, source_len, pattern_index + 1, pattern_len, buffer);
                if ret {
                    return true
                }
            }
        }

        return false
    }

    fn is_char_match(s: &str, p: &str) -> bool {
        (s == p) || (p == ".")
    }
}

pub(crate) struct Solution {}
