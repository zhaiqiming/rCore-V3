use crate::mm::{UserBuffer, translated_byte_buffer, translated_refmut, translated_str_safe};
use crate::task::{current_user_token, current_task};
use crate::fs::{make_pipe};
use crate::task::{get_task_by_pid};
use alloc::sync::{Arc};
use core::cmp::min;
use alloc::string::String;
use crate::config::{*};

pub fn sys_write(fd: usize, buf: *const u8, len: usize) -> isize {
    let token = current_user_token();
    let task = current_task().unwrap();
    let inner = task.acquire_inner_lock();
    if fd >= inner.fd_table.len() {
        return -1;
    }
    if let Some(file) = &inner.fd_table[fd] {
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
        // release Task lock manually to avoid deadlock
        drop(inner);
        file.read(
            UserBuffer::new(translated_byte_buffer(token, buf, len))
        ) as isize
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

pub fn sys_write_mail(pid: usize, buf: *mut u8, len: usize) -> isize {
    // pid -> task_control_block
    // println!("pid : {}", pid);
    let mut ans = get_task_by_pid(pid);
    if pid == current_task().unwrap().getpid() {
        ans = current_task();
    }
    match ans {
        Some(task_to) => {
            let token = current_user_token();
            let mut inner_to = task_to.acquire_inner_lock();

            if inner_to.mail_box.len() == MAIL_BOX_SIZE {
                return -1;
            }
            println!("len : {}", len);
            if len == 0 {
                return 0;
            }

            match translated_str_safe(token, buf, len) {
                Some(mut message) => {
                    println!("message : {}", message);
                    while message.len() > MAIL_BOX_MESSAGE_SIZE {
                        message.remove(MAIL_BOX_MESSAGE_SIZE);
                    }
                    let length = message.len();
                    inner_to.mail_box.push_back(message);
                    return length as isize;
                }
                None => return -1,
            }
        }
        None => {
            println!("find task fail !");
            -1
        }
    }
}

pub fn sys_read_mail(buf: *mut u8, len: usize) -> isize {
    let task = current_task().unwrap();
    let token = current_user_token();
    let mut inner = task.acquire_inner_lock();
    if len == 0 {
        if inner.mail_box.len() > 0 {
            return 0;
        }else {
            return -1;
        }
    }
    let mut message = inner.mail_box.pop_front();
    match message {
        Some(mut message) => {
            while message.len() > len {
                message.remove(len);
            }

            let mut num: isize = 0;
            let mut addr_user = buf;
            for index in 0..min(len, message.len()) {
                *translated_refmut(token, addr_user) = message.as_bytes()[index];
                unsafe { addr_user = addr_user.add(1); }
                num += 1;
            }
            return num;
        }
        None => {
            return -1;
        }
    }
}