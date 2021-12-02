use crate::mm::{
    UserBuffer,
    translated_byte_buffer,
    translated_refmut,
    translated_str,
};
use crate::task::{current_user_token, current_task};
use crate::fs::{make_pipe, OpenFlags, open_file, Stat, StatMode, link_file, find_inode_by_name, unlink_file, get_nlink_num};
use alloc::sync::Arc;

pub fn sys_write(fd: usize, buf: *const u8, len: usize) -> isize {
    let token = current_user_token();
    let task = current_task().unwrap();
    let inner = task.acquire_inner_lock();
    if fd >= inner.fd_table.len() {
        return -1;
    }
    if let Some(file) = &inner.fd_table[fd] {
        if !file.writable() {
            return -1;
        }
        let file = file.clone();
        // release Task lock manually to avoid deadlock
        drop(inner);
        file.write(
            UserBuffer::new(translated_byte_buffer(token, buf, len))
        ) as isize
    } else {
        -1
    }
}

pub fn sys_read(fd: usize, buf: *const u8, len: usize) -> isize {
    let token = current_user_token();
    let task = current_task().unwrap();
    let inner = task.acquire_inner_lock();
    if fd >= inner.fd_table.len() {
        return -1;
    }
    if let Some(file) = &inner.fd_table[fd] {
        let file = file.clone();
        if !file.readable() {
            return -1;
        }
        // release Task lock manually to avoid deadlock
        drop(inner);
        file.read(
            UserBuffer::new(translated_byte_buffer(token, buf, len))
        ) as isize
    } else {
        -1
    }
}

pub fn sys_open(path: *const u8, flags: u32) -> isize {
    let task = current_task().unwrap();
    let token = current_user_token();
    let path = translated_str(token, path);
    if let Some(inode) = open_file(
        path.as_str(),
        OpenFlags::from_bits(flags).unwrap()
    ) {
        let mut inner = task.acquire_inner_lock();
        let fd = inner.alloc_fd();
        inner.fd_table[fd] = Some(inode);
        fd as isize
    } else {
        -1
    }
}

pub fn sys_close(fd: usize) -> isize {
    let task = current_task().unwrap();
    let mut inner = task.acquire_inner_lock();
    if fd >= inner.fd_table.len() {
        return -1;
    }
    if inner.fd_table[fd].is_none() {
        return -1;
    }
    inner.fd_table[fd].take();
    0
}

pub fn sys_link_at(path_old: *const u8, path_new: *const u8) -> isize {
    let task = current_task().unwrap();
    let token = current_user_token();
    let name_old = translated_str(token, path_old);
    let name_new = translated_str(token, path_new);
    // println!("{} - {}", path_old as usize , path_new as usize);
    let mut inner = task.acquire_inner_lock();
    if link_file(name_old.as_str(), name_new.as_str()) == false {
        return -1;
    } 
    0
    // match find_inode_by_name(name_old.as_str()) {
    //     Some(inode) => {
    //         for it in inner.fd_table.iter() {
    //             match *it {
    //                 Some(ref osinode) => {
    //                     println!("{} {}", osinode.get_inode_number(), inode.get_inode_id());
    //                     if osinode.get_inode_number() == inode.get_inode_id() {
    //                         osinode.increase_one_nlink();
    //                         println!("IN");
    //                         return 0;
    //                     } else {
    //                         continue;
    //                     }
    //                 }
    //                 None => {
    //                     continue;
    //                 }
    //             }
    //         }
    //         return -1;
    //     }
    //     None => {
    //         return -1;
    //     }
    // }
}

pub fn sys_unlink_at(path: *const u8) -> isize {
    let task = current_task().unwrap();
    let token = current_user_token();
    let name = translated_str(token, path);
    let mut inner = task.acquire_inner_lock();
    if unlink_file(name.as_str()) == false {
        return -1;
    }
    0
    // match find_inode_by_name(name.as_str()) {
    //     Some(inode) => {
    //         for it in inner.fd_table.iter() {
    //             match *it {
    //                 Some(ref osinode) => {
    //                     if osinode.get_inode_number() == inode.get_inode_id() {
    //                         return 0;
    //                     } else {
    //                         continue;
    //                     }
    //                 }
    //                 None => {
    //                     continue;
    //                 }
    //             }
    //         }
    //         return -1;
    //     }
    //     None => {
    //         return -1;
    //     }
    // }
}

pub fn sys_fstat(fd: usize, stat_address: usize) -> isize {
    let task = current_task().unwrap();
    let token = current_user_token();
    let mut stat = translated_refmut(token, stat_address as *mut Stat);
    let mut inner = task.acquire_inner_lock();
    match inner.fd_table[fd] {
        Some(ref osinode) => {
            stat.ino = osinode.get_inode_number() as u64;
            stat.nlink = get_nlink_num(stat.ino as usize) as u32;
            stat.mode = osinode.get_file_type();
            return 0;
        }
        None => {
            return -1;
        }
    }
}

pub fn sys_pipe(pipe: *mut usize) -> isize {
    let task = current_task().unwrap();
    let token = current_user_token();
    let mut inner = task.acquire_inner_lock();
    let (pipe_read, pipe_write) = make_pipe();
    let read_fd = inner.alloc_fd();
    inner.fd_table[read_fd] = Some(pipe_read);
    let write_fd = inner.alloc_fd();
    inner.fd_table[write_fd] = Some(pipe_write);
    *translated_refmut(token, pipe) = read_fd;
    *translated_refmut(token, unsafe { pipe.add(1) }) = write_fd;
    0
}

pub fn sys_dup(fd: usize) -> isize {
    let task = current_task().unwrap();
    let mut inner = task.acquire_inner_lock();
    if fd >= inner.fd_table.len() {
        return -1;
    }
    if inner.fd_table[fd].is_none() {
        return -1;
    }
    let new_fd = inner.alloc_fd();
    inner.fd_table[new_fd] = Some(Arc::clone(inner.fd_table[fd].as_ref().unwrap()));
    new_fd as isize
}