fn main() {
	let x = 5;
	//let y = &x;  -> Reference pointing to the value of x.
	let y = Box::new(x); // -> istance of a Box<T> pointing to a copied value of x.

	//defference is that 

	assert_eq!(x, 5);
	assert_eq!(*y, 5);
}

