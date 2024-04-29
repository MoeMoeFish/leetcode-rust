mod solution1;

pub(crate) fn demo() {
    solution1::Solution::set_zeroes(&mut vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]]);
}

#[cfg(test)]
mod test {
    use super::solution1;

    fn inner_test(input: Vec<Vec<i32>>, expected: Vec<Vec<i32>>) {
        let mut i1 = input.clone();
        solution1::Solution::set_zeroes(&mut i1);

        assert_eq!(i1, expected);
    }

    #[test]
    fn test1() {
        inner_test(
            vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]],
            vec![vec![1, 0, 1], vec![0, 0, 0], vec![1, 0, 1]],
        );
    }

    #[test]
    fn test2() {
        inner_test(
            vec![vec![0, 1, 2, 0], vec![3, 4, 5, 2], vec![1, 3, 1, 5]],
            vec![vec![0, 0, 0, 0], vec![0, 4, 5, 0], vec![0, 3, 1, 0]],
        );
    }
}