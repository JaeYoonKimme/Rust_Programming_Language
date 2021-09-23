/* This code will make compile error. Think about reason.
enum List {
	Cons(i32, Box<List>),
	Nil,
}

use crate::List::{Cons,Nil};

fn main() {
	let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
	let b = Cons(3, Box::new(a));
	let c = Cons(4, Box::new(a));
}

*/

enum List {
	Cons(i32, Rc<List>),
	Nil,
}

use crate::List::{Cons,Nil};
use std::rc::Rc;


fn main() {
	let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil))))); 
	println!("count after creating a = {}", Rc::strong_count(&a));
	let b = Cons(3, Rc::clone(&a)); //Rc::clone() does not make deepcopy. It just increments reference count.
	println!("count after creating b = {}", Rc::strong_count(&a));
	{
		let c = Cons(4, Rc::clone(&a)); //So it does not take much time.
		println!("count after creating c = {}", Rc::strong_count(&a));
	}
	println!("count after c goes out of scope = {}", Rc::strong_count(&a));

	//Rc<T> type also has a weak_count()

}
