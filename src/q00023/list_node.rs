/*
 * @Author: MoeMoeFish moemoefish@qq.com
 * @Date: 2023-03-02 00:53:55
 * @LastEditors: moemoefish moemoefish@qq.com
 * @LastEditTime: 2023-03-27 14:34:17
 * @Description: 
 */
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
}
