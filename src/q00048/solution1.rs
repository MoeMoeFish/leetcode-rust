impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let m = matrix.len();
        
        let mut i = 0;
        
        while i < m - 1 {
            let mut x = 0;
            let y = i;

            x += i;


            while x < m - 1 - i {
                let tmp = matrix[y][x];
                matrix[y][x] = matrix[m - 1 - x][y];
                matrix[m - 1 - x][y] = matrix[m - 1 - y][m - 1 - x];
                matrix[m - 1 - y][m - 1 - x] = matrix[x][m - 1 - y];
                matrix[x][m - 1 - y] = tmp;

                x = x + 1;
            }

            i += 1;
        }
    }
}

pub(crate) struct Solution;