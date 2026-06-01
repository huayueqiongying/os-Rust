#![no_std]
#![no_main]
#![feature(lazy_cell)]

use core::arch::global_asm;
#[macro_use] mod console;
mod lang_items;
mod sbi;
mod trap;
mod syscall;
mod loader;
mod config;
mod task;

global_asm!(include_str!("entry.asm"));
global_asm!(include_str!("link_app.S"));

fn clear_bss() {
    unsafe extern "C" { fn sbss(); fn ebss(); }
    let mut p = sbss as usize as *mut u8;
    while p < ebss as usize as *mut u8 {
        unsafe { p.write(0); p = p.add(1); }
    }
}

#[unsafe(no_mangle)]
pub fn rust_main() -> ! {
    clear_bss();
    println!("[kernel] Hello, world!");
    trap::init();
    loader::load_apps();
    task::run_first_task();
}
