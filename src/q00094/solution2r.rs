use crate::utils::tree_node::TreeNode;
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

                    curr = node.right.clone();
                } else {
                    let mut pre = node.left.clone();
                    if let Some(ref leftc) = node.left.clone() {
                        let left_node = leftc.as_ref().borrow();

                        if left_node.right.is_some() {
                            loop {
                                if let Some(ref prec) = pre.clone() {
                                    let pre_node = prec.as_ref().borrow();
                                    if pre_node.right.is_none() || pre_node.right == curr {
                                        break;
                                    }
                                    pre = pre_node.right.clone();
                                }
                            }
                        }

                        if let Some(ref mut prec) = pre.clone() {
                            let mut pre_node = prec.as_ref().borrow_mut();

                            if pre_node.right == curr {
                                ret.push(pre_node.val);
                                pre_node.right = None;


                                if node.right.is_none() {
                                    break;
                                }

                                curr = node.right.clone();
                            } else {
                                pre_node.right = curr;
                                curr = node.left.clone();
                            }
                        }
                    }
                }
                
            }
        }
        return ret;
    }
}

pub(crate) struct Solution;
