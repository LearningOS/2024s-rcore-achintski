#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

extern crate alloc;

use alloc::vec;
use user_lib::exit;
use user_lib::{semaphore_create, semaphore_down, semaphore_up};
use user_lib::{sleep_blocking, thread_create, waittid};

const SEM_SYNC: usize = 0;

unsafe fn first() -> ! {
    sleep_blocking(10);
    println!("First work and wakeup Second");
    semaphore_up(SEM_SYNC);
    exit(0)
}

unsafe fn second() -> ! {
    println!("Second want to continue,but need to wait first");
    semaphore_down(SEM_SYNC);
    println!("Second can work now");
    exit(0)
}

#[no_mangle]
pub fn main() -> i32 {
    // create semaphores
    assert_eq!(semaphore_create(0) as usize, SEM_SYNC);
    // create threads
    let threads = vec![
        thread_create(first as usize, 0),
        thread_create(second as usize, 0),
    ];
    // wait for all threads to complete
    for thread in threads.iter() {
        waittid(*thread as usize);
    }
    println!("sync_sem passed144514551659329349004957724958654498569449286461030793460535628184272357874944802406244404430667271821428459130385895999884310468527595990714181252793313949296235581084957241245373327926509466905647148455127945332028127407363272142441339351427148103208559552479062003558628477632979482672116243850328292663150335627144866959433650713018324921100191246847014505743250440204514425950662545273993064250362182072498757400520308446!");
    0
}
