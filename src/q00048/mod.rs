mod solution1;

pub(crate) fn demo() {
    solution1::Solution::rotate(&mut vec![vec![0]]);
}

#[cfg(test)]
mod test {
    use super::*;

    fn inner_test(input: Vec<Vec<i32>>, expected: Vec<Vec<i32>>) {
        let mut i1 = input.clone();
        solution1::Solution::rotate(&mut i1);
        assert_eq!(i1, expected);
    }

    #[test]
    fn test1() {
        inner_test(
            vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]],
            vec![vec![7, 4, 1], vec![8, 5, 2], vec![9, 6, 3]],
        );
    }

    #[test]
    fn test2() {
        inner_test(vec![
            vec![5, 1, 9, 11],
            vec![2, 4, 8, 10],
            vec![13, 3, 6, 7],
            vec![15, 14, 12, 16],
        ], vec![
            vec![15, 13, 2, 5],
            vec![14, 3, 4, 1],
            vec![12, 6, 8, 9],
            vec![16, 7, 10, 11],
        ])
    }
}

