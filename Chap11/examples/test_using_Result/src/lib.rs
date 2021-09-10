#[cfg(test)]
mod tests {
    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
			Ok(())
		} else {
			Err(String::from("two plus two does not equal four"))
		}
    }
}

/*
   You can’t use the #[should_panic] annotation on tests that use Result<T, E>. 
   Instead, you should return an Err value directly when the test should fail.
*/
