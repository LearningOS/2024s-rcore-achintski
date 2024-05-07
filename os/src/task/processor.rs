//!Implementation of [`Processor`] and Intersection of control flow
//!
//! Here, the continuous operation of user apps in CPU is maintained,
//! the current running state of CPU is recorded,
//! and the replacement and transfer of control flow of different applications are executed.

use super::__switch;
use super::{fetch_task, TaskStatus};
use super::{TaskContext, TaskControlBlock};
use crate::sync::UPSafeCell;
use crate::trap::TrapContext;
use alloc::sync::Arc;
use lazy_static::*;
// lab1
use crate::timer::get_time_ms;
use crate::syscall::TaskInfo;
// lab2
use crate::mm::translated_byte_t;
use crate::mm::{VirtAddr, MapPermission};
use crate::config::MAX_SYSCALL_NUM;

/// Processor management structure
pub struct Processor {
    ///The task currently executing on the current processor
    current: Option<Arc<TaskControlBlock>>,

    ///The basic control flow of each core, helping to select and switch process
    idle_task_cx: TaskContext,
}

impl Processor {
    ///Create an empty Processor
    pub fn new() -> Self {
        Self {
            current: None,
            idle_task_cx: TaskContext::zero_init(),
        }
    }

    // lab3
    ///set current
    fn set_current(&mut self, new_current: Option<Arc<TaskControlBlock>>) {
        self.current = new_current;
    }

    ///Get mutable reference to `idle_task_cx`
    fn get_idle_task_cx_ptr(&mut self) -> *mut TaskContext {
        &mut self.idle_task_cx as *mut _
    }

    ///Get current task in moving semanteme
    pub fn take_current(&mut self) -> Option<Arc<TaskControlBlock>> {
        self.current.take()
    }

    ///Get current task in cloning semanteme
    pub fn current(&self) -> Option<Arc<TaskControlBlock>> {
        self.current.as_ref().map(Arc::clone)
    }
}

lazy_static! {
    pub static ref PROCESSOR: UPSafeCell<Processor> = unsafe { UPSafeCell::new(Processor::new()) };
}

///The main part of process execution and scheduling
///Loop `fetch_task` to get the process that needs to run, and switch the process through `__switch`
pub fn run_tasks() {
    loop {
        let mut processor = PROCESSOR.exclusive_access();
        if let Some(task) = fetch_task() {
            let idle_task_cx_ptr = processor.get_idle_task_cx_ptr();
            // access coming task TCB exclusively
            let mut task_inner = task.inner_exclusive_access();
            let next_task_cx_ptr = &task_inner.task_cx as *const TaskContext;
            task_inner.task_status = TaskStatus::Running;
            // lab1 lab3
            // 在TCB第一次被调度时，初始化TCBInner中的start_time
            if task_inner.start_time == 0 {
                task_inner.start_time = get_time_ms();
            }
            // release coming task_inner manually
            drop(task_inner);
            // release coming task TCB manually
            processor.current = Some(task);
            // release processor manually
            drop(processor);
            unsafe {
                __switch(idle_task_cx_ptr, next_task_cx_ptr);
            }
        } else {
            warn!("no tasks available in run_tasks");
        }
    }
}

/// Get current task through take, leaving a None in its place
pub fn take_current_task() -> Option<Arc<TaskControlBlock>> {
    PROCESSOR.exclusive_access().take_current()
}

/// Get a copy of the current task
pub fn current_task() -> Option<Arc<TaskControlBlock>> {
    PROCESSOR.exclusive_access().current()
}

/// Get the current user token(addr of page table)
pub fn current_user_token() -> usize {
    let task = current_task().unwrap();
    task.get_user_token()
}

///Get the mutable reference to trap context of current task
pub fn current_trap_cx() -> &'static mut TrapContext {
    current_task()
        .unwrap()
        .inner_exclusive_access()
        .get_trap_cx()
}

///Return to idle control flow for new scheduling
pub fn schedule(switched_task_cx_ptr: *mut TaskContext) {
    let mut processor = PROCESSOR.exclusive_access();
    let idle_task_cx_ptr = processor.get_idle_task_cx_ptr();
    drop(processor);
    unsafe {
        __switch(switched_task_cx_ptr, idle_task_cx_ptr);
    }
}

// lab1 lab3
/// update tcb syscall_times
pub fn update_syscall_times(syscall_id: usize) {
    // 只需要更新tcb.inner中的syscall_times
    let task = current_task().unwrap();
    let mut tcb_inner = task.inner_exclusive_access();

    tcb_inner.syscall_times[syscall_id] += 1;
}

// lab2 lab3
/// get current taskinfo
pub fn current_task_info() -> (TaskStatus, [u32; MAX_SYSCALL_NUM], usize) {
    let task = current_task().unwrap();
    let tcb_inner = task.inner_exclusive_access();

    let ms = get_time_ms();
    (TaskStatus::Running, tcb_inner.syscall_times, ms - tcb_inner.start_time)
}

// lab1 lab2 lab3
/// init task_info from tcb
pub fn init_task_info(_ti: *mut TaskInfo){
    let taskinfo = current_task_info();
    let ti = TaskInfo::new(taskinfo.0, taskinfo.1, taskinfo.2,);

    translated_byte_t(current_user_token(), _ti, &ti);
}

// lab2 lab3
/// sys_mmap
pub fn current_ms_mmap(start_va: VirtAddr, end_va: VirtAddr, perm: MapPermission) -> isize {
    let task = current_task().unwrap();
    let mut tcb_inner = task.inner_exclusive_access();
    let ms = &mut tcb_inner.memory_set;

    // 重叠则返回-1
    if ms.is_overlap(start_va, end_va) {
        return -1;
    }
    ms.insert_framed_area(start_va, end_va, perm);
    0
}

// lab2 lab3
/// sys_munmap
pub fn current_ms_munmap(start_va: VirtAddr, end_va: VirtAddr) -> isize {
    let task = current_task().unwrap();
    let mut tcb_inner = task.inner_exclusive_access();
    let ms = &mut tcb_inner.memory_set;

    ms.current_ms_munmap(start_va, end_va)
}

// lab3
/// sys_spawn
pub fn set_current(new_current: Option<Arc<TaskControlBlock>>) {
    PROCESSOR.exclusive_access().set_current(new_current);
}