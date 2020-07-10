extern "C" {
    fn i4_write_memory(code: *const u8, len: usize);
}

fn main() {
    let a = "Hello, C!".as_ptr();
    let b = 1 as usize;  
    unsafe { testing(a, b); } 
    println!("");
}
