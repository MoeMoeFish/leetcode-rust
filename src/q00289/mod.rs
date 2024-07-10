mod solution1;
mod solution1r;

pub(crate) fn demo() {

    let mut i2: Vec<Vec<i32>> = vec![];
    solution1::Solution::game_of_life(&mut i2);
    solution1r::Solution::game_of_life(&mut i2);
}

#[cfg(test)]
mod test {
    use super::*;

    fn inner_test(input: Vec<Vec<i32>>, ans: Vec<Vec<i32>>) {
        // let mut i1 = input.clone();
        // solution1::Solution::game_of_life( &mut i1);
        // assert_eq!(i1, ans);

        let mut i2 = input.clone();
        solution1r::Solution::game_of_life(&mut i2);
        assert_eq!(i2, ans);
    }

    #[test]
    fn test01() {
        let input = vec![
            vec![0,1,0],
            vec![0,0,1],
            vec![1,1,1],
            vec![0,0,0]
        ];
        
        let ans = vec![
            vec![0,0,0],
            vec![1,0,1],
            vec![0,1,1],
            vec![0,1,0],
        ];
        inner_test(input, ans)
    }
}