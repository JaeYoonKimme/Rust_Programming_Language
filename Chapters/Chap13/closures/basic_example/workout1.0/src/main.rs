use std::thread;
use std::time::Duration;

/*
fn simulated_expensive_calculation(intensity: u32) -> u32 {
	println!("Calculation slowly...");
	thread::sleep(Duration::from_secs(2));
	intensity
}
This function is replaced by 'expensive_closure'
*/

fn main() {
	let simulated_user_specified_value = 10;
	let simulated_random_number = 7;

	generate_workout(simulated_user_specified_value, simulated_random_number);
}


fn generate_workout(intensity: u32, random_number: u32){
	let expensive_closure = |num| { //Defining Closure.. start with a pair of vertical pipes( | ) with parameter inside
		println!("calculating slowly...");
		thread::sleep(Duration::from_secs(2));
		num
	};/*
		In here, 'expensive_closure' does not contains the resulting value of Anonymous function
	    this contains definition of an anonymous function..
	  */

	if intensity < 25 {
		println!(
				"Today, do {} pushups!",
				expensive_closure(intensity)
		);
		println!(
				"Next, do {} situps!",
				expensive_closure(intensity)
		);
	} else {
		if random_number == 3 {
			println!("Take a break today! Remember to stay hydrated!");
		} else {
			println!(
					"Today, run for {} minutes!",
					expensive_closure(intensity)
			);
		}
	}
}

/*
   Closure does not need type annotation(with its parameter and return)
   Why??
   Type annotation is requred on functions because they are part of explicit interface exposed to users.
   But Closuer is not.
   It is just stored in variables and used without exposing them to users of library..
   Like a Variable!?

   Buy Optionally, it is also alloawed to annotate type like this.
   let expensive_closure = |num: i32| -> i32 { ....


*/


