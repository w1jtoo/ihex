extern "C" {
    fn testing(code: *const u8, len: usize);
}

fn main() {
    let a = "Hello, C!".as_ptr();
    unsafe { testing(a); } 
    println!("");
}
