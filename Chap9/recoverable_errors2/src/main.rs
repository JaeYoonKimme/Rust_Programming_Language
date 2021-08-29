use std::fs::File;
use std::io;
use std::io::Read;


//A shortcut for propagating Errors : The '?' Operator
fn read_username_from_file() -> Result<String, io::Error> {
	let mut f = File::open("hello.txt")?;
	let mut s = String::new();
	f.read_to_string(&mut s)?;
	Ok(s);

	//Using '?' operator, if the result is Err, it will be returned from the whole function like 'return' keyword
	//if not, Ok will get returned just from this scope(expression). So program will continue
}

//Shorter one
fn read_username_from_file2() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}


//Much Shorter one
use std::fs;
fn read_username_from_file3() -> Result<String, io::Error> {
	fs::read_to_string("hello.txt")  //This will open file,  create new string and read from file to string
}


//Result can be used at function which returns Result type
/*
fn main() {	
	let f = File::open("hello.txt")?; -> This will make error.
}
*/
//One valid return type that 'main' can have.
fn main() -> Result<(), Box<dyn Error>> {
	let f = File::open("hello.txt")?;
	Ok(())
}

