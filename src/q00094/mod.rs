use std::cell::RefCell;
use std::rc::Rc;

use crate::utils::tree_node::TreeNode;

mod solution1;
mod solution2;
mod solution2r;

pub (crate) fn demo() {
    let input : Vec<Option<i32>>= vec![Some(1), None, Some(2), Some(3)];

    {
        let tree_node = TreeNode::from_binary_tree_array(input.clone());
        let root = match tree_node {
            None => None,
            Some(v) => Some(Rc::new(RefCell::new(v))),
        };

        solution1::Solution::inorder_traversal(root);
    }
    {
        let tree_node = TreeNode::from_binary_tree_array(input.clone());
        let root = match tree_node {
            None => None,
            Some(v) => Some(Rc::new(RefCell::new(v))),
        };

        solution2::Solution::inorder_traversal(root);
    }
    {
        let tree_node = TreeNode::from_binary_tree_array(input.clone());
        let root = match tree_node {
            None => None,
            Some(v) => Some(Rc::new(RefCell::new(v))),
        };

        solution2r::Solution::inorder_traversal(root);
    }
}

#[cfg(test)]
mod test {
    use std::cell::RefCell;
    use std::rc::Rc;

    use crate::utils::tree_node::TreeNode;
    use crate::utils::string_utils::string_to_option_int_vec;
    use super::solution1;
    use super::solution2r;

    fn inner_test(input: &str, expected: Vec<i32>) {
        let input_vec = string_to_option_int_vec(input);

        let tree_node_1 = TreeNode::from_leetcode_array(input_vec.clone());
        assert_eq!(expected, solution1::Solution::inorder_traversal(tree_node_1));


        let tree_node_2 = TreeNode::from_leetcode_array(input_vec.clone());
        assert_eq!(expected, solution2r::Solution::inorder_traversal(tree_node_2));
    }

    #[test]
    fn test1() {
        let input = "1,null,2,3";
        let expected = vec![1,3,2];

        inner_test(input, expected);
    }

    #[test]
    fn test2() {
        let input = "1,null,2,3";
        let expected = vec![1,3,2];

        inner_test(input, expected);
    }

    #[test]
    fn test_rc() {
        let rc1 = Rc::new(RefCell::new(TreeNode::new(1)));
        let rc2 = rc1.clone();

        assert_eq!(rc1, rc2);
    }
}
