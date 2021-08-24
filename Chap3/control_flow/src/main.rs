fn main() {

	let number = 3;

	if number < 5{
		println!("Condition True");
	} else{
		println!("Condition False");
	}

	//This would not be compiled
	/*
	if number{
		println!("HI");
	}
	*/

	let condition = true;
	let number = if condition { 5 } else { 6 };

	println!("The value of number is : {}", number);


	/*
	This code would not be compiled
	let condition = true;
	let number = if condition { 5 } else { "SIX };

	println!("The value of number is : {}", number);
	*/


	let mut counter = 0;
	let result = loop {
		counter += 1;

		if counter == 100 {
			break counter * 100;
		}
	};
	println!("The result is {}",result);


	let arr = [10, 20, 30, 40, 50];

	for el in arr.iter() {
		println!("the value is : {}", el);
	}

	for number in (1..5).rev(){
		println!("{}!",number);
	}

		
}
