#[test]
fn it_works() {
	assert_eq!(2 + 2, 4);
}

#[test]
#[ignore]
fn expensive_test() {
	// code that takes an hour to run
}


/*
   For testing all #[ignore] functions,
   use
   cargo test -- --ignored

*/
