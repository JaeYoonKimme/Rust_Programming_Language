/*
	cfg means Configuration

	#[cfg(test)]
	This tells Rust that the following module should be included when given a certain config option
	In here,
	We gives 'test' option to configuration
	So tests module will be compiled only when we give 'cargo test'
*/

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
