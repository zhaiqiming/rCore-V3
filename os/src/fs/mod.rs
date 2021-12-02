mod pipe;
mod stdio;
mod inode;
mod stat;

use crate::mm::UserBuffer;

pub trait File : Send + Sync {
    fn readable(&self) -> bool;
    fn writable(&self) -> bool;
    fn read(&self, buf: UserBuffer) -> usize;
    fn write(&self, buf: UserBuffer) -> usize;

    // fn get_nlink_amount(&self) -> usize {
    //     return 0;
    // }
    fn get_inode_number(&self) -> usize {
        return 0;
    }
    fn get_file_type(&self) -> StatMode {
        return StatMode::NULL;
    }
    // fn increase_one_nlink(&self) {
        
    // }
    // fn decrease_one_nlink(&self) {
        
    // }
}


pub use pipe::{Pipe, make_pipe};
pub use stdio::{Stdin, Stdout};
pub use inode::{OSInode, open_file, OpenFlags, list_apps, link_file, find_inode_by_name, unlink_file, get_nlink_num};
pub use stat::{Stat, StatMode};