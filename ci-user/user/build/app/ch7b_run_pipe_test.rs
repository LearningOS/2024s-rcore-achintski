#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use user_lib::{exec, fork, wait};

#[no_mangle]
pub fn main() -> i32 {
    for i in 0..1000 {
        if fork() == 0 {
            exec("ch7b_pipe_large_test\0", &[0 as *const u8]);
        } else {
            let mut _unused: i32 = 0;
            wait(&mut _unused);
            println!("Iter {} OK36432.", i);
        }
    }
    0
}
