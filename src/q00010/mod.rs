/*
 * @Author: MoeMoeFish moemoefish@qq.com
 * @Date: 2023-02-01 20:16:33
 * @LastEditors: MoeMoeFish moemoefish@qq.com
 * @LastEditTime: 2023-02-17 13:35:37
 * @Description: 10. Regular Expression Matching
 */

mod solution1;
mod solution2;

pub(crate) fn demo() {
    let s = "aa";
    let p = "a";

    println!("q00010 solution1: {}", solution1::Solution::is_match(s.to_string(), p.to_string()));
    println!("q00010 solution1: {}", solution2::Solution::is_match(s.to_string(), p.to_string()));
}

#[cfg(test)]
mod test {
    use super::solution1;
    use super::solution2;

    fn inner_test(s: String, p: String, ans: bool) {
        let ret1 = solution1::Solution::is_match(s.clone(), p.clone());
        assert_eq!(ans, ret1);

        let ret2 = solution2::Solution::is_match(s.clone(), p.clone());
        assert_eq!(ans, ret2);
    }

    #[test]
    fn test1() {
        let s = "aa".to_string();
        let p = "a".to_string();

        inner_test(s, p, false);
    }

    #[test]
    fn test2() {
        let s = "aa".to_string();
        let p = "a*".to_string();

        inner_test(s, p, true);
    }

    #[test]
    fn test3() {
        let s = "ab".to_string();
        let p = ".*".to_string();

        inner_test(s, p, true);
    }

    #[test]
    fn test4() {
        let s = "bbbba".to_string();
        let p = ".*a*a".to_string();

        inner_test(s, p, true);

    }

    #[test]
    fn test5() {
        let s = "aaaaaaaaaaaaaaaaaaab".to_string();
        let p = "a*a*a*a*a*a*a*a*a*a*a*a*a*a*a*".to_string();

        inner_test(s, p, false);

    }

    #[test]
    fn test6() {
        let s = "aaaaaaaaaaaaaaaaaaab".to_string();
        let p = "a*a*a*a*a*a*a*a*a*a*".to_string();

        inner_test(s, p, false);

    }

    #[test]
    fn test7() {
        let s = "aaab".to_string();
        let p = "a*a*".to_string();

        inner_test(s, p, false);

    }

}

