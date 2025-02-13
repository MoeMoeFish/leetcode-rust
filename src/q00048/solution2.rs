impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        if matrix.len() <= 1 {
            return;
        }

        let n = matrix.len() - 1;
        let (mut left, mut right) = (0, n);
        let mut i = 0;

        while left < right {
            for j in left..right {
                let tmp = matrix[i][j];
                matrix[i][j] = matrix[n - j][i];
                matrix[n - j][i] = matrix[n - i][n - j];
                matrix[n - i][n - j] = matrix[j][n - i];
                matrix[j][n - i] = tmp;
            }

            left += 1;
            right -= 1;
            i += 1;
        }
    }
}

pub(crate) struct Solution;
