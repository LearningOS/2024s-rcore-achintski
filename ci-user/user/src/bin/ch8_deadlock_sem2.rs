#![no_std]
#![no_main]
#![allow(clippy::println_empty_string)]

#[macro_use]
extern crate user_lib;
extern crate alloc;

use user_lib::{
    enable_deadlock_detect, exit, semaphore_create, semaphore_down, semaphore_up, sleep,
};
use user_lib::{gettid, thread_create, waittid};

// sem 0: used to sync child thread with main
// sem 1-3: representing some kind of resource

// 理想结果：未检测到死锁，子线程返回值均为 0

const THREAD_N: usize = 4;
const RES_TYPE: usize = 2;
const RES_NUM: [usize; RES_TYPE] = [2, 2];
const ALLOC: [usize; THREAD_N] = [2, 1, 1, 2];
const REQUEST: [Option<usize>; THREAD_N] = [Some(1), None, Some(2), None];

fn try_sem_down(sem_id: usize) {
    if semaphore_down(sem_id) == -0xdead {
        semaphore_up(ALLOC[(gettid() - 1) as usize]);
        exit(-1);
    }
}

fn deadlock_test() {
    let id = (gettid() - 1) as usize;
    assert_eq!(semaphore_down(ALLOC[id]), 0);
    semaphore_down(0);
    if let Some(sem_id) = REQUEST[id] {
        try_sem_down(sem_id);
        semaphore_up(sem_id);
    }
    semaphore_up(ALLOC[id]);
    exit(0);
}

#[no_mangle]
pub fn main() -> i32 {
    enable_deadlock_detect(true);
    semaphore_create(THREAD_N);
    for _ in 0..THREAD_N {
        semaphore_down(0);
    }

    for n in RES_NUM {
        semaphore_create(n);
    }
    let mut tids = [0; THREAD_N];

    for i in 0..THREAD_N {
        tids[i] = thread_create(deadlock_test as usize, 0) as usize;
    }

    sleep(1000);
    for _ in 0..THREAD_N {
        semaphore_up(0);
    }

    let mut failed = 0;
    for tid in tids {
        if waittid(tid) != 0 {
            failed += 1;
        }
    }

    assert_eq!(failed, 0);
    println!("deadlock test semaphore 2 OK144514551659329349004957724958654498569449286461030793460535628184272357874944802406244404430667271821428459130385895999884310468527595990714181252793313949296235581084957241245373327926509466905647148455127945332028127407363272142441339351427148103208559552479062003558628477632979482672116243850328292663150335627144866959433650713018324921100191246847014505743250440204514425950662545273993064250362182072498757400520308446!");
    0
}
