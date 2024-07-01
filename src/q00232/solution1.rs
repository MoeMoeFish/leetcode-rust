/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
use std::collections::VecDeque;
impl MyQueue {
    pub(super) fn new() -> Self {
        MyQueue {
            front: VecDeque::new(),
            back: VecDeque::new(),
        }
    }
    
    pub(super) fn push(&mut self, x: i32) {
        self.back.push_back(x);
    }
    
    pub(super) fn pop(&mut self) -> i32 {
        if !self.front.is_empty() {
            self.front.pop_back().unwrap()
        } else {
            while !self.back.is_empty() {
                self.front.push_back(self.back.pop_back().unwrap());
            }

            self.front.pop_back().unwrap()
        }
    }
    
    pub(super) fn peek(&mut self) -> i32 {
        if !self.front.is_empty() {
            self.front.back().unwrap().clone()
        } else {
            while !self.back.is_empty() {
                self.front.push_back(self.back.pop_back().unwrap());
            }

            self.front.back().unwrap().clone()
        }
    }
    
    pub(super) fn empty(&self) -> bool {
        self.front.is_empty() && self.back.is_empty()
    }
}

pub(crate) struct MyQueue {
    front: VecDeque<i32>,
    back: VecDeque<i32>,
}