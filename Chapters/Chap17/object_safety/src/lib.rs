
/*
This is Clone trait looks like.

pub trait Clone {
	fn clone(&self) -> Self;
}

*/

pub struct Screen {
	pub components: Vec<Box<dyn Clone>>,
	//This makes error because "Clone" trait is not object safe
}
