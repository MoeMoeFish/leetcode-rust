/*
 * @Author: ytyu
 * @Date: 2022-06-30 13:30:30
 * @LastEditors: MoeMoeFish moemoefish@qq.com
 * @LastEditTime: 2022-10-18 13:46:34
 * @Description: 
 */
mod q00241;
mod q00648;
mod q01217;
mod q00417;
mod q02190;
mod q00902;

fn main() {
    println!("rust leetcode 刷题指南");

    run_demo();
}

// 为了编译不报错
fn run_demo() {
    q00241::demo();
    q00417::demo();
    q00648::demo();
    q01217::demo();
    q02190::demo();
    q00902::demo();
}
