use crate::utils::tree_node::TreeNode;
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

impl Solution {
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut ret: Vec<i32> = vec![];

        let mut curr = root;

        loop {
            if curr.is_none() {
                break;
            }

            let a = curr.clone().unwrap();

            if a.borrow().left.is_none() {
                ret.push(a.borrow().val);

                if a.borrow().right.is_none() {
                    break;
                }

                curr = a.borrow().right.clone();
                continue;
            }
        }


        return ret;
    }
}

pub(crate) struct Solution;
