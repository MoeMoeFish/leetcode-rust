impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        if matrix.len() == 0 || matrix[0].len() == 0 {
            return vec![]
        }

        let mut ret: Vec<i32> = vec![];
        let (n, m) = (matrix.len(), matrix[0].len());

        let (mut top, mut bottom, mut left, mut right) = (0, n - 1, 0, m - 1);
        
        while top <= bottom && left <= right {
            for i in left..=right {
                ret.push(matrix[top][i]);
            }

            for i in top + 1..=bottom {
                ret.push(matrix[i][right]);
            }

            if top < bottom && left < right {
                for i in (left..=right-1).rev() {
                    ret.push(matrix[bottom][i])
                }

                for i in (top+1..=bottom-1).rev() {
                    ret.push(matrix[i][left]);
                }
            }

            if bottom == 0 || right == 0 {
                break;
            }
            top = top + 1;
            bottom = bottom - 1;
            left = left + 1;
            right = right - 1;
        }


        return ret;
    }
}

pub(crate) struct Solution;
