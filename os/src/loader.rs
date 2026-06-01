use crate::config::{APP_BASE_ADDRESS, APP_SIZE_LIMIT};
use core::arch::asm;

pub fn get_base_i(app_id: usize) -> usize {
    APP_BASE_ADDRESS + app_id * APP_SIZE_LIMIT
}

pub fn get_num_app() -> usize {
    unsafe extern "C" {
        fn _num_app();
    }
    unsafe { (_num_app as usize as *const usize).read_volatile() }
}

pub fn load_apps() {
    let num_app = get_num_app();
    unsafe extern "C" { fn _num_app(); }
    let num_app_ptr = _num_app as usize as *const usize;
    let app_start = unsafe {
        core::slice::from_raw_parts(num_app_ptr.add(1), num_app + 1)
    };
    println!("[kernel] num_app = {}", num_app);
    unsafe { asm!("fence.i"); }
    for i in 0..num_app {
        let base_i = get_base_i(i);
        let size = app_start[i + 1] - app_start[i];
        println!("[kernel] app_{}: base={:#x}, src={:#x}, size={}", i, base_i, app_start[i], size);
        (base_i..base_i + APP_SIZE_LIMIT).for_each(|addr| unsafe {
            (addr as *mut u8).write_volatile(0)
        });
        let src = unsafe {
            core::slice::from_raw_parts(app_start[i] as *const u8, size)
        };
        let dst = unsafe {
            core::slice::from_raw_parts_mut(base_i as *mut u8, src.len())
        };
        dst.copy_from_slice(src);
        // print first 8 bytes of the loaded app
        let first_bytes = unsafe { *(base_i as *const u64) };
        println!("[kernel] app_{} first 8 bytes: {:#018x}", i, first_bytes);
    }
}

#[repr(C)]
pub struct TrapContext {
    pub x: [usize; 32],   // offset 0..255
    pub sstatus: usize,   // offset 256 (32*8) -- matches trap.S save order
    pub sepc: usize,      // offset 264 (33*8)
}

impl TrapContext {
    pub fn set_sp(&mut self, sp: usize) {
        self.x[2] = sp;
    }

    pub fn app_init_context(entry: usize, sp: usize) -> Self {
        let mut cx = Self {
            x: [0; 32],
            sepc: entry,
            sstatus: 0x1800,
        };
        cx.set_sp(sp);
        cx
    }
}
