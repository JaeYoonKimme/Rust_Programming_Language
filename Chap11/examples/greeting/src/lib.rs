pub fn greeting(name: &str) -> String {
	//format!("Hello {}!",name) -> Right code
	format!("Hello") //this will make bug
}


#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn greeting_contains_name() {
		let result = greeting("Carol");
		
		//We can give Error message like this
		assert!( 
				result.contains("Carol"),
				"Greeting did not contain name, value was '{}'",
				result
			   );
	}
}
