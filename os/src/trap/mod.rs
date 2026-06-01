use riscv::register::{
    stvec, scause, stval, sepc,
    scause::{Trap, Exception, Interrupt}
};
use crate::syscall::syscall;
use crate::loader::TrapContext;
use crate::task::exit_current_and_run_next;
use core::arch::global_asm;

global_asm!(include_str!("trap.S"));

pub fn init() {
    unsafe extern "C" { fn __alltraps(); }
    unsafe {
        stvec::write(__alltraps as usize, stvec::TrapMode::Direct);
    }
}

#[unsafe(no_mangle)]
pub fn trap_handler(cx: &mut TrapContext) -> &mut TrapContext {
    let scause_val = scause::read();
    let cause = scause_val.cause();
    match cause {
        Trap::Exception(Exception::UserEnvCall) => {
            cx.sepc += 4;
            cx.x[10] = syscall(cx.x[17], [cx.x[10], cx.x[11], cx.x[12]]) as usize;
            cx
        }
        _ => {
            println!("[kernel] trap in app: scause={:#x}, sepc={:#x}, stval={:#x}",
                     scause_val.bits(), sepc::read(), stval::read());
            exit_current_and_run_next();
            loop {}
        }
    }
}
