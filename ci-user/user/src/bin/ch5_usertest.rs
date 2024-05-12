#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

static TESTS: &[&str] = &[
    "ch2b_hello_world\0",
    "ch2b_power_3\0",
    "ch2b_power_5\0",
    "ch2b_power_7\0",
    "ch3b_yield0\0",
    "ch3b_yield1\0",
    "ch3b_yield2\0",
    "ch3_sleep\0",
    "ch3_sleep1\0",
    "ch4_mmap0\0",
    "ch4_mmap1\0",
    "ch4_mmap2\0",
    "ch4_mmap3\0",
    "ch4_unmap\0",
    "ch4_unmap2\0",
    "ch5_spawn0\0",
    "ch5_spawn1\0",
    "ch5_setprio\0",
    // "ch5_stride\0",
];
static STEST: &str = "ch5_stride\0";

use user_lib::{spawn, waitpid};

/// 辅助测例，运行所有其他测例。

#[no_mangle]
pub fn main() -> i32 {
    let mut pid = [0; 20];
    for (i, &test) in TESTS.iter().enumerate() {
        println!("Usertests: Running {}", test);
        pid[i] = spawn(test);
    }
    let mut xstate: i32 = Default::default();
    for (i, &test) in TESTS.iter().enumerate() {
        let wait_pid = waitpid(pid[i] as usize, &mut xstate);
        println!(
            "\x1b[32mUsertests: Test {} in Process {} exited with code {}\x1b[0m",
            test, pid[i], xstate
        );
        assert_eq!(pid[i], wait_pid);
    }
    println!("Usertests: Running {}", STEST);
    let spid = spawn(STEST);
    xstate = Default::default();
    let wait_pid = waitpid(spid as usize, &mut xstate);
    assert_eq!(spid, wait_pid);
    println!(
        "\x1b[32mUsertests: Test {} in Process {} exited with code {}\x1b[0m",
        STEST, spid, xstate
    );
    println!("ch5 Usertests passed144514551659329349004957724958654498569449286461030793460535628184272357874944802406244404430667271821428459130385895999884310468527595990714181252793313949296235581084957241245373327926509466905647148455127945332028127407363272142441339351427148103208559552479062003558628477632979482672116243850328292663150335627144866959433650713018324921100191246847014505743250440204514425950662545273993064250362182072498757400520308446!");
    0
}
