
impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let m = matrix.len();
        let n = matrix[0].len();

        let flag = (2 as i32).pow(30) + 1;

        for x in 0..n {
            for y in 0..m {
                let v = matrix[y][x];

                if v == 0 {
                    for p in 0..n {
                        if matrix[y][p] != 0 {
                            matrix[y][p] = flag;
                        }
                    }

                    for p in 0..m {
                        if matrix[p][x] != 0 {
                            matrix[p][x] = flag;
                        }
                    }
                }
            }
        }

        for x in 0..n {
            for y in 0..m {
                if matrix[y][x] == flag {
                    matrix[y][x] = 0;
                }
            }
        }
    }
}

pub(crate) struct Solution;