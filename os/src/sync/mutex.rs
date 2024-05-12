//! Mutex (spin-like and blocking(sleep))

use super::UPSafeCell;
use crate::task::TaskControlBlock;
use crate::task::{block_current_and_run_next, suspend_current_and_run_next};
use crate::task::{current_task, wakeup_task};
use alloc::{collections::VecDeque, sync::Arc};
// lab5
use crate::task::{current_process};


/// Mutex trait
pub trait Mutex: Sync + Send {
    /// Lock the mutex
    // lab5 修改参数、返回值
    fn lock(&self, tid: usize, mutex_id: usize) -> isize;
    /// Unlock the mutex
    // lab5 修改参数
    fn unlock(&self, tid: usize, mutex_id: usize);
}

/// Spinlock Mutex struct
pub struct MutexSpin {
    locked: UPSafeCell<bool>,
}

impl MutexSpin {
    /// Create a new spinlock mutex
    pub fn new() -> Self {
        Self {
            locked: unsafe { UPSafeCell::new(false) },
        }
    }
}

impl Mutex for MutexSpin {
    /// Lock the spinlock mutex
    // lab5 修改参数、返回值
    fn lock(&self, _tid: usize, _mutex_id: usize) -> isize {
        trace!("kernel: MutexSpin::lock");
        loop {
            let mut locked = self.locked.exclusive_access();
            if *locked {
                drop(locked);
                suspend_current_and_run_next();
                continue;
            } else {
                *locked = true;
                return 0;
            }
        }
    }

    // lab5 修改参数
    fn unlock(&self, _tid: usize, _mutex_id: usize) {
        trace!("kernel: MutexSpin::unlock");
        let mut locked = self.locked.exclusive_access();
        *locked = false;
    }
}

/// Blocking Mutex struct
pub struct MutexBlocking {
    inner: UPSafeCell<MutexBlockingInner>,
}

pub struct MutexBlockingInner {
    locked: bool,
    wait_queue: VecDeque<Arc<TaskControlBlock>>,
}

impl MutexBlocking {
    /// Create a new blocking mutex
    pub fn new() -> Self {
        trace!("kernel: MutexBlocking::new");
        Self {
            inner: unsafe {
                UPSafeCell::new(MutexBlockingInner {
                    locked: false,
                    wait_queue: VecDeque::new(),
                })
            },
        }
    }
}

impl Mutex for MutexBlocking {
    /// lock the blocking mutex
    // lab5 修改参数、返回值
    fn lock(&self, tid: usize, mutex_id: usize) -> isize {
        trace!("kernel: MutexBlocking::lock");
        let mut mutex_inner = self.inner.exclusive_access();
        // lab5
        let process = current_process();
        let mut process_inner = process.inner_exclusive_access();
        if mutex_inner.locked {
            // 没有能力分配，需要阻塞
            process_inner.dd.need[tid][mutex_id] += 1;
            if process_inner.dd.is_enable && process_inner.dd.detect_deadlock() {
                process_inner.dd.need[tid][mutex_id] -= 1;
                return -0xdead;
            }
            drop(process_inner);
            mutex_inner.wait_queue.push_back(current_task().unwrap());
            drop(mutex_inner);
            block_current_and_run_next();
        } else {
            // 有能力分配
            process_inner.dd.available[mutex_id] -= 1;
            process_inner.dd.allocation[tid][mutex_id] += 1;
            if process_inner.dd.is_enable && process_inner.dd.detect_deadlock() {
                process_inner.dd.available[mutex_id] += 1;
                process_inner.dd.allocation[tid][mutex_id] -= 1;
                return -0xdead;
            }
            mutex_inner.locked = true;
        }
        0
    }

    /// unlock the blocking mutex
    // lab5 修改参数
    fn unlock(&self, tid: usize, mutex_id: usize) {
        trace!("kernel: MutexBlocking::unlock");
        let mut mutex_inner = self.inner.exclusive_access();
        assert!(mutex_inner.locked);
        // lab5
        let process = current_process();
        let mut process_inner = process.inner_exclusive_access();
        process_inner.dd.available[mutex_id] += 1;
        process_inner.dd.allocation[tid][mutex_id] -= 1;

        if let Some(waking_task) = mutex_inner.wait_queue.pop_front() {
            wakeup_task(waking_task);
        } else {
            mutex_inner.locked = false;
        }
    }
}
