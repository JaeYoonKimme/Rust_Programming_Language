#[derive(Debug)]
struct Rectangle {
	width: u32,
	height: u32,
}

impl Rectangle {
	fn can_hold(&self, other: &Rectangle) -> bool {
		self.width > other.width && self.height > other.height
	}
 
}


#[cfg(test)]
mod tests {
	use super::*;
	/*Because the tests module is an inner module, 
	  we need to bring the code under test in the outer module into the scope of the inner module.
	*/
	#[test]
	fn larger_can_hold_smaller() {
		let larger = Rectangle{
			width: 8,
			height: 7,
		};
		let smaller = Rectangle{
			width: 5,
			height: 1,
		};
	
		//assert!()
		//This will invoke the panic! macro if the provided expression cannot be evaluated to true at runtime.
		assert!(larger.can_hold(&smaller));  
	}

	#[test]
	fn smaller_can_hold_larger() {
		let larger = Rectangle {
			width: 8,
			height: 7,
		};
		let smaller = Rectangle {
			width: 5,
			height: 1,
		};
		assert!(!smaller.can_hold(&larger));
	}
}
