mod solution1;

pub(crate) fn demo() {
    let str: &str = "[]()";

    solution1::Solution::is_valid(str.to_string());
}

#[cfg(test)]
mod test {
    use super::solution1;

    fn inner_test(s: &str, expected: bool) {
        assert_eq!(solution1::Solution::is_valid(s.to_string()), expected);
    }

    #[test]
    fn test1() {
        inner_test("()", true);
    }

    #[test]
    fn test2() {
        inner_test("()[]{}", true);
    }

    #[test]
    fn test3() {
        inner_test("(]", false);
    }

    #[test]
    fn test4() {
        inner_test("[", false);
    }
}