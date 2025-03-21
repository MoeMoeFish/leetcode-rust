use std::cell::RefCell;
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
                    let left_node = inner_node.left.clone();
                    let mut pre = left_node.clone();

                    loop {
                        if let Some(ref pre_node) = pre.clone() {
                            let right = pre_node.as_ref().borrow().right.clone();
                            if right.is_some() && right != curr {
                                pre = right;
                            } else {
                                break;
                            }
                        } else {
                            unreachable!();
                        }
                    }

                    if let Some(ref pre_node) = pre.clone() {
                        let right = pre_node.as_ref().borrow().right.clone();

                        if right.is_none() {
                            pre_node.as_ref().borrow_mut().right = curr.clone();
                            curr = left_node;
                        } else if right == curr {
                            result.push(inner_node.val);
                            pre_node.as_ref().borrow_mut().right = None;

                            if inner_node.right.is_some() {
                                curr = inner_node.right.clone();
                            } else {
                                break;
                            }
                        }

                    } else {
                        unreachable!();
                    }
                }
            } else {
                unreachable!();
            }
        }

        result
    }
}

pub(crate) struct Solution;
