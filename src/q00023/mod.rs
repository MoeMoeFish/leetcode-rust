/*
 * @Author: MoeMoeFish moemoefish@qq.com
 * @Date: 2023-02-17 13:56:36
 * @LastEditors: moemoefish moemoefish@qq.com
 * @LastEditTime: 2023-02-28 13:48:48
 * @Description: q00023
 * https://leetcode.cn/problems/merge-k-sorted-lists/
 */

mod listNode;
mod solution1;
use listNode::ListNode;


fn convert_vec_to_linked_list(val: Vec<i32>) -> Option<Box<ListNode>>  {
    let mut head = None;

    for &val in val.iter().rev() {
        let mut node = ListNode::new(val);
        node.next = head;
        head = Some(Box::new(node));
    }

    head
}

fn convert_vec_to_linked_list_list(val: Vec<Vec<i32>>) -> Vec<Option<Box<ListNode>>> {
    let mut lists: Vec<Option<Box<ListNode>>> = vec![];

    for item in val {
        let node = convert_vec_to_linked_list(item);
        lists.push(node);
    }

    lists
}

pub(crate) fn demo() {
    let input = vec![vec![1,4,5],vec![1,3,4],vec![2,6]];
    let lists: Vec<Option<Box<ListNode>>> = convert_vec_to_linked_list_list(input.clone());

    solution1::Solution::merge_k_lists(lists);




    println!("q00023");
}