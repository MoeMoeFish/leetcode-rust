/*
 * @Author: ytyu
 * @Date: 2022-06-30 13:30:30
 * @LastEditors: MoeMoeFish moemoefish@qq.com
 * @LastEditTime: 2023-01-29 13:43:57
 * @Description: 
 */
mod q00241;
mod q00417;
mod q00648;
mod q00902;
mod q01217;
mod q01424;
mod q02190;
mod q00004;

fn main() {
    println!("rust leetcode 刷题指南");

    run_demo();
}

// 为了编译不报错
fn run_demo() {
    q00004::demo();
    q00241::demo();
    q00417::demo();
    q00648::demo();
    q00902::demo();
    q01217::demo();
    q01424::demo();
    q02190::demo();
}
