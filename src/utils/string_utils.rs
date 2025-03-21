
/*
 * convert a string similar "1, null, 2, 3" to a Vec<Option<i32>>
 */
#[allow(dead_code)]
pub(crate) fn string_to_option_int_vec(text: &str) -> Vec<Option<i32>> {
    if text.is_empty() {
        return vec![];
    }

    let parts = text.split(",");

    let mut ret: Vec<Option<i32>> = vec![]; 

    for p in parts {
        let new_p = p.trim();

        if new_p == "null" {
            ret.push(None)
        } else {
            let v: i32 = new_p.parse().unwrap();
            ret.push(Some(v));
        }
    }

    ret
}