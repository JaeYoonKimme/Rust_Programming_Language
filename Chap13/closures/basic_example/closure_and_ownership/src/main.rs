fn main() {
	let x = vec![1,2,3];

	let equal_to_x = move |z| z == x;

	println!("can't use x here: {:?}",x);

	let y = vec![1,2,3];

	assert!(equal_to_x(y));
}

/*
   The x value(Ownership) is moved into the closure when the closure is defined, 
   because we added the move keyword.(See keyworkd 'move' in the closure)
   The closure then has ownership of x, and main isn’t allowed to use x anymore in the println! statement. 
   Removing println! will fix this example.




   Closure has three type of traits
   [FnOnce] consumes the variables it captures from its enclosing scope, known as the closure’s environment. 
   To consume the captured variables, the closure must take ownership of these variables 
   and move them into the closure when it is defined.

   [FnMut] borrows environment as mutable, so it can be changed in closure, withouht taking(move) ownership.

   [Fn] borrows environment as immutable. (Same with immutable refference)

 */

