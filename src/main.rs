/*
 * @Author: ytyu
 * @Date: 2022-06-30 13:30:30
 * @LastEditors: MoeMoeFish moemoefish@qq.com
 * @LastEditTime: 2023-04-06 11:03:55
 * @Description: 
 */
use std::env;

mod q00004;
mod q00010;
mod q00023;
mod q00025;
mod q00241;
mod q00417;
mod q00648;
mod q00902;
mod q01217;
mod q01424;
mod q02190;

pub mod libs;

fn main() {
    println!("rust leetcode 刷题指南");
    let args: Vec<_> = env::args().collect();
    println!("{:?}", args);

    if args.len() > 1 && &args[1] == "--all" {
        run_demo();
    } else {
        run_this();
    }
}

// 为了编译不报错
fn run_demo() {
    q00004::demo();
    q00010::demo();
    q00023::demo();
    q00025::demo();
    q00241::demo();
    q00417::demo();
    q00648::demo();
    q00902::demo();
    q01217::demo();
    q01424::demo();
    q02190::demo();
}

// 为了在完成某个题目时，不用执行其他的
fn run_this() {
    q00025::demo();
}
