/*
   Do not communicate by sharing memory.
   Instaed, share memory by communicating.

   Rust provides "Channel" to provide communication among threads.
   "Channel" consists of two parts, "Transmitter" and "Reciever"

   "Channel" is closed when either one is dropped.
*/

use std::sync::mpsc; //mpsc : multiple producer, single consumer.
//It meas multiple sending ends which produce values, but only one receiving end that consumes those values.
use std::thread;

fn main() {
	let (tx, rx) = mpsc::channel();

	thread::spawn(move || {
		let val = String::from("HI");
		tx.send(val).unwrap(); //send will return Result<T,E> so if error occur(ex, channel drop), it will panic.
		//One thing you should remember is "send" takes "val"'s ownership.
		//and it is delivered to reciever.
	});

	let recieved = rx.recv().unwrap();
	//let recieved = rx.try_recv().unwrap();
	/* recv wait until message comming until channel is alive.
	   try_recv immediately recieves message, and return Result<T, E>
	*/
	println!("Got: {}", recieved);

}
