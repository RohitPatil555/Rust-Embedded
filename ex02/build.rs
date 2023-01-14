extern crate cc;

fn main() {

	println!("cargo:rerun-if-changed=src/hello.c");

    cc::Build::new()
        .file("src/hello.c")
        .compile("libhello.a");
}
