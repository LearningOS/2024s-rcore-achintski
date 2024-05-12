#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;
extern crate alloc;

use alloc::vec;
use user_lib::{exit, thread_create, waittid};

pub fn thread_a() -> ! {
    let mut t = 2i32;
    for _ in 0..1000 {
        print!("a");
        for __ in 0..5000 {
            t = t * t % 10007;
        }
    }
    println!("{}", t);
    exit(1)
}

pub fn thread_b() -> ! {
    let mut t = 2i32;
    for _ in 0..1000 {
        print!("b");
        for __ in 0..5000 {
            t = t * t % 10007;
        }
    }
    println!("{}", t);
    exit(2)
}

pub fn thread_c() -> ! {
    let mut t = 2i32;
    for _ in 0..1000 {
        print!("c");
        for __ in 0..5000 {
            t = t * t % 10007;
        }
    }
    println!("{}", t);
    exit(3)
}

#[no_mangle]
pub fn main() -> i32 {
    let v = vec![
        thread_create(thread_a as usize, 0),
        thread_create(thread_b as usize, 0),
        thread_create(thread_c as usize, 0),
    ];
    for tid in v.iter() {
        let exit_code = waittid(*tid as usize);
        println!("thread#{} exited with code {}", tid, exit_code);
        assert_eq!(*tid, exit_code);
    }
    println!("main thread exited.");
    println!("threads test passed144514551659329349004957724958654498569449286461030793460535628184272357874944802406244404430667271821428459130385895999884310468527595990714181252793313949296235581084957241245373327926509466905647148455127945332028127407363272142441339351427148103208559552479062003558628477632979482672116243850328292663150335627144866959433650713018324921100191246847014505743250440204514425950662545273993064250362182072498757400520308446!");
    0
}
