/*
 * @Author: moemoefish moemoefish@qq.com
 * @Date: 2023-02-27 14:04:14
 * @LastEditors: moemoefish moemoefish@qq.com
 * @LastEditTime: 2023-03-23 19:12:06
 * @Description: 类似归并排序的方法，先排序最下一层，在一层一层向上逐渐排序
 */


use super::list_node::ListNode;

use std::collections::BinaryHeap;
use std::cmp::{Ord, Ordering, PartialEq};

impl Ord for ListNode {
    fn cmp(&self, other: &Self) -> Ordering {
        // 默认是最大堆，这里颠倒顺序，实现最小堆。
        other.val.cmp(&self.val)
    }
}

impl PartialOrd for ListNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let l = lists.len();
        let mut node_lists = lists.to_vec();
        Solution::merge_k_lists_recur(node_lists)
        
    }

    fn merge_k_lists_recur(mut lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let len = lists.len();

        if len == 0 {
            return None;
        } 

        if len == 1 {
            return lists.pop().unwrap();
        }

        let mut left = None;
        let mut right = None;


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

        let mut head = ListNode::new(0);

        while left != None || right != None {
            head.next = right;
            right.unwrap().next = None;
        }

        todo!()
        
    }
}

pub(crate) struct Solution {}