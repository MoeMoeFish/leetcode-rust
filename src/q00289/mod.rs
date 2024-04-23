mod solution1;

pub(crate) fn demo() {
    let input = vec![
        vec![0,1,0],
        vec![0,0,1],
        vec![1,1,1],
        vec![0,0,0]
    ];

    let mut i2: Vec<Vec<i32>> = vec![];
    solution1::Solution::game_of_life(&mut i2)
}

#[cfg(test)]
mod test {

}