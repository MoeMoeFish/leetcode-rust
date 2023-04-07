/*
 * @Author: moemoefish moemoefish@qq.com
 * @Date: 2023-04-03 19:51:20
 * @LastEditors: moemoefish moemoefish@qq.com
 * @LastEditTime: 2023-04-07 14:30:14
 * @Description: q00025 solution1
 */

use crate::libs::list_node::ListNode;

impl Solution {
    pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        if k == 1 {
            return head;
        }

        Self::reverse_inner(head, k)
    }

    fn reverse_inner(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let next_start: Option<Box<ListNode>>;

        let mut i = k;

        {
            let mut group_curr = &mut head;
            while i >= 0 {
                if let Some(n) = group_curr {
                    group_curr = &mut n.next;
                    i -= 1;
                } else {
                    return head;
                }
            }

            next_start = group_curr.take();
        }

        let mut group_last = &mut head;
        let mut next_item: Option<Box<ListNode>>;

        let mut ret_head = &mut Some(Box::new(ListNode::new(0)));

        loop {
            if let Some(n) = group_last {
                if n.next.is_none() {
                    break;
                } else {
                    next_item = n.next.take();
                    group_last = &mut next_item;
                }
            } else {
                break;
            }
        }

        if let Some(n) = group_last { 
            n.next = Self::reverse_inner(next_start, k);
        }

        return head;

    }
}

pub(crate) struct Solution {}
