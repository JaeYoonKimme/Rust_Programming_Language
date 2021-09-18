fn main() {
	another_function(5);	

	//let x = (let y = 6); -> it occurs error(assignment does not return anything in RUST)

	//let x = 5;

	let y = {
		let x = 3;
		x + 1
	};

	println!("The value of y is : {}",y);

	let z = plus_one(6);
	println!("The value of z is : {}",z);
}

fn another_function(x: i32) {
	println!("The value of x is : {}", x);
}

fn plus_one(n: i32) -> i32 {
	n+1
}
