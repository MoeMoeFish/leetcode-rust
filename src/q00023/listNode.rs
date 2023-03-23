/*
 * @Author: moemoefish moemoefish@qq.com
 * @Date: 2023-02-27 12:50:41
 * @LastEditors: moemoefish moemoefish@qq.com
 * @LastEditTime: 2023-03-02 12:45:36
 * @Description: 
 */
#[derive(PartialEq, Eq, Debug, Clone)]
pub(crate) struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    pub fn new(val: i32) -> Self {
        Self {
            val,
            next: None
        }
    }
}
