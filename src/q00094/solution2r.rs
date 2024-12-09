use crate::utils::tree_node::TreeNode;
use std::borrow::Borrow;
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut ret: Vec<i32> = vec![];

        let mut curr = root;

        loop {
            if curr.is_none() {
                break;
            }

            if let Some(ref rc) = curr.clone() {
                let node  = rc.as_ref().borrow();

                if node.left.is_none() {
                    ret.push(node.val);

                    if node.right.is_none() {
                        break;
                    }

                    curr = rc.as_ref().borrow().right.clone();
                }
                
            }
        }
        return ret;
    }
}

pub(crate) struct Solution;
