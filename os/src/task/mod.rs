mod context;
mod switch;
mod task;

use crate::loader::{get_num_app, get_app_data};
use crate::trap::TrapContext;
use core::cell::RefCell;
use lazy_static::*;
use switch::__switch;
use task::{TaskControlBlock, TaskStatus};
use alloc::vec::Vec;
use crate::mm::MapPermission;
use crate::mm::{VirtPageNum, VirtAddr};

pub use context::TaskContext;

pub struct TaskManager {
    num_app: usize,
    inner: RefCell<TaskManagerInner>,
}

struct TaskManagerInner {
    tasks: Vec<TaskControlBlock>,
    current_task: usize,
}

unsafe impl Sync for TaskManager {}

lazy_static! {
    pub static ref TASK_MANAGER: TaskManager = {
        println!("init TASK_MANAGER");
        let num_app = get_num_app();
        println!("num_app = {}", num_app);
        let mut tasks: Vec<TaskControlBlock> = Vec::new();
        for i in 0..num_app {
            tasks.push(TaskControlBlock::new(
                get_app_data(i),
                i,
            ));
        }
        TaskManager {
            num_app,
            inner: RefCell::new(TaskManagerInner {
                tasks,
                current_task: 0,
            }),
        }
    };
}

impl TaskManager {
    fn run_first_task(&self) {
        self.inner.borrow_mut().tasks[0].task_status = TaskStatus::Running;
        let next_task_cx_ptr2 = self.inner.borrow().tasks[0].get_task_cx_ptr2();
        let _unused: usize = 0;
        unsafe {
            __switch(
                &_unused as *const _,
                next_task_cx_ptr2,
            );
        }
    }

    fn mark_current_suspended(&self) {
        let mut inner = self.inner.borrow_mut();
        let current = inner.current_task;
        inner.tasks[current].task_status = TaskStatus::Ready;
    }

    fn mark_current_exited(&self) {
        let mut inner = self.inner.borrow_mut();
        let current = inner.current_task;
        inner.tasks[current].task_status = TaskStatus::Exited;
    }

    fn find_next_task(&self) -> Option<usize> {
        let inner = self.inner.borrow();
        let current = inner.current_task;
        (current + 1..current + self.num_app + 1)
            .map(|id| id % self.num_app)
            .find(|id| {
                inner.tasks[*id].task_status == TaskStatus::Ready
            })
    }

    fn get_current_token(&self) -> usize {
        let inner = self.inner.borrow();
        let current = inner.current_task;
        inner.tasks[current].get_user_token()
    }

    fn get_current_trap_cx(&self) -> &mut TrapContext {
        let inner = self.inner.borrow();
        let current = inner.current_task;
        inner.tasks[current].get_trap_cx()
    }

    fn run_next_task(&self) {
        if let Some(next) = self.find_next_task() {
            let mut inner = self.inner.borrow_mut();
            let current = inner.current_task;
            inner.tasks[next].task_status = TaskStatus::Running;
            inner.current_task = next;
            let current_task_cx_ptr2 = inner.tasks[current].get_task_cx_ptr2();
            let next_task_cx_ptr2 = inner.tasks[next].get_task_cx_ptr2();
            core::mem::drop(inner);
            unsafe {
                __switch(
                    current_task_cx_ptr2,
                    next_task_cx_ptr2,
                );
            }
        } else {
            panic!("All applications completed!");
        }
    }

    fn mmap(&self, start: usize, len: usize, port: usize) -> isize {
        if len == 0 {
            return 0;
        }
        if len > 1073741824{
            return -1;
        }
        if start % 4096 != 0 {
            return -1;
        }
        let mut length = len;
        if len % 4096 != 0 {
            length = len + (4096 - len % 4096);
        }
        if (port & !0x7 != 0) || (port & 0x7 == 0) {
            return -1;
        }
        
        // println!("@");
        let mut inner = self.inner.borrow_mut();
        let current = inner.current_task;
        // println!("Start : {:#X}", VirtPageNum::from(start/4096).0);
        let from:usize = start / 4096;
        let to:usize = (start + length) / 4096;
        // println!("from to {} {}", from, to);
        for vpn in from..to {
            if true == inner.tasks[current].memory_set.find_vpn(VirtPageNum::from(vpn)) {
                return -1;
            }
        }
        
        let permission = match port {
            1 => MapPermission::U | MapPermission::R,
            2 => MapPermission::U | MapPermission::W,
            3 => MapPermission::U | MapPermission::R | MapPermission::W,
            4 => MapPermission::U | MapPermission::X,
            5 => MapPermission::U | MapPermission::R | MapPermission::X,
            6 => MapPermission::U | MapPermission::X | MapPermission::W,
            _ => MapPermission::U | MapPermission::R | MapPermission::W | MapPermission::X,
        };

        inner.tasks[current].memory_set.insert_framed_area(VirtAddr::from(start), VirtAddr::from(start+length), permission);

        for vpn in from..to {
            if false == inner.tasks[current].memory_set.find_vpn(VirtPageNum::from(vpn)) {
                return -1;
            }
        }
        return length as isize;
    }

    pub fn munmap(&self, start: usize, len: usize) -> isize {
        if len == 0 {
            return 0;
        }
        if len > 1073741824{
            return -1;
        }
        if start % 4096 != 0 {
            return -1;
        }
        let mut length = len;
        if len % 4096 != 0 {
            length = len + (4096 - len % 4096);
        }
        let mut inner = self.inner.borrow_mut();
        let current = inner.current_task;
        let from:usize = start / 4096;
        let to:usize = (start + length) / 4096;
        for vpn in from..to {
            if false == inner.tasks[current].memory_set.find_vpn(VirtPageNum::from(vpn)) {
                return -1;
            }
        }

        for vpn in from..to {
            inner.tasks[current].memory_set.munmap(VirtPageNum::from(vpn));
        }

        for vpn in from..to {
            if true == inner.tasks[current].memory_set.find_vpn(VirtPageNum::from(vpn)) {
                return -1;
            }
        }

        return len as isize;
    }
}

pub fn run_first_task() {
    TASK_MANAGER.run_first_task();
}

fn run_next_task() {
    TASK_MANAGER.run_next_task();
}

fn mark_current_suspended() {
    TASK_MANAGER.mark_current_suspended();
}

fn mark_current_exited() {
    TASK_MANAGER.mark_current_exited();
}

pub fn suspend_current_and_run_next() {
    mark_current_suspended();
    run_next_task();
}

pub fn exit_current_and_run_next() {
    mark_current_exited();
    run_next_task();
}

pub fn current_user_token() -> usize {
    TASK_MANAGER.get_current_token()
}

pub fn current_trap_cx() -> &'static mut TrapContext {
    TASK_MANAGER.get_current_trap_cx()
}

pub fn mmap(start: usize, len: usize, port: usize) -> isize {
    TASK_MANAGER.mmap(start, len, port)
}

pub fn munmap(start: usize, len: usize) -> isize {
    TASK_MANAGER.munmap(start, len)
}