#![no_std]

use core::panic::PanicInfo;
#[macro_use]
use crate::println;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("user app panicked!");
    loop {}
}
