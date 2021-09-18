fn main() {

	let example_closure = |x| x; //This just consumes x and return it

	let s = example_closure(String::from("hello")); 
	let n = example_closure(5);

	/*
	   This makes compile error(MissMatched Type!)
	   at first use of closure, the types are locked into the closure.. in 'example_closure'
	   So it will get type error if we use it with different type.!
	*/

}
