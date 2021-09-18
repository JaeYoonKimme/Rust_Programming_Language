fn is_valid_stone(x: u32, y: u32) -> bool {
	if x > 0 && x < 20 && y > 0 && y < 20 {
		true
	}else {
		panic!("Not valid");
	}
}


/*
   Which test will be tested and passed with 'cargo test' command?

   If we want to run only 'test3",
   what command should we use?
*/

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test1() {
		assert!(is_valid_stone(10,10));
	}

	#[test]
	fn test2() {
		assert_eq!(is_valid_stone(10,10), true);
	}

	#[test]
	#[ignore]
	fn test3() {
		assert_eq!(is_valid_stone(10,10), true);
	}

	#[test]
	fn test4() {
		assert_ne!(is_valid_stone(10,10), true);
	}

	#[test]
	#[should_panic]
	fn test5() {
		is_valid_stone(100,100);
	}

}
