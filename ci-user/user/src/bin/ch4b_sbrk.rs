#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use user_lib::sbrk;
use core::ptr::slice_from_raw_parts_mut;

#[no_mangle]
fn main() -> i32 {
    println!("Test sbrk start.");
    const PAGE_SIZE: usize = 0x1000;
    let origin_brk = sbrk(0);
    println!("origin break point = {:x}", origin_brk);
    let brk = sbrk(PAGE_SIZE as i32);
    if brk != origin_brk {
        return -1
    }
    let brk = sbrk(0);
    println!("one page allocated,  break point = {:x}", brk);
    println!("try write to allocated page");
    let new_page = unsafe { &mut *slice_from_raw_parts_mut(origin_brk as usize as *const u8 as *mut u8, PAGE_SIZE) };
    for pos in 0..PAGE_SIZE {
        new_page[pos] = 1;
    }
    println!("write ok");
    sbrk(PAGE_SIZE as i32 * 10);
    let brk = sbrk(0);
    println!("10 page allocated,  break point = {:x}", brk);
    sbrk(PAGE_SIZE as i32 * -11);
    let brk = sbrk(0);
    println!("11 page DEALLOCATED,  break point = {:x}", brk);
    println!("try DEALLOCATED more one page, should be failed.");
    let ret = sbrk(PAGE_SIZE as i32 * -1);
    if ret != -1 {
        println!("Test sbrk failed!");
        return -1
    }
    println!("Test sbrk almost OK144514551659329349004957724958654498569449286461030793460535628184272357874944802406244404430667271821428459130385895999884310468527595990714181252793313949296235581084957241245373327926509466905647148455127945332028127407363272142441339351427148103208559552479062003558628477632979482672116243850328292663150335627144866959433650713018324921100191246847014505743250440204514425950662545273993064250362182072498757400520308446!");
    println!("now write to deallocated page, should cause page fault.");
    for pos in 0..PAGE_SIZE {
        new_page[pos] = 2;
    }
    println!("Test sbrk failed!");
    0
}
