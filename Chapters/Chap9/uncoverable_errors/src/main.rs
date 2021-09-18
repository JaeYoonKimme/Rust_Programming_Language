fn main() {
	let v = vec![1,2,3,4,5];
	v[99]; // -> This will make panic! 
		   // Lets run this with 'RUST_BACKTRACE=1 cargo run'
		   // Backtrace is all list of functions that are used in program until the panic.


	//Or you can make panic by using macro like this.
	//panic!("panic messages");
}
