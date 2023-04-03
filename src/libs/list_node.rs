use std::cmp::Ordering;

/*
 * @Author: moemoefish moemoefish@qq.com
 * @Date: 2023-04-03 14:13:02
 * @LastEditors: moemoefish moemoefish@qq.com
 * @LastEditTime: 2023-04-03 19:52:54
 * @Description: leetcode standard ListNode
 */
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    pub fn new(val: i32) -> ListNode {
        ListNode {
            val,
            next: None
        }
    }
}

pub fn cmp_list_node(node1: &ListNode, node2: &ListNode) -> Ordering {
    node1.val.cmp(&node2.val)
}

pub fn convert_vec_to_linked_list(val: Vec<i32>) -> Option<Box<ListNode>>  {
    let mut head = None;

    for &val in val.iter().rev() {
        let mut node = ListNode::new(val);
        node.next = head;
        head = Some(Box::new(node));
    }

    head
}

pub fn convert_vec_to_linked_list_list(val: Vec<Vec<i32>>) -> Vec<Option<Box<ListNode>>> {
    let mut lists: Vec<Option<Box<ListNode>>> = vec![];

    for item in val {
        let node = convert_vec_to_linked_list(item);
        lists.push(node);
    }

    lists
}

pub fn convert_list_node_to_vec(node: Option<Box<ListNode>>) -> Vec<i32> {
    let mut node_ref = &node;

    let mut ret: Vec<i32> = vec!();

    loop {
        if let Some(v) = node_ref {
            ret.push(v.val);
            node_ref = &v.next;
        } else {
            break;
        }
    }

    ret
}