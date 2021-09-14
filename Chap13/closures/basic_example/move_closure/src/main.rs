fn main() {
	let x = vec![1,2,3];

	let equal_to_x = move |z| z == x;

	println!("can't use x here: {:?}",x);

	let y = vec![1,2,3];

	assert!(equal_to_x(y));
}

/*
   The x value is moved into the closure when the closure is defined, because we added the move keyword.
   The closure then has ownership of x, and main isn’t allowed to use x anymore in the println! statement. 
   Removing println! will fix this example.
 */
