pub fn add_two(a: i32) -> i32 {
	internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
	a + b
}

#[cfg(test)]
mod tests {
    use super::*;

	#[test]
	fn internal() {
		assert_eq!(internal_adder(2, 2), 4);
	}	
}

/*
   In this test, we bring all of the test module’s parent’s items into scope with use super::*, 
   and then the test can call internal_adder.

*/
