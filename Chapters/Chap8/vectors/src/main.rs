fn main() {
	//create new, empty vector to hold i32 type values
	let v: Vec<i32> = Vec::new();

	//vec! macro will create ector that holds the values we give it
	let v2 = vec![1, 2, 3]; // -> this is more like a conventional way to init Vector


	//Updating a Vector
	let mut v3 = Vec::new();
	v3.push(5);
	v3.push(6);
	v3.push(7);


	//Vectors life time in a scope
	{
		let v4 = vec![1,2,3];
	} //v4 will dropped(free) together with its values

	
	//How to read elements of vectors?
	let v5 = vec![1,2,3,4,5];
	let third: &i32 = &v5[2]; 
	println!("The third element is {}", third);

	match v5.get(2) {
		Some(val) => println!("The third element is {}",val),
		None => println!("There is no third element."),
	}


	//Handling out of index
	let v6 = vec![1,2,3,4,5];
	//let does_not_exist = &v6[100]; // This won't be compiled. it makes panic
	let does_not_exist = v.get(100); // This will return 'None' so we can handle it




	//Vector borrowing(ref) ownership and touch it
	/*
	let mut v7 = vec![1,2,3,4,5];
	let first = &v[0];
	
	v.push(6);
	println!("The first element is : {}",first);   it will result in error
	*/



	//Iterating over the values
	let mut v8 = vec![100,32,57];
	for i in &mut v8 {
		*i += 50;
		println!("{}",i);
	}



	//Store multiple types using Enum
	enum SpreadsheetCell {
		Int(i32),
		Float(f64),
		Text(String),
	}

	let row = vec![
		SpreadsheetCell::Int(3),
		SpreadsheetCell::Text(String::from("blue")),
		SpreadSheetCell::Float(10,12),
	];

}
