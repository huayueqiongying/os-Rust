use core::arch::global_asm;
use riscv::register::{stvec, mtvec::TrapMode, scause, stval};
use crate::syscall::syscall;
use crate::batch::run_next_app;

global_asm!(include_str!("trap.S"));

mod context;
pub use context::TrapContext;

pub fn init() {
    unsafe extern "C" { fn __alltraps(); }
    unsafe {
        stvec::write(__alltraps as *const () as usize, TrapMode::Direct);
    }
}

#[unsafe(no_mangle)]
pub fn trap_handler(cx: &mut TrapContext) -> &mut TrapContext {
    let cause = scause::read().cause();
    let stv = stval::read();
    match cause {
        scause::Trap::Exception(scause::Exception::UserEnvCall) => {
            cx.sepc += 4;
            cx.x[10] = syscall(cx.x[17], [cx.x[10], cx.x[11], cx.x[12]]) as usize;
        }
        scause::Trap::Exception(scause::Exception::StoreFault) |
        scause::Trap::Exception(scause::Exception::StorePageFault) => {
            println!("[kernel] Store PageFault, kill app");
            run_next_app();
        }
        scause::Trap::Exception(scause::Exception::IllegalInstruction) => {
            println!("[kernel] IllegalInstruction, kill app");
            run_next_app();
        }
        other => {
            panic!("unhandled trap: {:?}, stval={:#x}", other, stv);
        }
    }
    cx
}
