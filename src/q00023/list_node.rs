/*
 * @Author: MoeMoeFish moemoefish@qq.com
 * @Date: 2023-03-02 00:53:55
 * @LastEditors: moemoefish moemoefish@qq.com
 * @LastEditTime: 2023-03-24 19:20:17
 * @Description: 
 */
use core::mem;

#[derive(PartialEq, Eq, Debug, Clone)]
pub(crate) struct ListNode {
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

    pub fn pop_next(&mut self) -> Option<Box<ListNode>> {
        let mut ret: Option<Box<ListNode>> = None;
        mem::swap(&mut self.next, &mut ret);

        return ret;
    }
}
