/**
 * 与上一种解法的思路一样，只是用一个循环来代替了四个循环。不需要额外处理边界条件。
 */

impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let (m, n) = (matrix.len(), matrix[0].len());
        let (mut t, mut r, mut b, mut l) = (0, n, m, 0);
        let mut ret: Vec<i32> = vec![];

        loop {
            if l >= r || t >= b {
                return ret;
            }

            for i in l..r {
                ret.push(matrix[t][i]);
            }

            for i in t + 1..b {
                ret.push(matrix[i][r-1]);
            }

            if l < r - 1  && t < b - 1 {
                for i in (l..r-1).rev() {
                    ret.push(matrix[b-1][i]);
                }

                for i in (t+1..b-1).rev() {
                    ret.push(matrix[i][l]);
                }
            }

            t += 1;
            r -= 1;
            b -= 1;
            l += 1;
        }

    }
}

pub(crate) struct Solution;
