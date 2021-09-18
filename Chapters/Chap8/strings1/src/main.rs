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
	println!("S9 is {}",s9);
	/* '+' Operator uses 'add' method 
		fn add(self, s: &str) -> String {.....

		It takes 'self' as first parameter, and returns 'String' (not a ref),
		So first parameter will be moved to return with its ownership 

		that results 's7' no more valid...


		but we can find that &s8 which we gives as parameter is &String! not a &str (which 'add' needs)
		This is because of "deref coercion" of Rust which turns &s8 into &s8[..] (&String into &str)
	*/


	let s10 = String::from("tic");
	let s11 = String::from("tac");
	let s12 = String::from("toc");
	let s14 = String::from("tic");
	let s13 = s10 + "-" + &s11 + "-" + &s12;
	//(Using format! macro is much easier to use and read)
	let s14 = format!("{}-{}-{}",s14,s11,s12); //both s13 ,s14 are same. 
	println!("S13 is {}",s13);
	println!("S14 is {}",s14);


}
