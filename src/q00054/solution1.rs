impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let (m, n) = (matrix.len(), matrix[0].len());
        let (mut t, mut r) = (0, 0);
        let mut dir = 0; // 0: right, 1: down, 2: left, 3: up
        let mut ret: Vec<i32> = vec![];

        let (mut x, mut y) = (0, 0);


        if n == 1 {
            for i in 0..m {
                ret.push(matrix[i][0]);
            }

            return ret;
        }

        ret.push(matrix[0][0]);
        loop {
            if dir == 0 {
                if x + 1 >= n - r {
                    return ret;
                }

                for x in x + 1..n - r {
                    ret.push(matrix[y][x]);
                }

                x = n - r - 1;
                t += 1;
                dir = 1;
            } else if dir == 1 {
                if y + 1 >= m - t + 1 {
                    return ret;
                }

                for i in y + 1..m - t + 1 {
                    ret.push(matrix[i][x]);
                }

                y = m - t;
                r += 1;
                dir = 2;
            } else if dir == 2 {
                if x - 1 < r - 1  {
                    return ret;
                }

                for i in (r - 1..x).rev() {
                    ret.push(matrix[y][i]);
                } 

                x = r - 1;
                dir = 3;
            } else {
                if y - 1 < t {
                    return ret;
                }

                for i in (t..y).rev() {
                    ret.push(matrix[i][x]);
                }

                y = t;
                dir = 0;
            }
        }
    }
}

pub(crate) struct Solution;
