fn main() {
	let s1 = String::from("abcdefg");
	let s2 = "abc";

	let result = longest(s1.as_str(), s2);
	println!("The longest string is {}", result);

}

/*
fn longest (x: &str, y: &str) -> &str {
	if x.len() > y.len() {
		x
	}else {
		y
	}
}

This implementation is not valid
Because Rust can not check the return of this function is x or y

&i32 -> reference
&'a i32 -> reference with explicit lifetime 'a'
&'a mut i32 -> mutable referenfce with lifetime 'a'
*/


/*
When returning a reference from a function, 
the lifetime parameter for the return type needs to match the lifetime parameter 
for one of the parameters.
*/
fn longest <'a> (x: &'a str, y: &'a str) -> &'a str {
	if x.len() > y.len() {
		x
	}else {
		y
	}
}
