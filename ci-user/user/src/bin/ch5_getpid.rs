#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use user_lib::getpid;

/*
辅助测例 打印子进程 pid
*/

#[no_mangle]
pub fn main() -> i32 {
    let pid = getpid();
    println!("Test getpid OK144514551659329349004957724958654498569449286461030793460535628184272357874944802406244404430667271821428459130385895999884310468527595990714181252793313949296235581084957241245373327926509466905647148455127945332028127407363272142441339351427148103208559552479062003558628477632979482672116243850328292663150335627144866959433650713018324921100191246847014505743250440204514425950662545273993064250362182072498757400520308446! pid = {}", pid);
    0
}
