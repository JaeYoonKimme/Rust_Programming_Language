use std::io;

fn main() {
	//Character type : char
	let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

	let kor = 'ㅋ';
	println!("{} {} {} {}",c,z,heart_eyed_cat, kor);



	//Tuple type
	let tup = (500, 6.4, 1, "String");

	let (a, b, c, d) = tup; //This is called 'destructuring'

	println!("Tuple print a : {} b : {} c : {} d : {} tup.3 : {}",a,b,c,d,tup.3);

	
	//The Array Type
	let arr = [1, 2, 3, 4, 5];

	println!("Please enter an arr index.");

	let mut index = String::new();

	io::stdin()
		.read_line(&mut index)
		.expect("Failed to read line");

	let index: usize = index
		.trim()
		.parse()
		.expect("Index entered was not a number");

	let element = arr[index]; //Panic!!

	println!(
			"The value of the element at index {} is : {}",
			index, element
	);
}
