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

	for (i, &item) in bytes.iter().enurmerate() {
		if item == b' ' {
			return &s[0..1];
		}
	}

	&s[..]
}
