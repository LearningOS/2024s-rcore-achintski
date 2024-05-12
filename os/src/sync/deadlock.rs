//! lab5
//! Implementation of  [`DeadlockDetector`]

// use crate::sync::UPSafeCell;
// use lazy_static::*;
use alloc::vec;
use alloc::vec::Vec;


pub const MAX_ID: usize = 10;
pub const MAX_RESOURCE: usize = 10;

/// Deadlock Detector
pub struct DeadlockDetector {
    /// 是否开启死锁检测
    pub is_enable: bool,
    /// 可利用资源向量
    pub available: Vec<usize>,
    /// 分配矩阵
    pub allocation: Vec<Vec<usize>>,
    /// 需求矩阵
    pub need: Vec<Vec<usize>>,
    /// 工作向量
    work: Vec<usize>, 
    /// 结束向量
    finish: Vec<bool>,
}

impl DeadlockDetector {
    /// 初始化
    pub fn new() -> Self {
        DeadlockDetector {
            is_enable: false,
            available: vec![0; MAX_RESOURCE],
            allocation: vec![vec![0; MAX_RESOURCE]; MAX_ID],
            need: vec![vec![0; MAX_RESOURCE]; MAX_ID],
            work: vec![0; MAX_RESOURCE],
            finish: vec![false; MAX_ID],
        }
    }

    /// 清除全部状态
    pub fn flush(&mut self) {
        self.is_enable = false;
        self.available.iter_mut().for_each(|x| *x = 0);
        self.allocation.iter_mut().for_each(|row| row.iter_mut().for_each(|x| *x = 0));
        self.need.iter_mut().for_each(|row| row.iter_mut().for_each(|x| *x = 0));
        self.work.iter_mut().for_each(|x| *x = 0);
        self.finish.iter_mut().for_each(|x| *x = false);
    }

    /// 检测是否死锁
    pub fn detect_deadlock(&mut self) -> bool {
        // 先初始化
        for i in 0..self.available.len() {
            self.work[i] = self.available[i];
        }
        self.finish.iter_mut().for_each(|x| *x = false);

        let mut i = 0;
        while i < self.finish.len() {
            if!self.finish[i] && self.can_allocate(i) {
                // 尝试分配成功，回收
                self.allocate(i);
                // 需要从头开始尝试
                i = 0;
            } else {
                i += 1;
            }
        }
        !self.finish.iter().all(|&x| x)
    }

    /// 尝试分配给第i号线程看是否成功
    pub fn can_allocate(&self, i: usize) -> bool {
        for j in 0..self.available.len() {
            if self.need[i][j] > self.work[j] {
                return false;
            }
        }
        true
    }

    /// 尝试分配给第i号线程成功后，回收其占有的所有类型资源
    pub fn allocate(&mut self, i: usize) {
        for j in 0..self.available.len() {
            self.work[j] += self.allocation[i][j];
        }
        self.finish[i] = true;
    }
}

// lazy_static! {
//     /// DEADLOCK_DETECTOR instance through lazy_static!
//     pub static ref DEADLOCK_DETECTOR: UPSafeCell<DeadlockDetector> = 
//         unsafe { UPSafeCell::new(DeadlockDetector::new()) };
// }

