pub fn sys_write(fd: usize, buf: *const u8, len: usize) -> isize {
    if fd == 1 {
        let s = unsafe { core::slice::from_raw_parts(buf, len) };
        let str = core::str::from_utf8(s).unwrap();
        print!("{}", str);
        len as isize
    } else {
        -1
    }
}
