//! Semaphore

use crate::sync::UPSafeCell;
use crate::task::{block_current_and_run_next, current_task, wakeup_task, TaskControlBlock};
use alloc::{collections::VecDeque, sync::Arc};
// lab5
use crate::task::{current_process};

/// semaphore structure
pub struct Semaphore {
    /// semaphore inner
    pub inner: UPSafeCell<SemaphoreInner>,
}

pub struct SemaphoreInner {
    pub count: isize,
    pub wait_queue: VecDeque<Arc<TaskControlBlock>>,
}

impl Semaphore {
    /// Create a new semaphore
    pub fn new(res_count: usize) -> Self {
        trace!("kernel: Semaphore::new");
        Self {
            inner: unsafe {
                UPSafeCell::new(SemaphoreInner {
                    count: res_count as isize,
                    wait_queue: VecDeque::new(),
                })
            },
        }
    }

    /// up operation of semaphore
    // lab5 修改参数
    pub fn up(&self, tid: usize, sem_id: usize) {
        trace!("kernel: Semaphore::up");
        let mut inner = self.inner.exclusive_access();
        inner.count += 1;
        // lab5
        let process = current_process();
        let mut process_inner = process.inner_exclusive_access();
        process_inner.dd.available[sem_id] += 1;
        process_inner.dd.allocation[tid][sem_id] -= 1;

        if inner.count <= 0 {
            if let Some(task) = inner.wait_queue.pop_front() {
                wakeup_task(task);
            }
        }
    }

    /// down operation of semaphore
    // lab5 修改参数、返回值
    pub fn down(&self, tid: usize, sem_id: usize) -> isize {
        trace!("kernel: Semaphore::down");
        let mut inner = self.inner.exclusive_access();
        inner.count -= 1;
        // lab5
        let process = current_process();
        let mut process_inner = process.inner_exclusive_access();
        if inner.count < 0 {
            // 没有能力分配，需要阻塞
            process_inner.dd.need[tid][sem_id] += 1;
            if process_inner.dd.is_enable && process_inner.dd.detect_deadlock() {
                process_inner.dd.need[tid][sem_id] -= 1;
                inner.count += 1;
                return -0xdead;
            }
            drop(process_inner);
            inner.wait_queue.push_back(current_task().unwrap());
            drop(inner);
            block_current_and_run_next();
        } else {
            // 有能力分配
            process_inner.dd.available[sem_id] -= 1;
            process_inner.dd.allocation[tid][sem_id] += 1;
            if process_inner.dd.is_enable && process_inner.dd.detect_deadlock() {
                process_inner.dd.available[sem_id] += 1;
                process_inner.dd.allocation[tid][sem_id] -= 1;
                inner.count += 1;
                return -0xdead;
            }
        }
        0
    }
}
