impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut v: Vec<char> = vec![];

        for c in s.chars() {
            if c == '[' || c == '{' || c == '(' {
                v.push(c);

                continue;
            }

            let t = v.pop().unwrap_or('0');

            let result = (c == ']' && t == '[') || (c == ')' && t == '(') || (c == '}' && t == '{');

            if !result {
                return false
            }
        }

        return v.is_empty();
    }
}

pub(crate) struct Solution;