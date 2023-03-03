/*
 * @Author: MoeMoeFish moemoefish@qq.com
 * @Date: 2023-02-17 13:56:36
 * @LastEditors: MoeMoeFish moemoefish@qq.com
 * @LastEditTime: 2023-03-02 01:08:02
 * @Description: q00023
 */

mod list_node;
use list_node::ListNode;

#[derive(Debug)]
struct User {
    active: bool,
    name: String,
    age: i32,
}

pub(crate) fn demo() {
    // let list: Vec<Option<Box<ListNode>>> = vec!([vec!([1,4,5]),vec!([1,3,4]),vec!([2,6])]);
    println!("q00023");

    let u1 = User {
        active: true,
        name: String::from("Bob"),
        age: 3,
    };

    let u2 = User {
        active: u1.active,
        name: String::from("Tom"),
        age: 10,
    };

    let s = String::from("中国人");
    let s1 = s.clone();
    let s2 = String::from("中国人");
    let s3 = String::from("国人");

    assert_eq!(s, s1);
    assert_eq!(s, s2);
    assert_eq!(s, s3);

    println!("u1: {:?}, u2: {:?}", u1, u2);
}


