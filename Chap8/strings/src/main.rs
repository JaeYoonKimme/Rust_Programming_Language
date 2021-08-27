fn main() {
	//Creating a string
	let mut s = String::new();

	let data = "initial contents"; //This is str type
	let s2 = data.to_string(); 
	let s3 = "initial contents".to_string(); //it will cast str to String (to_string)
	let s4 = String::from("initial contents"); //it is also same!



	//Updating a String
	let mut s5 = String::from("foo");
	s5.push_str("bar"); //Now s5 contains "foobar"
	let t = "bar";
	s5.push_str(t); //Now s5 contains "foobarbar"

	println!("S5 is {}", s5);

	let mut s6 = String::from("lo");
	s6.push('l'); //It takes one Character as a parameter
	println!("S6 is {}",s6);



	//Concatenation String with + Operator or format!(Macro)
	let s7 = String::from("Hello, ");
	let s8 = String::from("world!");
	let s9 = s7 + &s8; //s7 has been moved here.. so can not be used from now.



}
