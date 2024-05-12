//! Synchronization and interior mutability primitives

mod condvar;
mod mutex;
mod semaphore;
mod up;
// lab5
mod deadlock;

pub use condvar::Condvar;
pub use mutex::{Mutex, MutexBlocking, MutexSpin};
pub use semaphore::Semaphore;
pub use up::UPSafeCell;
// lab5
pub use deadlock::DeadlockDetector;
