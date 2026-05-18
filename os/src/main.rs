#![no_std]
#![no_main]

#[macro_use]
mod console;
mod lang_items;
mod sbi;
mod syscall;
mod trap;
mod batch;

use core::arch::global_asm;

global_asm!(include_str!("entry.asm"));
global_asm!(include_str!("link_app.S"));

fn clear_bss() {
    unsafe extern "C" {
        fn sbss();
        fn ebss();
    }
    let s = sbss as *const () as usize;
    let e = ebss as *const () as usize;
    for a in s..e {
        unsafe { (a as *mut u8).write_volatile(0) };
    }
}

#[unsafe(no_mangle)]
pub fn rust_main() -> ! {
    clear_bss();
    println!("[kernel] Hello, kernel!");
    trap::init();
    batch::init();
    batch::run_next_app();
}
