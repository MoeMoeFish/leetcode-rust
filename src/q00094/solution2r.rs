use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

use crate::utils::tree_node::TreeNode;


impl Solution {
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result: Vec<i32> = vec![];

        if root.is_none() {
            return result;
        }

        let mut curr = root;

        loop {
            if let Some(curr_node) = curr.clone() {
                let inner_node = curr_node.as_ref().borrow();

                if inner_node.left.is_none() {
                    result.push(inner_node.val);

                    if inner_node.right.is_none() {
                        break;
                    } else {
                        curr = inner_node.right.clone();
                        continue;
                    }
                } else {
                    let mut pre = curr_node.clone().as_ref().borrow().left; 

                    loop {
                        if let Some(pre_node) = pre.clone() {
                            
                        }
                    }

                }
            } else {
                break;
            }
        }

        result
    }
}

pub(crate) struct Solution;