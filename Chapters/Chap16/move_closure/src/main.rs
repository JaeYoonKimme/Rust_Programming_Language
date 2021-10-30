use std::thread;

fn main() {
	let v = vec![1, 2, 3];

	let handle = thread::spawn(move || {
		println!("Here's a vector: {:?}", v);
	});

	//drop(v); -> use moved value. v moved to thread.

	handle.join().unwrap();
}
