use cc; 

fn main() { 
    cc::Build::new()
    .file("src/mem_writer/writer.c")
    .compile("libfoo.a");
}