use crate::utils::tree_node::TreeNode;

use std::rc::Rc;
use std::cell::RefCell;

/*
 * 通过 stack 记录已经遍历过的节点（中序节点）
 */
impl Solution {
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut stack: Vec<Rc<RefCell<TreeNode>>> = vec![];
        let mut ret: Vec<i32> = vec![];

        if let Some(v) = root {
            let mut curr = v.clone();
            let mut is_return = false;

            loop {
                let m_curr = curr.clone();
                let r_left = &m_curr.as_ref().borrow().left;
                if !is_return && r_left.is_some() {
                    if let Some(left) = r_left {
                        stack.push(curr.clone());
                        curr = left.clone();
                        is_return = false;
                        continue;
                    }
                }

                ret.push(m_curr.as_ref().borrow().val);

                if let Some(right) = &m_curr.as_ref().borrow().right {
                    curr = right.clone();
                    is_return = false;
                    continue;
                }

                if !stack.is_empty() {
                    is_return = true;
                    curr = stack.pop().unwrap();
                    continue;
                } 

                break;
            }
            return ret;
        } else {
            return vec![];
        }
    }
}

pub(crate) struct Solution;