use std::io;
use std::process;


fn fibo_recursion(n: u32) -> u32 {
	if n == 1 || n == 2 {
		1
	}else {
		fibo_recursion(n-1) + fibo_recursion(n-2)
	}
}

fn fibo_dynamic(n: u32) -> u32 {
	let mut arr = [1; 256];

	for i in 3..n+1 {
		arr[i as usize] = arr[(i-1) as usize ] + arr[(i-2) as usize];	
	}

	arr[(n) as usize]
}


fn main() {
	println!("Enter the number!");

	let mut n = String::new();

	io::stdin()
		.read_line(&mut n)
		.expect("failed to read line");
	
	let n: u32 = match n.trim().parse(){
		Ok(num) => num,
		Err(err) => {
			eprintln!("error: {:?}", err);
			process::exit(0x0100);
		}
	};

	println!("Dynamic Result is : {}",fibo_dynamic(n));
	println!("Recursive Result is : {}",fibo_recursion(n)); 
}
