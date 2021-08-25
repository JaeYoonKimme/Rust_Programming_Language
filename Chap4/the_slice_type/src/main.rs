fn main() {
	let mut s1 = String::from("hello world");

	let word = first_word(&s1);
	s1.clear(); //// this empties the String, making it equal to ""
	println!("{}",word);

	
	//This is possible
	let s2 = String::from("hello universe");
	let hello = &s2[0..5];
	let world = &s2[6..];
	println!("{} {}",hello,world);


	let s3 = String::from("hello world");
	let word2 = first_word_rework(&s3);

	//s3.clear(); -> This will make error
	println!("word2 -> {}",word2);





	let my_string = String::from("hello world");

	let word3 = first_word_final(&my_string[..]);

	let my_string_literal = "hello world";

	let word4 = first_word_final(&my_string_literal[..]);

	let word5 = first_word_final(my_string_literal);

	println!("word3 -> {}",word3);
	println!("word4 -> {}",word4);
	println!("word5 -> {}",word5);
}

fn first_word(s: &String) -> usize {
	let bytes = s.as_bytes();

	for (i, &item) in bytes.iter().enumerate() { //enumerate returns tuple (index, reference)
		if item == b' ' {
			return i; //What is the difference with just using "i" ????
		}
	}
	
	s.len()
}

fn first_word_rework(s: &String) -> &str {
	let bytes = s.as_bytes();

	for (i, &item) in bytes.iter().enumerate() {
		if item == b' ' {
			return &s[0..i];
		}
	}

	&s[..]
}

fn first_word_final(s: &str) -> &str {
	let bytes = s.as_bytes();

	for (i, &item) in bytes.iter().enumerate() {
		if item == b' ' {
			return &s[0..i];
		}
	}

	&s[..]
}

