mod solution1;

pub(crate) fn demo() {
    solution1::Solution::two_sum(vec![1,2], 3);
}

#[cfg(test)]
mod tests {
    use super::solution1;

    fn inner_test(nums: Vec<i32>, target: i32, answer: Vec<i32>) {
        assert_eq!(solution1::Solution::two_sum(nums, target), answer);
    }

    #[test]
    fn test1() {
        inner_test(vec![2,7,11,15], 9, vec![0,1]);
    }

    #[test]
    fn test2() {
        inner_test(vec![3,2,4], 6, vec![1,2]);
    }

    #[test]
    fn test3() {
        inner_test(vec![3,3], 6, vec![0,1]);
    }
}