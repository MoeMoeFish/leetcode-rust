impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        if matrix.len() == 0 || matrix[0].len() == 0 {
            return
        }

        let (m, n) = (matrix.len(), matrix[0].len());

        let mut row0 = 1;
        let mut col0 = 1;

        for i in 0..m {
            for j in 0..n {
                if i == 0 && matrix[i][j] == 0 {
                    row0 = 0;
                }

                if j == 0 && matrix[i][j] == 0 {
                    col0 = 0;
                }
            }
        }

        for i in 0..m {
            for j in 0..n {
                if matrix[i][j] == 0 {
                    matrix[i][0] = 0;
                    matrix[0][j] = 0;
                }
            }
        }

        for j  in 1..n {
            if matrix[0][j] == 0 {
                for row in 1..m {
                    matrix[row][j] = 0
                }
            }

            if row0 == 0 {
                matrix[0][j] = 0;
            }
            
        }

        for i in 0..m {
            if matrix[i][0] == 0 {
                for j in 1..n {
                    matrix[i][j] = 0;
                }
            }

            if col0 == 0 {
                matrix[i][0] = 0;
            }
        }

        matrix[0][0] = if row0 == 0 || col0 == 0 {
            0
        } else {
            matrix[0][0]
        };

    }
}

pub(crate) struct Solution;