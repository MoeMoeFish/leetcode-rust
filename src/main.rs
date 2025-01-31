/*
 * @Author: ytyu
 * @Date: 2022-06-30 13:30:30
 * @LastEditors: MoeMoeFish moemoefish@qq.com
 * @LastEditTime: 2023-03-04 00:50:19
 * @Description: 
 */
mod utils;

mod q00001;
mod q00004;
mod q00010;
mod q00020;
mod q00023;
mod q00048;
mod q00054;
mod q00073;
mod q00094;
mod q00232;
mod q00241;
mod q00289;
mod q00316;
mod q00417;
mod q00648;
mod q00902;
mod q01217;
mod q01424;
mod q02190;

fn main() {
    println!("rust leetcode 刷题指南");

    run_demo();
    // run_this();
}

// 为了编译不报错
fn run_demo() {
    q00001::demo();
    q00004::demo();
    q00010::demo();
    q00020::demo();
    q00023::demo();
    q00048::demo();
    q00054::demo();
    q00073::demo();
    q00094::demo();
    q00232::demo();
    q00241::demo();
    q00289::demo();
    q00316::demo();
    q00417::demo();
    q00648::demo();
    q00902::demo();
    q01217::demo();
    q01424::demo();
    q02190::demo();
}

// 为了在完成某个题目时，不用编译其他题目
// fn run_this() {
//     q00023::demo();
// }
