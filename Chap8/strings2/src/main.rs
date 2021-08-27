fn main() {
	//Indexing into Strings!
	let s1 = String::from("hello");
	//let h = s1[0]; -> compile error. Rust doesn't support String indexing



	//Slicing Strings	
	let hello = "Здравствуйте";
	let s = &hello[0..4]; // s -> 'Зд' because each of hello's character is 2Bytes.
	//let s = &hello[0..1] -> This will make panic


	//Iterating String using 'chars()'
	for c in "नमस्ते".chars() {
    	println!("{}", c);
	}

	//Iterating String using 'bytes()'
	for b in "नमस्ते".bytes() {
    	println!("{}", b);
	}




}
