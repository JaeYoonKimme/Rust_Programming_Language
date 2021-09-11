fn main() {
	
	let r;

	{
		let x = 5;
		r = &x;
	}


	println!("{}",r);
}

/*
	This code will make compile error
	Because it uses invalid reference 'r' (after x is out of scope)

	Rust compiler has "Borrow Checker" which checks if all borrows are valid"
*/
