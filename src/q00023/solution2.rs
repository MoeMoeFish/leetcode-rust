/*
 * @Author: moemoefish moemoefish@qq.com
 * @Date: 2023-03-31 12:55:03
 * @LastEditors: moemoefish moemoefish@qq.com
 * @LastEditTime: 2023-03-31 19:58:50
 * @Description: 使用堆排进行排序，
 */

use super::list_node::ListNode;
use std::cmp::{Ord, Ordering };

impl Ord for ListNode {
    fn cmp(&self, other: &Self) -> Ordering {
        // 默认是最大堆，这里颠倒顺序，实现最小堆。
        other.val.cmp(&self.val)
        // self.val.cmp(&other.val)
    }
}

impl PartialOrd for ListNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct Heap {
    buf: Vec<Option<Box<ListNode>>>,
}

impl Heap {
    fn new() -> Self {
        Self { buf: vec!() }
    }

    fn heapify(&mut self, i: usize) {
        let len = self.buf.len();

        let parent = i;
        let mut son = parent * 2 + 1;

        if son >= len {
            return;
        }

        if son + 1 < len && self.buf[son] < self.buf[son+1] {
            son = son + 1;
        }

        if self.buf[parent] < self.buf[son] {
            self.buf.swap(parent, son);
        }

        self.heapify(son);
    }

    fn init(&mut self, vec: Vec<Option<Box<ListNode>>>) {
        self.buf = vec;

        for i in (0..(self.buf.len() / 2) + 1).rev() {
            self.heapify(i);
        }
    }

    fn sort(&mut self) -> Option<Box<ListNode>> {
        let mut head = Some(Box::new(ListNode::new(1)));
        let mut head_ref = &mut head;

        while !self.buf.is_empty() {
            let last = self.buf.len() - 1;
            self.buf.swap(0, last);
            if let Some(b) = head_ref {
                b.next = self.buf.pop().unwrap();
                head_ref = &mut b.next;
            }
            self.heapify(0);
        }

        head.unwrap().next
    }
}

impl Solution {
    pub fn merge_k_lists(mut lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut flat_vec: Vec<Option<Box<ListNode>>> = vec![];
        
        loop {
            if lists.len() == 0 {
                break;
            }

            let mut head = lists.pop().unwrap();

            loop {
                if head == None {
                    break;
                }

                if let Some(h_node) = head.as_mut() {
                    let next = h_node.next.take();
                    h_node.next = None;
                    flat_vec.push(head);
                    head = next;
                }
            }
        }

        let mut heap = Heap::new();
        heap.init(flat_vec);

        heap.sort()
    }
}



pub(crate) struct Solution {}