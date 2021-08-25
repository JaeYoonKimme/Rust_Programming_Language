fn main() {
	//give Reference parameter instead of ownership!
	let s1 = String::from("hello");
	let len = calculate_length(&s1); //It does not give ownership. This is called 'borrowing'
	println!("The length of {} is {}",s1,len);


	
	let mut s2 = String::from("hello"); //If we declare it as mutable, 
	change(&mut s2);					//we can change its value with its reference
	println!("s2 -> {}",s2);


	/*Errored code 
	You can have only one mutable refference -> This restriction prevents 'Data Races'
	
	let mut s3 = String::from("hello");
	let r1 = &mut s3;
	let r2 = &mut s3;

	println!("{} {}", r1, r2);
	*/



	/*Errored code
	Similar rule exist when mutable and immutable are combined

	let mut s = String::from("hello");
	let r1 = &s; ->no problem
	let r2 = &s; ->no problem
	let r3 = &mut s;  
	println!("{}, {} and {}", r1, r2, r3); -> This makes problem 
	*/



	let mut s3 = String::from("hello");

	let r1 = &s3;
	let r2 = &s3;
	println!("{} and {}", r1, r2);

	let r3 = &mut s3;
	println!("{}",r3);


	
	//let reference_to_nothing = dangle();
	let new_owner = no_dangle();
	println!("{} new owner",new_owner);

}

fn calculate_length(s: &String) -> usize {
	//s.push_str(", world") -> This won't be compiled
	s.len()
}//Nothing happens because s has no ownership

fn change(some_string: &mut String) {
	some_string.push_str(", world");
}

/*
fn dangle() -> &String { //!! This is wrong function
	let s = String::from("hello");

	&s
} //But s will be 'drop' ed here so reference to s will be broken!! 
*/


fn no_dangle() -> String {
	let s = String::from("hello");

	s
}//Ownership will be moved and nothing happens
