use riscv::register::{
    stvec,
    scause::{self, Trap, Exception, Interrupt},
    stval,
    sie,
};
use crate::syscall::syscall;
use crate::loader::TrapContext;
use crate::task::{exit_current_and_run_next, suspend_current_and_run_next};
use crate::timer::set_next_trigger;
use core::arch::global_asm;
global_asm!(include_str!("trap.S"));
pub fn init() {
    unsafe extern "C" { fn __alltraps(); }
    unsafe {
        stvec::write(__alltraps as usize, stvec::TrapMode::Direct);
    }
}
pub fn enable_timer_interrupt() {
    unsafe { sie::set_stimer(); }
}
#[unsafe(no_mangle)]
pub fn trap_handler(cx: &mut TrapContext) -> &mut TrapContext {
    let scause = scause::read();
    let stval = stval::read();
    match scause.cause() {
        Trap::Exception(Exception::UserEnvCall) => {
            cx.sepc += 4;
            cx.x[10] = syscall(cx.x[17], [cx.x[10], cx.x[11], cx.x[12]]) as usize;
        }
        Trap::Exception(Exception::StoreFault) |
        Trap::Exception(Exception::StorePageFault) => {
            println!("[kernel] PageFault");
            exit_current_and_run_next();
        }
        Trap::Exception(Exception::IllegalInstruction) => {
            println!("[kernel] IllegalInstruction");
            exit_current_and_run_next();
        }
        Trap::Interrupt(Interrupt::SupervisorTimer) => {
            set_next_trigger();
            suspend_current_and_run_next();
        }
        _ => {
            panic!("trap {:?}", scause.cause());
        }
    }
    cx
}
