/**
 * 
 */

mod solution1;
mod solution2;

pub(crate) fn demo() {
    let v = vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]];
    solution1::Solution::set_zeroes(&mut v.clone());
    solution2::Solution::set_zeroes(&mut v.clone());
}

#[cfg(test)]
mod test {
    use super::solution1;
    use super::solution2;

    fn inner_test(input: Vec<Vec<i32>>, expected: Vec<Vec<i32>>) {
        let mut i1 = input.clone();
        solution1::Solution::set_zeroes(&mut i1);
        assert_eq!(i1, expected);

        let mut i2 = input.clone();
        solution2::Solution::set_zeroes(&mut i2);
        assert_eq!(i2, expected);
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

    #[test]
    fn test3() {
        inner_test(vec![vec![3,5,5,6,9,1,4,5,0,5],vec![2,7,9,5,9,5,4,9,6,8],vec![6,0,7,8,1,0,1,6,8,1],vec![7,2,6,5,8,5,6,5,0,6],vec![2,3,3,1,0,4,6,5,3,5],vec![5,9,7,3,8,8,5,1,4,3],vec![2,4,7,9,9,8,4,7,3,7],vec![3,5,2,8,8,2,2,4,9,8]], 
        vec![vec![0,0,0,0,0,0,0,0,0,0],vec![2,0,9,5,0,0,4,9,0,8],vec![0,0,0,0,0,0,0,0,0,0],vec![0,0,0,0,0,0,0,0,0,0],vec![0,0,0,0,0,0,0,0,0,0],vec![5,0,7,3,0,0,5,1,0,3],vec![2,0,7,9,0,0,4,7,0,7],vec![3,0,2,8,0,0,2,4,0,8]]);
    }
}