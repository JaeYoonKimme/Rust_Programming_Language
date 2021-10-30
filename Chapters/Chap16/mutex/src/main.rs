use std::sync::Mutex;

fn main() {
	let m = Mutex::new(5);

	{
		let mut num = m.lock().unwrap(); 
		//here return value of lock is smart pointer => "MutexGuard" wraped in "LockResult"
		//when unwrap, return is MutesGuard, which implements Deref. 
		* num = 6; //So by deref num, we can access mut ref of data inside(in this case 5).

		/*
		   MutexGuard releases lock when it goes out of scope and is droped.
		   So we don't need to worry about it.
		*/

		println!("{:?}", m);
	}

	println!("m = {:?}", m);

}
