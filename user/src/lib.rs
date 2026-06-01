#![no_std]
#![no_main]
#![feature(asm_experimental_arch)]
mod syscall;
pub use syscall::sys_yield;
pub use syscall::sys_exit;
pub use syscall::yield_;
pub use syscall::sys_get_time;
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ({
        use core::fmt::Write;
        let _ = write!($crate::UartWriter, $($arg)*);
    });
}
#[macro_export]
macro_rules! println {
    ($($arg:tt)*) => ({
        use core::fmt::Write;
        let _ = writeln!($crate::UartWriter, $($arg)*);
    });
}
pub struct UartWriter;
impl core::fmt::Write for UartWriter {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for &b in s.as_bytes() {
            unsafe {
                core::arch::asm!(
                    "li a0, 1",
                    "mv a1, {c}",
                    "li a7, 64",
                    "ecall",
                    c = in(reg) b,
                    out("a0") _,
                    out("a1") _,
                    out("a7") _
                );
            }
        }
        Ok(())
    }
}
#[unsafe(link_section = ".text.entry")]
#[unsafe(no_mangle)]
unsafe extern "C" fn _start() -> ! {
    unsafe extern "C" { fn main() -> i32; }
    syscall::sys_exit(unsafe { main() });
    unreachable!()
}
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
pub fn get_time() -> isize { sys_get_time() }
