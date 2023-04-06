/*
 * @Author: MoeMoeFish moemoefish@qq.com
 * @Date: 2023-04-05 10:16:33
 * @LastEditors: MoeMoeFish moemoefish@qq.com
 * @LastEditTime: 2023-04-05 10:25:13
 * @Description: q00025
 */
use crate::libs::list_node;

mod solution1;

pub (crate) fn demo() {
    // let input = vec![vec![1,4,5],vec![1,3,4],vec![2,6]];

    let input = vec![1,2,3,4,5];
    let k = 2;

    let ret1 = solution1::Solution::reverse_k_group(list_node::convert_vec_to_linked_list(input), k);

    println!("q00025 solution1. {:?}", ret1);
}