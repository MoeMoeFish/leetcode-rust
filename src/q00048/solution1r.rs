/**
 * 00 10 20 30 40
 * 01 11 21 31 41
 * 02 12 22 32 42
 * 03 13 23 33 43
 * 04 14 24 34 44
 * 
 * x = y
 * y = n - x
 */


impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let n = matrix.len();
        if n == 0 || n == 1 {
            return
        }

        for j in 0..n/2 {
            for i in j..n-1-j {
                let curr = matrix[j][i];
                matrix[j][i] = matrix[n-1-i][j];
                matrix[n-1-i][j] = matrix[n-1-i][n-1-j];
                matrix[n-1-i][n-1-j] = matrix[i][n-1-j];
                matrix[i][n-1-j] = curr;
            }
        }
    }
}

pub(crate) struct Solution;