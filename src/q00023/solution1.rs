/*
 * @Author: moemoefish moemoefish@qq.com
 * @Date: 2023-02-27 14:04:14
 * @LastEditors: moemoefish moemoefish@qq.com
 * @LastEditTime: 2023-03-01 13:56:23
 * @Description: 类似归并排序的方法，先排序最下一层，在一层一层向上逐渐排序
 */

use std::ops::Deref;

use super::listNode::ListNode;


impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let l = lists.len();
        Solution::merge_k_lists_recur(lists.as_mut(), 0, l - 1)
    }

    fn merge_k_lists_recur(lists: &mut Vec<Option<Box<ListNode>>>, start: usize, end: usize) -> Option<Box<ListNode>> {
        if start <= end {
            let b = &lists[start];
            lists[start] = None;
        }
        todo!()
        
    }
}

pub(crate) struct Solution {}