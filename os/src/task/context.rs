#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct TaskContext {
    pub ra: usize,
    pub s: [usize; 12],
}

impl TaskContext {
    pub fn goto_restore() -> Self {
        unsafe extern "C" {
            fn __restore();
        }
        Self {
            ra: __restore as usize,
            ..Default::default()
        }
    }
}
