/*
 * @Author: moemoefish moemoefish@qq.com
 * @Date: 2023-02-27 14:04:14
 * @LastEditors: MoeMoeFish moemoefish@qq.com
 * @LastEditTime: 2023-03-25 01:19:50
 * @Description: 类似归并排序的方法，先排序最下一层，在一层一层向上逐渐排序
 */

use super::list_node::ListNode;
use std::cmp::{Ord, Ordering, PartialEq};

impl Ord for ListNode {
    fn cmp(&self, other: &Self) -> Ordering {
        // 默认是最大堆，这里颠倒顺序，实现最小堆。
        // other.val.cmp(&self.val)
        self.val.cmp(&other.val)
    }
}

impl PartialOrd for ListNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let node_lists = lists.to_vec();
        Solution::merge_k_lists_recur(node_lists)
    }

    fn merge_k_lists_recur(mut lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let len = lists.len();

        println!("merge_k_lists_recur: {:?}", len);

        if len == 0 {
            return None;
        } 

        if len == 1 {
            return lists.pop().unwrap();
        }

        let mut left: Option<Box<ListNode>>;
        let mut right: Option<Box<ListNode>>;

        if len == 2 {
            left = lists.pop().unwrap();
            right = lists.pop().unwrap();
        } else {
            let mut middle = len / 2;
            let mut other = len - middle;

            let mut left_list: Vec<Option<Box<ListNode>>> = vec![];
            let mut right_list: Vec<Option<Box<ListNode>>> = vec![];

            while middle > 0 {
                left_list.push(lists.pop().unwrap());
                middle -= 1;
            }

            while other > 0 {
                right_list.push(lists.pop().unwrap());
                other -= 1;
            }

            left = Solution::merge_k_lists_recur(left_list);
            right = Solution::merge_k_lists_recur(right_list);
        }

        let mut head: Option<Box<ListNode>> = Some(Box::new(ListNode::new(0)));
        let mut curr = &mut head;

        while left != None && right != None {
            println!("left: {:?}, right: {:?}, left < right: {:?}", left, right, left < right);
            if left < right {
                let left_next = match left.as_mut() {
                    Some(b) => b.pop_next(),
                    None => None,
                };

                if let Some(ln) = curr {
                    ln.next = left;
                    curr = &mut ln.next;
                }
                left = left_next;
            } else {
                let right_next = match right.as_mut() {
                    Some(b) => b.pop_next(),
                    None => None,
                };

                if let Some(ln) = curr {
                    ln.next = right;
                    curr = &mut ln.next;
        }

                right = right_next;
            }
        }

        if left != None {
            if let Some(ln) = curr {
                ln.next = left;
            }
        }

        if right != None {
            if let Some(ln) = curr {
                ln.next = right;
            }
        }
        
        head.unwrap().next
    }
}

pub(crate) struct Solution {}