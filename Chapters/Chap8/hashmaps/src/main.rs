use std::collections::HashMap;


fn main() {

	//Creating a New Hash Map
	let mut scores = HashMap::new();

	scores.insert(String::from("Blue"), 10);
	scores.insert(String::from("Yellow"), 50);



	//zip(), into_iter(), collect().. 
	let teams = vec![String::from("Blue"), String::from("Yellow")];
	let initial_scores = vec![10,50];

	let mut scores2: HashMap<_, _> = 
		teams.into_iter().zip(initial_scores.into_iter()).collect();

	

	//Hash Maps and Ownership
	let field_name = String::from("Favorite color");
	let field_value = String::from("Blue");

	let mut map = HashMap::new();
	map.insert(field_name, field_value); //Values moved at here
	//println!("{}",field_name);  -> Error. 'field_name' is moved.


	

	//Accessing values in a Hash Map
	for (key, value) in &scores {
		println!("{}: {}", key, value);
	}



	//Updating a Hash Map
	scores.insert(String::from("Blue"), 100); // -> Overwriting a value
	for (key, value) in &scores {
		println!("{}: {}", key, value);
	}

	

	//Inserting a value only if Key has no Value
	scores.entry(String::from("Yellow")).or_insert(60); //It will work only when the key, "Yellow" does not exist
	scores.entry(String::from("Green")).or_insert(60); //entry() will return mutable reference which corresponds to given parameter key..
	for (key, value) in &scores {
		println!("{}: {}", key, value);
	}


	//Updating hashmap using Old value
	let text = "hello world wonderful world";
	let mut words = HashMap::new();

	for word in text.split_whitespace() {
		let count = words.entry(word).or_insert(0); //or_insert() will return '&mut V' mutable ref to 'count'
		*count += 1; //So we should use * to modify mut ref 'count'
	}

	println!("{:?}",words);



}
