#[cfg(test)]
mod tests {
    #[test]    //This indicates that this is a test function
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }


	#[test]
	fn another() {
		panic!("Make this test fail");
	}
}
