mod solution1;

pub(crate) fn demo() {
    println!("q00316");
    let s = "bcabc".to_string();
    solution1::Solution::remove_duplicate_letters(s);
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {
        inner_test("bcabc".to_string(), "a".to_string());
    }

    fn inner_test(input: String, ans: String) {
        assert_eq!(input, ans);
    }
}
