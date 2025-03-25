mod solution1;

pub (crate) fn demo() {
    let num = "".to_string();
    let k: i32 = 1;
    solution1::Solution::remove_kdigits(num, k);
}

#[cfg(test)]
mod test {
    use super::solution1;

    fn inner_test(num: &str, k: i32, expected: &str) {
        let num_s = num.to_string();
        assert_eq!(solution1::Solution::remove_kdigits(num_s, k), expected);
    }
}