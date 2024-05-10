//! File and filesystem-related syscalls
use crate::fs::{open_file, OpenFlags, Stat};
use crate::mm::{translated_byte_buffer, translated_str, UserBuffer};
use crate::task::{current_task, current_user_token};
// lab1 lab3
use super::{SYSCALL_WRITE, SYSCALL_READ};
use crate::task::update_syscall_times;
// lab4
use super::{SYSCALL_OPEN, SYSCALL_CLOSE, SYSCALL_LINKAT, SYSCALL_UNLINKAT, SYSCALL_FSTAT};
use crate::fs::{link_at, OSInode, unlink_at};

pub fn sys_write(fd: usize, buf: *const u8, len: usize) -> isize {
    trace!("kernel:pid[{}] sys_write", current_task().unwrap().pid.0);
    // lab1
    update_syscall_times(SYSCALL_WRITE);

    let token = current_user_token();
    let task = current_task().unwrap();
    let inner = task.inner_exclusive_access();
    if fd >= inner.fd_table.len() {
        return -1;
    }
    if let Some(file) = &inner.fd_table[fd] {
        if !file.writable() {
            return -1;
        }
        let file = file.clone();
        // release current task TCB manually to avoid multi-borrow
        drop(inner);
        file.write(UserBuffer::new(translated_byte_buffer(token, buf, len))) as isize
    } else {
        -1
    }
}

pub fn sys_read(fd: usize, buf: *const u8, len: usize) -> isize {
    trace!("kernel:pid[{}] sys_read", current_task().unwrap().pid.0);
    // lab3
    update_syscall_times(SYSCALL_READ);

    let token = current_user_token();
    let task = current_task().unwrap();
    let inner = task.inner_exclusive_access();
    if fd >= inner.fd_table.len() {
        return -1;
    }
    if let Some(file) = &inner.fd_table[fd] {
        let file = file.clone();
        if !file.readable() {
            return -1;
        }
        // release current task TCB manually to avoid multi-borrow
        drop(inner);
        trace!("kernel: sys_read .. file.read");
        file.read(UserBuffer::new(translated_byte_buffer(token, buf, len))) as isize
    } else {
        -1
    }
}

pub fn sys_open(path: *const u8, flags: u32) -> isize {
    trace!("kernel:pid[{}] sys_open", current_task().unwrap().pid.0);
    // lab4
    update_syscall_times(SYSCALL_OPEN);

    let task = current_task().unwrap();
    let token = current_user_token();
    let path = translated_str(token, path);
    if let Some(inode) = open_file(path.as_str(), OpenFlags::from_bits(flags).unwrap()) {
        let mut inner = task.inner_exclusive_access();
        let fd = inner.alloc_fd();
        inner.fd_table[fd] = Some(inode);
        fd as isize
    } else {
        -1
    }
}

pub fn sys_close(fd: usize) -> isize {
    trace!("kernel:pid[{}] sys_close", current_task().unwrap().pid.0);
    // lab4
    update_syscall_times(SYSCALL_CLOSE);

    let task = current_task().unwrap();
    let mut inner = task.inner_exclusive_access();
    if fd >= inner.fd_table.len() {
        return -1;
    }
    if inner.fd_table[fd].is_none() {
        return -1;
    }
    inner.fd_table[fd].take();
    0
}

/// YOUR JOB: Implement fstat.
pub fn sys_fstat(_fd: usize, _st: *mut Stat) -> isize {
    trace!(
        "kernel:pid[{}] sys_fstat",
        current_task().unwrap().pid.0
    );
    // lab4
    update_syscall_times(SYSCALL_FSTAT);
    let task = current_task().unwrap();
    let inner = task.inner_exclusive_access();
    if _fd >= inner.fd_table.len() {
        return -1;
    }
    if let Some(file) = &inner.fd_table[_fd] {
        let file = file.clone();
        // release current task TCB manually to avoid multi-borrow
        drop(inner);
        if let Some(os_inode) = file.as_any().downcast_ref::<OSInode>() {
            os_inode.fstat(_st);
            0
        } else {
            -1
        }
    } else {
        -1
    }
}

/// YOUR JOB: Implement linkat.
pub fn sys_linkat(_old_name: *const u8, _new_name: *const u8) -> isize {
    trace!(
        "kernel:pid[{}] sys_linkat",
        current_task().unwrap().pid.0
    );
    // lab4
    update_syscall_times(SYSCALL_LINKAT);
    let token = current_user_token();
    let _old_name = translated_str(token, _old_name);
    let _new_name = translated_str(token, _new_name);
    if _old_name == _new_name {
        return -1;
    }
    link_at(_old_name.as_str(), _new_name.as_str())
}

/// YOUR JOB: Implement unlinkat.
pub fn sys_unlinkat(_name: *const u8) -> isize {
    trace!(
        "kernel:pid[{}] sys_unlinkat",
        current_task().unwrap().pid.0
    );
    // lab4
    update_syscall_times(SYSCALL_UNLINKAT);
    let token = current_user_token();
    let _name = translated_str(token, _name);
    unlink_at(_name.as_str())
}
