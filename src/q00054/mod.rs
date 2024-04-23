mod solution1;
mod solution2;

pub(crate) fn demo() {
    let v = vec![
        vec![1, 2, 3],
        vec![4, 5, 6],
        vec![7, 8, 9],
    ];
    solution1::Solution::spiral_order(v.clone());
    solution2::Solution::spiral_order(v.clone());
}

#[cfg(test)]
mod test {
    use super::solution1;
    use super::solution2;

    fn inner_test(input: Vec<Vec<i32>>, expected: Vec<i32>) {
        assert_eq!(solution1::Solution::spiral_order(input.clone()), expected);
        assert_eq!(solution2::Solution::spiral_order(input.clone()), expected);
    }

    #[test]
    fn test1() {
        inner_test(
            vec![
                vec![1, 2, 3],
                vec![4, 5, 6],
                vec![7, 8, 9],
            ],
            vec![1, 2, 3, 6, 9, 8, 7, 4, 5],
        );
    }

    #[test]
    fn test2() {
        inner_test(vec![
            vec![2,3]
        ], 
        vec![2,3])
    }

    #[test]
    fn test3() {
        inner_test(vec![
            vec![2],
            vec![3],
        ], 
        vec![2,3])
    }

    #[test]
    fn test4() {
        inner_test(vec![
            vec![1,2],
            vec![3,4],
        ], 
        vec![1,2,4,3])   
    }
}