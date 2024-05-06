//! Process management syscalls
use crate::{
    config::MAX_SYSCALL_NUM,
    task::{
        change_program_brk, exit_current_and_run_next, suspend_current_and_run_next, TaskStatus,
    },
    // lab1
    task::{update_syscall_times, init_task_info},
    timer::{get_time_us},
    syscall::{SYSCALL_YIELD, SYSCALL_GET_TIME, SYSCALL_TASK_INFO, SYSCALL_MMAP, SYSCALL_MUNMAP, SYSCALL_SBRK},
    // lab2
    mm::{frame_is_full, VirtAddr, MapPermission, translated_byte_t},
    task::{current_user_token, current_ms_mmap, current_ms_munmap},
};

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

/// Task information
#[allow(dead_code)]
pub struct TaskInfo {
    /// Task status in it's life cycle
    pub status: TaskStatus,
    /// The numbers of syscall called by task
    pub syscall_times: [u32; MAX_SYSCALL_NUM],
    /// Total running time of task
    pub time: usize,
}

/// task exits and submit an exit code
pub fn sys_exit(_exit_code: i32) -> ! {
    trace!("kernel: sys_exit");
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    trace!("kernel: sys_yield");
    // lab1
    update_syscall_times(SYSCALL_YIELD);

    suspend_current_and_run_next();
    0
}

/// YOUR JOB: get time with second and microsecond
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TimeVal`] is splitted by two pages ?
pub fn sys_get_time(_ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");
    // lab1
    update_syscall_times(SYSCALL_GET_TIME);

    let us = get_time_us();
    // lab2
    let ts = TimeVal {
        sec: us / 1_000_000,
        usec: us % 1_000_000,
    };
    translated_byte_t(current_user_token(), _ts, &ts);
    0
}

/// YOUR JOB: Finish sys_task_info to pass testcases
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TaskInfo`] is splitted by two pages ?
pub fn sys_task_info(_ti: *mut TaskInfo) -> isize {
    trace!("kernel: sys_task_info NOT IMPLEMENTED YET!");
    // lab1
    update_syscall_times(SYSCALL_TASK_INFO);
    if _ti.is_null() {
        return -1;
    }
    init_task_info(_ti);
    0
}

// YOUR JOB: Implement mmap.
pub fn sys_mmap(_start: usize, _len: usize, _port: usize) -> isize {
    trace!("kernel: sys_mmap NOT IMPLEMENTED YET!");
    // lab2
    update_syscall_times(SYSCALL_MMAP);
    // 排除参数问题
    if _port & !0x7 != 0 || _port & 0x7 == 0 {
        return -1
    }
    // 注意_start按页对齐，len可直接按页向上取整
    let start_va = VirtAddr::from(_start);
    let end_va = VirtAddr::from(_start + _len);
    // 排除未对齐、内存不足、范围重叠问题
    if !start_va.aligned() || frame_is_full() {
        return -1
    }
    // 根据port生成对应的权限
    let mut perm = MapPermission::U;
    if _port & 0b001!= 0 {
        perm |= MapPermission::R;
    }
    if _port & 0b010!= 0 {
        perm |= MapPermission::W;
    }
    if _port & 0b100!= 0 {
        perm |= MapPermission::X;
    }
    // 没有问题，可以插入地址空间
    // 注意该函数帮你：分配了Map_area，分配了物理页帧，并绑定了Page_table
    current_ms_mmap(start_va, end_va, perm)
}

// YOUR JOB: Implement munmap.
pub fn sys_munmap(_start: usize, _len: usize) -> isize {
    trace!("kernel: sys_munmap NOT IMPLEMENTED YET!");
    // lab2
    update_syscall_times(SYSCALL_MUNMAP);
    // 注意_start按页对齐，len可直接按页向上取整
    let start_va = VirtAddr::from(_start);
    let end_va = VirtAddr::from(_start + _len);
    // 排除未对齐
    if !start_va.aligned() {
        return -1
    }
    current_ms_munmap(start_va,end_va)
}
/// change data segment size
pub fn sys_sbrk(size: i32) -> isize {
    trace!("kernel: sys_sbrk");
    // lab2
    update_syscall_times(SYSCALL_SBRK);

    if let Some(old_brk) = change_program_brk(size) {
        old_brk as isize
    } else {
        -1
    }
}
