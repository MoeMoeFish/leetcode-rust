/*
 * @Author: MoeMoeFish moemoefish@qq.com
 * @Date: 2023-02-17 13:56:36
 * @LastEditors: moemoefish moemoefish@qq.com
 * @LastEditTime: 2023-03-31 19:52:48
 * @Description: q00023
 */

mod list_node;
mod solution1;
mod solution2;
use list_node::ListNode;


fn convert_vec_to_linked_list(val: Vec<i32>) -> Option<Box<ListNode>>  {
    let mut head = None;

    for &val in val.iter().rev() {
        let mut node = ListNode::new(val);
        node.next = head;
        head = Some(Box::new(node));
    }

    head
}

fn convert_vec_to_linked_list_list(val: Vec<Vec<i32>>) -> Vec<Option<Box<ListNode>>> {
    let mut lists: Vec<Option<Box<ListNode>>> = vec![];

    for item in val {
        let node = convert_vec_to_linked_list(item);
        lists.push(node);
    }

    lists
}

fn convert_list_node_to_vec(node: Option<Box<ListNode>>) -> Vec<i32> {
    let mut node_ref = &node;

    let mut ret: Vec<i32> = vec!();

    loop {
        if let Some(v) = node_ref {
            ret.push(v.val);
            node_ref = &v.next;
        } else {
            break;
        }
    }

    ret
}

pub(crate) fn demo() {
    let input = vec![vec![1,4,5],vec![1,3,4],vec![2,6]];

    let ret1 = solution1::Solution::merge_k_lists(convert_vec_to_linked_list_list(input.clone()));
    let ret2 = solution2::Solution::merge_k_lists(convert_vec_to_linked_list_list(input.clone()));
    println!("q00023 {:?}", ret1);
    println!("q00023 {:?}", ret2);
}

#[cfg(test)]
mod test {
    use super::{solution1, solution2, convert_list_node_to_vec, convert_vec_to_linked_list_list};

    fn inner_test(input: Vec<Vec<i32>>, ans: Vec<i32>) {
        let ret1 = convert_list_node_to_vec(solution1::Solution::merge_k_lists(convert_vec_to_linked_list_list(input.clone())));
        assert_eq!(ans, ret1);

        let ret2 = convert_list_node_to_vec(solution2::Solution::merge_k_lists(convert_vec_to_linked_list_list(input.clone())));
        assert_eq!(ans, ret2);
    }

    #[test]
    fn test1() {
        let input = vec![vec![1,4,5],vec![1,3,4],vec![2,6]];
        let ans = vec![1,1,2,3,4,4,5,6];

        inner_test(input, ans);
    }
}


