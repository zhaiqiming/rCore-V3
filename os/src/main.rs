#![no_std]
#![no_main]
#![feature(global_asm)]
#![feature(llvm_asm)]
#![feature(panic_info_message)]
#![feature(const_in_array_repeat_expressions)]
#![feature(alloc_error_handler)]

extern crate alloc;

#[macro_use]
extern crate bitflags;

#[macro_use]
mod console;
mod lang_items;
mod sbi;
mod syscall;
mod trap;
mod config;
mod task;
mod timer;
mod mm;
mod fs;
mod drivers;
mod comlog;


global_asm!(include_str!("entry.asm"));

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| {
        unsafe { (a as *mut u8).write_volatile(0) }
    });
}

#[no_mangle]
pub fn rust_main() -> ! {
    comlog::info!("[Kernel] init start");
    clear_bss();

    comlog::init();
    comlog::info!("comlog init finished!");

    mm::init();
    comlog::info!("memory manager init finished!");
    mm::remap_test();

    trap::init();
    comlog::info!("trap manager init finished!");
    trap::enable_timer_interrupt();

    timer::set_next_trigger();
    fs::list_apps();
    task::add_initproc();

    comlog::info!("[Kernel] init finished, start to run tasks");
    task::run_tasks();
    panic!("Unreachable in rust_main!");
}