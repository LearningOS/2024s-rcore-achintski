//! File trait & inode(dir, file, pipe, stdin, stdout)

mod inode;
mod stdio;

use crate::mm::UserBuffer;
// lab4
pub use inode::{link_at};
use alloc::sync::Arc;

// lab4
/// A trait that allows converting a type to a `&dyn Any` reference.
///
/// This trait is useful when you need to convert a type to a `&dyn Any` reference,
/// which can be used for dynamic casting or other purposes.
pub trait AnyConvertor {
    /// Converts `self` to a `&dyn Any` reference.
    ///
    /// This method is used to convert a type to a `&dyn Any` reference,
    /// which can be used for dynamic casting or other purposes.
    fn as_any(&self) -> &dyn core::any::Any;
}

/// Implementation of `AnyConvertor` for `Arc<dyn File + Send + Sync>`.
///
/// This implementation allows `Arc<dyn File + Send + Sync>` to be converted
/// to a `&dyn Any` reference using the `as_any` method.
impl AnyConvertor for Arc<dyn File + Send + Sync> {
    /// Converts `self` to a `&dyn Any` reference.
    ///
    /// This method is used to convert a type to a `&dyn Any` reference,
    /// which can be used for dynamic casting or other purposes.
    fn as_any(&self) -> &dyn core::any::Any {
        self
    }
}

/// trait File for all file types
pub trait File: Send + Sync {
    /// the file readable?
    fn readable(&self) -> bool;
    /// the file writable?
    fn writable(&self) -> bool;
    /// read from the file to buf, return the number of bytes read
    fn read(&self, buf: UserBuffer) -> usize;
    /// write to the file from buf, return the number of bytes written
    fn write(&self, buf: UserBuffer) -> usize;
}

/// The stat of a inode
#[repr(C)]
#[derive(Debug)]
pub struct Stat {
    /// ID of device containing file
    pub dev: u64,
    /// inode number
    pub ino: u64,
    /// file type and mode
    pub mode: StatMode,
    /// number of hard links
    pub nlink: u32,
    /// unused pad
    pad: [u64; 7],
}

bitflags! {
    /// The mode of a inode
    /// whether a directory or a file
    pub struct StatMode: u32 {
        /// null
        const NULL  = 0;
        /// directory
        const DIR   = 0o040000;
        /// ordinary regular file
        const FILE  = 0o100000;
    }
}

pub use inode::{list_apps, open_file, OSInode, OpenFlags};
pub use stdio::{Stdin, Stdout};
