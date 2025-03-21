use crate::utils::tree_node::TreeNode;

use std::rc::Rc;
use std::cell::RefCell;

/**
 * 使用 morris 中序方式。
 * 对于当前节点有左节点的节点，查找左子树的最右节点...
 */
impl Solution {
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut ret: Vec<i32> = Vec::new();

        let mut curr = root;

        while curr.is_some() {
            if let Some(node) = curr.clone() {
                let ref_node = node.as_ref().borrow();

                if ref_node.left.is_none() {
                    ret.push(ref_node.val);
                    if let Some(r_node) = ref_node.right.as_ref() {
                        curr = Some(r_node.clone());
                    } else {
                        curr = None;
                    }
                    continue;
                } else {
                    if let Some(left) = ref_node.left.as_ref() {
                        let mut pre_ref = Some(left.clone());

                        loop {
                            if let Some(pre_node) = pre_ref.clone() {
                                if pre_node.as_ref().borrow().right == curr {
                                    break;
                                }

                                if pre_node.as_ref().borrow().right.is_none() {
                                    pre_node.as_ref().borrow_mut().right = curr;
                                    curr = ref_node.left.clone();
                                    break;
                                }

                                pre_ref = pre_node.as_ref().borrow().right.clone();
                            } else {
                                break;
                            }
                        }
                    }
                }
            }
        }

        ret
    }
}

pub(crate) struct Solution;