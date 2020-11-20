use std::os::raw::c_uchar;

extern "C" {
    fn i4_write_memory(code: *const u8, len: usize) -> *const c_uchar;
    //fn i4_write_memory(code: *const u8, len: usize) -> i32;
}

unsafe fn emit_code(code: *const u8, len: usize) -> extern "C" fn(i32, i32) -> i32 {
    let func = i4_write_memory(code, len);
    let result: extern "C" fn(i32, i32) -> i32 = std::mem::transmute(func);
    return result;
}

fn main() {
    let raw_code: *const u8 = [0x8d, 0x04, 0x37, 0xc3].as_ptr();
    let code = unsafe { emit_code(raw_code, 4) };
    let result = code(10, 12);
    println!("{}", result)
}
