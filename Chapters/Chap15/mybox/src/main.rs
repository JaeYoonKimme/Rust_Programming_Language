use std::ops::Deref;

/*
   Implimentation of Deref trait.
   Deref trait requires us to implement one method named 'deref'.
   It returns a reference to the inner data.
*/
impl<T> Deref for MyBox<T> {
	type Target = T; //Associated type.(will be covered at Chap19)

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
	fn new(x: T) -> MyBox<T> {
		MyBox(x)
	}
}


fn main() {
	let x = 5;
	let y = MyBox::new(x);

	assert_eq!(5, *y); // *(y.deref()) 
	// '*' operator will call deref
}
