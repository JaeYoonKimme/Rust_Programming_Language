struct CustomSmartPointer {
	data: String,
}

/*
   We can define dropping action with implementing 'Drop Trait' for our type
*/
impl Drop for CustomSmartPointer {
	fn drop(&mut self) {
		println!("Dropping CustomSmartPointer with data '{}'!", self.data);
	}
}

fn main() {
	let c = CustomSmartPointer {
		data: String::from("my stuff"),
	};

	let d = CustomSmartPointer {
		data: String::from("other stuff"),
	};

	println!("CustomSmartPointers created.");

	/*
	   Dropping order is reverse of their creation order at the end of scope.
	*/

	//c.drop(); -> explicit drop is not allowed. Because it could make double free..? 
	drop(c);
	my_drop(d);
	println!("CustomSmartPointer dropped before the end of main");
}

fn my_drop<T>(variable: T) {
	//Do nothing. Just end this scope and drop T.	
}
