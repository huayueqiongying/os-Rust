use crate::task::{suspend_current_and_run_next, exit_current_and_run_next};
use crate::sbi;

pub fn sys_write(fd: usize, buf: u8) -> isize {
    if fd == 1 {
        sbi::console_putchar(buf as usize);
        1
    } else {
        -1
    }
}

pub fn sys_exit(exit_code: i32) -> ! {
    println!("[kernel] App exited with code {}", exit_code);
    exit_current_and_run_next();
    loop {}
}

pub fn sys_yield() -> isize {
    suspend_current_and_run_next();
    0
}
