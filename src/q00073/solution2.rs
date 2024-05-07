/**
 * 通过第一行和第一列保存该行或该列是否应为 0
 * 通过一个变量，保存第一行或第一列是否本身包含 0
 */

fn pp(items: &Vec<Vec<i32>>) {
    println!("[");
    for y in 0..items.len() {
        for x in 0..items[0].len() {
            print!("{}, ", items[y][x]);
        }
        println!(",");
    }
    println!("]");
}

impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let m = matrix.len();
        let n = matrix[0].len();

        let mut flag: u8 = 0;

        for y in 0..m {
            if matrix[y][0] == 0 {
                flag = flag | 0b1;
                break;
            }
        }

        for x in 0..n {
            if matrix[0][x] == 0 {
                flag = flag | 0b10;
                break;
            }
        }

        for y in 0..m {
            for x in 0..n {
                let v = matrix[y][x];

                if v == 0 {
                    matrix[y][0] = 0;
                    matrix[0][x] = 0;
                }
            }
        }

        for y in 1..m {
            if matrix[y][0] == 0 {
                for x in 0..n {
                    matrix[y][x] = 0;
                }
            }
        }

        for x in 1..n {
            if matrix[0][x] == 0 {
                for y in 1..m {
                    matrix[y][x] = 0;
                }
                if flag & 0b10 != 0 {
                    matrix[0][x] = 0;
                }

            }
        }

        for y in 0..m {
            if flag & 0b1 != 0 {
                matrix[y][0] = 0;
            }
        }

        for x in 0..n {
            if flag & 0b10 != 0 {
                matrix[0][x] = 0;
            }
        }
    }
}

pub (crate) struct Solution;