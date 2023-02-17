/*
 * @Author: MoeMoeFish moemoefish@qq.com
 * @Date: 2023-02-09 12:47:15
 * @LastEditors: MoeMoeFish moemoefish@qq.com
 * @LastEditTime: 2023-02-17 13:37:10
 * @Description: 使用标准的 dp 方法，
 * 针对当前字符是否为 * 做特殊判断
 */
impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        if s == "" && p == "" {
            return true
        }
        
        let m = s.len();
        let n = p.len();

        let mut buffer = vec![vec![false; n + 1]; m + 1];

        let mut ret: bool;
        for i in 0..m+1 {
            for j in 0..n+1 {
                println!("compare: {}, {}", i, j); // todo!
                if i == 0 && j == 0 {
                    buffer[i][j] = true;
                    continue;
                } else if j == 0 {
                    buffer[i][j] = false;
                    continue;
                } else if i == 0 {
                    if &p[j-1..j] == "*" {
                        buffer[i][j] = buffer[i][j - 2];
                    } else {
                        buffer[i][j] = false;
                    }
                    continue;
                }
                
                let pc = if j > 0 { &p[j-1..j] } else { "" };
                let sc = if i > 0 { &s[i-1..i] } else { "" };

                if pc == "*" {
                    let real = &p[j-2..j-1];
                    ret = buffer[i][j - 2];

                    if !ret {
                        if Solution::compare_str(sc, real) {
                            ret = buffer[i - 1][j - 2] || buffer[i - 1][j];
                        } else {
                            ret = buffer[i][j - 2]
                        }
                    }
                } else if Solution::compare_str(sc, pc) {
                    ret = buffer[i - 1][j - 1];
                } else {
                    ret = false;
                }

                buffer[i][j] = ret;
            }
        }

        println!("result: {:?}", buffer); // todo!
        return buffer[m][n];
        
    }

    pub fn compare_str(s: &str, p:&str) -> bool {
        s == p || p == "."
    }
}

pub(crate) struct Solution;