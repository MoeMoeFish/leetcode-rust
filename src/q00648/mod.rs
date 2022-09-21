/*
 * @Author: MoeMoeFish moemoefish@qq.com
 * @Date: 2022-07-07 19:42:38
 * @LastEditors: MoeMoeFish moemoefish@qq.com
 * @LastEditTime: 2022-09-21 14:00:47
 * @Description: 
 */
pub struct Solution;

impl Solution {
    pub fn replace_words(dictionary: Vec<String>, sentence: String) -> String {
        let words = sentence.split(" ").collect::<Vec<&str>>();
        let mut dict = dictionary;
        dict.sort_by(|a, b| a.len().cmp(&(b.len())));

        let mut new_words: Vec<&str> = vec![];

        for word in words.into_iter() {
            let mut new_word = word;
            for root in &dict[..] {
                if root.len() >= word.len() {
                    break;
                } else {
                    let mut is_equal = true;
                    for (i, rc) in root.chars().enumerate() {
                        if word.chars().nth(i).unwrap() != rc {
                            is_equal = false;
                            break;
                        }
                    }
                    if is_equal {
                        new_word = &root[..];
                        break;
                    }
                }
            }

            new_words.push(new_word);
        }

        let ret = new_words.join(" ");
        ret
    }
}

pub fn demo() {
    let dict = vec![String::from("cat"), String::from("bat"), String::from("rat")];
    let sentence = String::from("the cattle was rattled by the battery");
    let answer = Solution::replace_words(dict, sentence);
    println!("q00648: {}", answer)
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test1() {
        let dict = vec![String::from("cat"), String::from("bat"), String::from("rat")];
        let sentence = String::from("the cattle was rattled by the battery");
        let ret = Solution::replace_words(dict, sentence);
        assert_eq!(ret, "the cat was rat by the bat");
    }

    #[test]
    fn test2() {
        let dict = vec![String::from("a"), String::from("b"), String::from("c")];
        let sentence = String::from("aadsfasf absbs bbab cadsfafs"); ;

        let ret = Solution::replace_words(dict, sentence);
        assert_eq!(ret, "a a b c");
    }

    #[test]
    fn test3() {
        let dict = vec![String::from("a"), String::from("aa"), String::from("aaa"), String::from("aaaa")];
        let sentence = String::from("a aa a aaaa aaa aaa aaa aaaaaa bbb baba ababa");

        let ret = Solution::replace_words(dict, sentence);
        assert_eq!(ret, "a a a a a a a a bbb baba a");
    }
}