use std::thread;
use std::time::Duration;

fn main() {
	//spawn will return "JoinHandle"
	let handle = thread::spawn(|| {
		for i in 1..10 {
			println!("Hi number {} from spawned thread!", i);
			thread::sleep(Duration::from_millis(1));
		}
	});

	handle.join().unwrap();
	//unwrap() will unpack enum type like Option<T>..

	for i in 1..5 {
		println!("hi number {} from the main thread!",i);
		thread::sleep(Duration::from_millis(1));
	}

	//handle.join().unwrap();
	
}
