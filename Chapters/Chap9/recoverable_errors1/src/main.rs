/*
This is definition of 'Result enum' having two variants 'Ok' and 'Err'
   
enum Result<T, E> {  //generic type 'T' and 'E'
	Ok(T),  //Success
	Err(E), //Failure
}
*/


use std::fs::File;
use std::io::ErrorKind;
use std::io;
use std::io:Read;
fn main() {
	//Opening a file
	let f = File::open("hello.txt");	//'open' returns 'result'. 
										//we can find it at standard library API documentation
										//Or we can check it at error message. lets limit 'f' with some type like u32!

	//How to handle possible error?
	let f = match f {
		Ok(file) => file,  //Result variants are already exist in this scope. So we don't need to specify it like 'Result::Ok"
		
		//We can handle the each different errors by seperately
		Err(error) => match error.kind() {
			ErrorKind::NotFound => match File::create("hello.txt") {
				Ok(fc) => fc,
				Err(e) => panic!("Problem creating the file : {:?}",e),
			},
			other_error => panic!("Problem opening the file: {:?}", other_error),
		},
	};




	
	//Shrtcuts for Panic on Error: 'unwrap' and 'expect'
	let f2 = File::open("hello2.txt").unwrap(); //'unwrap()' will call panic if the Result is Err
	let f3 = File::open("hello3.txt").expect("Failed to open hello.txt"); //we can set error message using 'expect()'


}



/*
   When we writing a function, We can return error to handle it out of functions,
   rather handle it at inside of the functions
   This is called Propagiting Errors.
*/

//Propagating Errors.
fn read_username_from_file() -> Result<String, io::Error> { //Retuns result it self
	let f = File::open("hello.txt");

	let mut f = match f {
		Ok(file) => file,
		Err(e) => return Err(e), //In here, if we don't explicity use 'return', the return will be go to 'f'
								 //In this case, we should use 'return' to tell this is function return! not a scope
	};

	let mut s = String::new();

	match f.read_to_string(&mut s) { //'read_to_string' will read file to string 's'
		Ok(_) => Ok(s),
		Err(e) => Err(e),
	}

}

