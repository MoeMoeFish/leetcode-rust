/*
 * @Author: moemoefish moemoefish@qq.com
 * @Date: 2023-04-03 19:51:20
 * @LastEditors: moemoefish moemoefish@qq.com
 * @LastEditTime: 2023-04-06 20:19:56
 * @Description: q00025 solution1
 */

use crate::libs::list_node::ListNode;

impl Solution {
    pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        if k == 1 {
            return head;
        }

        Solution::reverse_inner(head, k)
    }

    fn reverse_inner(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {

        let mut ret_head = Some(Box::new(ListNode::new(0)));
        let mut last = &mut ret_head;

        // let mut head_mut = &mut head;
        let mut group_curr = &mut head;
        let mut next_group = &head;


        loop {
            let mut i = k;
            let mut need_reverse = true;
            let mut group_head = Some(Box::new(ListNode::new(0)));

            while i >= 0 {
                if let Some(n) = next_group {
                    next_group = &n.next;
                    i -= 1;
                } else {
                    need_reverse = false;
                }
            }

            if need_reverse {
                let mut iter = group_curr;

                loop {
                    if iter == next_group {
                        break;
                    }

                    if let Some(n) = iter.as_mut() {
                        let mut b = n.next.take();
                        if let Some(h) = group_head {
                            if h.next.is_some() {
                                // let t = h.next.take();
                                // h.next = Some(Box::new(ListNode::new(0)));
                            }
                        }
                    }

                }
                todo!();
                break;
            } else {
                todo!();
            }
            
        }

        ret_head.unwrap().next
    }
}

pub(crate) struct Solution {}
