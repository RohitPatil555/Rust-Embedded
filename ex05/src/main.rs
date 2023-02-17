
use futures::executor::block_on;

async fn hello_world() {
	println!("Hello, World!");
}


fn main() {
	let f1 = hello_world();
	block_on(f1);
}
