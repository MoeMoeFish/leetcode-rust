use crate::utils::tree_node::TreeNode;

use std::borrow::Borrow;
use std::rc::Rc;
use std::cell::RefCell;

/**
 * 使用 morris 中序方式。
 * 对于当前节点有左节点的节点，查找左子树的最右节点...
 */
impl Solution {
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut ret = vec![];

        if root.is_none() {
            return ret;
        }
        let mut curr = root.unwrap();

        loop {
            let left = curr.as_ref().borrow().left.clone();

            if left.is_none() {
                ret.push(curr.as_ref().borrow().val);

                let right = curr.as_ref().borrow().right.clone();

                if right.is_none() {
                    break;
                }

                if let Some(v) = right {
                    curr = v;
                }
            }

            let mut pre = curr.as_ref().borrow().left.clone();

            while let Some(v) = pre {
                pre = v.as_ref().borrow().right.clone();
            }

            
        }

        return ret;
    }
}

pub(crate) struct Solution;