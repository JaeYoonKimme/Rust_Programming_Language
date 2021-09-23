use std::ops::Deref;

impl<T> Deref for MyBox<T> {
	type Target = T;

	fn deref(&self) -> &T {
		&self.0
	}
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
	fn new(x: T) -> MyBox<T> {
		MyBox(x)
	}
}

fn hello(name: &str) {
	println!("Hello, {}!", name);
}

fn main() {
	let m = MyBox::new(String::from("Rust"));
	hello(&m);

	/* 
	   Deref on MyBox will return &String, and it again call deref, 
	   it will return &str (String slice) as it defined on standard library.
	   Standard library provides an implementation of Deref on String to return &str.
	*/

	hello(&(*m)[..]);
	/*
	   (*m) will return String, 
	   & and [...] will return &str, String Slice of whole string.



	*/
}
