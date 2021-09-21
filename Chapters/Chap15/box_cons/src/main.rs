/*
enum List {
	Cons(i32, List),
	Nil,
}

use crate::List::{Cons, Nil};

fn main() {
	let list = Cons(1, Cons(2, Cons(3, Nil)));



   Above Code makes compile error.
   To determine how much space to allocate for a List, Rust compiler will check through size of each variants.
   So it first checks Cons, which need the size of i32 + List, and it again checks List to determine space recursively.


   This is called recursive data type.
   To manage this, we can use Box<T> type.
   Because Box<T> is a pointer, Rust know how much space it need.
*/






enum List {
	Cons(i32, Box<List>),
	Nil,
}

use crate::List::{Cons, Nil}; //To use Cons without List::Cons()

fn main() {
	let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}
