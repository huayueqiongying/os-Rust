use crate::task::{suspend_current_and_run_next, exit_current_and_run_next};
use crate::timer::get_time_ms;

pub fn sys_exit(exit_code: i32) -> ! {
    println!("[kernel] App exited with code {}", exit_code);
    exit_current_and_run_next();
    loop {}
}

pub fn sys_yield() -> isize {
    suspend_current_and_run_next();
    0
}

pub fn sys_get_time() -> isize {
    get_time_ms() as isize
}