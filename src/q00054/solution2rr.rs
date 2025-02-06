impl Solution {
    pub(crate) fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        if matrix.is_empty() || matrix[0].is_empty() {
            return vec![];
        }

        let (n, m) = (matrix.len(), matrix[0].len());

        let mut result: Vec<i32> = Vec::with_capacity(m * n);

        let (mut top, mut bottom, mut left, mut right) = (0, n - 1, 0, m - 1);

        while top <= bottom && left <= right {
            for i in left..=right {
                result.push(matrix[top][i]);
            }

            for i in top + 1..=bottom {
                result.push(matrix[i][right]);
            }

            if top < bottom && left < right {
                for i in (left..=right - 1).rev() {
                    result.push(matrix[bottom][i]);
                }

                for i in (top + 1..=bottom - 1).rev() {
                    result.push(matrix[i][left]);
                }
            }

            if bottom == 0 || right == 0 {
                break;
            }

            top += 1;
            bottom -= 1;
            left += 1;
            right -= 1;
        }

        result
    }
}

pub(crate) struct Solution;
