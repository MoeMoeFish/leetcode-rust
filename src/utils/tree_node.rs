use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    pub(crate) fn new(val: i32) -> TreeNode {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
    pub(crate) fn from_leetcode_array(arr: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
        if arr.len() == 0 {
            return None;
        }

        let mut q: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();
        let root = Rc::new(RefCell::new(TreeNode::new(arr[0].unwrap())));
        let mut curr: Rc<RefCell<TreeNode>> = root.clone();

        let mut i = 1;

        while i < arr.len() {
            if arr[i].is_some() {
                let val = arr[i].unwrap();
                let left = Rc::new(RefCell::new(TreeNode::new(val)));
                curr.as_ref().borrow_mut().left = Some(left.clone());
                q.push_back(left.clone());
            }

            if i + 1 < arr.len() && arr[i + 1].is_some() {
                let val = arr[i + 1].unwrap();
                let right = Rc::new(RefCell::new(TreeNode::new(val)));
                curr.as_ref().borrow_mut().right = Some(right.clone());
                q.push_back(right.clone());
            }

            curr = q.pop_front().unwrap();
            i += 2;
        }

        return Some(root);
    }

    pub(crate) fn from_binary_tree_array(arr: Vec<Option<i32>>) -> Option<TreeNode> {
        if arr.len() == 0 {
            return None;
        }

        let node = TreeNode::from_array(&arr, 0);
        Some(node) 
    }

    fn from_array(arr: &Vec<Option<i32>>, pos: usize) -> TreeNode {
        let val = arr[pos].unwrap();

        let left_pos = pos * 2 + 1;
        let right_pos = pos * 2 + 2;

        let mut tree_node = TreeNode {
            val,
            left: None,
            right: None,
        };

        if left_pos < arr.len() && arr[left_pos].is_some()  {
            tree_node.left = Some(Rc::new(RefCell::new(TreeNode::from_array(arr, left_pos))));
        }

        if right_pos < arr.len() && arr[right_pos].is_some() {
            tree_node.right = Some(Rc::new(RefCell::new(TreeNode::from_array(arr, right_pos))));
        }

        return tree_node;
    }

}

#[cfg(test)]
mod test {
    use super::TreeNode;

    #[test]
    fn test_from_array_01() {
        let tree_node = TreeNode::from_array(&vec![Some(1)], 0);

        assert_eq!(tree_node.val, 1);
        assert_eq!(tree_node.left, None);
        assert_eq!(tree_node.right, None);
    }

    #[test]
    fn test_from_array_02() {
        let tree_node = TreeNode::from_array(&vec![Some(1), Some(2), Some(3)], 0);

        assert_eq!(tree_node.val, 1);
        assert_eq!(tree_node.left.unwrap().as_ref().borrow().val, 2);
        assert_eq!(tree_node.right.unwrap().as_ref().borrow().val, 3);
    }

    #[test]
    fn test_from_array_03() {
        let tree_node = TreeNode::from_array(&vec![Some(1), Some(2), Some(3), None, None, None, Some(7)], 0);
        assert_eq!(tree_node.val, 1);
        
        if let Some(v) = tree_node.left.as_ref() {
            let node = v.as_ref().borrow();
            assert_eq!(node.val, 2);
            assert_eq!(node.left, None);
            assert_eq!(node.right, None);
        } else {
            assert!(false, "no left value");
        }

        if let Some(v) = tree_node.right.as_ref() {
            let node = v.as_ref().borrow();
            assert_eq!(node.val, 3);
            assert_eq!(node.left, None);

            if let Some(u) = node.right.as_ref() {
                let last_node = u.as_ref().borrow();
                assert_eq!(last_node.val, 7)
            } else {
                assert!(false, "len 7 is not correct")
            }
        } else {
            assert!(false, "no left value");
        }
    }

    #[test]
    fn test_from_leetcode_array_01() {
        let arr_1: Vec<Option<i32>> = vec![];
        let tree_node_1 = TreeNode::from_leetcode_array(arr_1);
        assert!(tree_node_1.is_none());

        let arr_2: Vec<Option<i32>> = vec![Some(1)];
        let tree_node_2 = TreeNode::from_leetcode_array(arr_2);

        if let Some(v) = tree_node_2 {
            assert_eq!(v.borrow().val, 1);
            assert_eq!(v.borrow().left, None);
            assert_eq!(v.borrow().right, None);
        } else {
            assert!(false, "tree_node_2 should not None");
        }
    }
}
