

fn main() {

	{
		let mut s = String::from("hello");
		s.push_str(", world!");
		println!("{}",s);

	}//s will be "drop" at this point(out of scope)


	
	let x = 5;
	let y = x; //what happened?
	println!("y = {}",y);
	//It just assign value 5 to y 
	//because integer is fixed size value which will be pushed on a stack



	let s1 = String::from("hello");
	let s2 = s1; //what happened?
	println!("{}, world (S2)",s2);
	//assign the poiner of s1 to s2
	//s1 is now not valid..
	//println!("{}, world!",s1); -> this won't be compiled
	//We say that s1 was moved to s2



	let s3 = String::from("hello");
	let s4 = s3.clone(); //Deep copy!
	println!("s3 = {}, s4 = {}", s3, s4); //So both valid



	let s5 = String::from("hello");
	takes_ownership(s5); //s is moved.. so it is no longer valid
	//println!("{}",s5); -> so this won't be compiled


	
	let x = 5;
	makes_copy(x); //It just copy value
	println!("{}",x);
	

	
	let s6 = String::from("hello");
	let s7 = takes_and_gives_back(s6); //Return will mv ownership again
	println!("{}",s7);



	let s8 = String::from("hello");
	let (s9,len) = calculate_len(s8);
	println!("The length of {} is {}",s9,len);

}

fn takes_ownership(some_string: String) {
	println!("{}",some_string);
}

fn makes_copy(some_integer: u32) {
	println!("{}",some_integer);
}

fn takes_and_gives_back(a_string: String) -> String {
	a_string
}

fn calculate_len(s: String) -> (String, usize) {
	let length = s.len();
	(s, length)
}

