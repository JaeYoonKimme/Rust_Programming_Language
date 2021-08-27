mod front_of_house {
	pub mod hosting {
		pub fn add_to_waitlist() {}

		fn seat_at_table() {}
	}

	mod serving {
		fn take_order() {}

		fn serve_order() {}

		fn take_payment() {}

	}
}

/*
How do we call the add_to_waitlist function? 
This is the same as asking, 
what’s the path of the add_to_waitlist function? 

crate::front_of::house::hosting::add_to_waitlist(); -> absolute path
front_of_house::hosting::add_to_waitlist(); -> Relative path
*/


pub fn eat_at_resttaurant() {
	crate::front_of::house::hosting::add_to_waitlist(); 
	front_of_house::hosting::add_to_waitlist(); 
}




mod back_of_house {
	pub struct Breakfast {
		pub toast: String,
		seasonal_fruit: String,
	}

	pub enum Appetizer {
		Soup,
		Salad,
	}

	impl Breakfast {
		pub fn summer(toast: &str) -> Breakfast {
			Breakfast {
				toast: String::from(toast),
				seasonal_fruit: String::from("peaches");
			}
		}
	}
}

pub fn eat_at_restaurant2() {
	let mut meal = back_of_house::Breakfast::summer("Rye");
	meal.toast = String::from("Wheat");
	println!("I'd like {} toast please", meal.toast);
}

pub fn eat_at_restaurant3() {
	let order1 = back_of_house::Appetizer::Soup;
	let order2 = back_of_house::Appetizer::Salad;
}





//Use keyword

mod front_of_house {
	pub mod hosting {
		pub fn add_to_waitlist(){}
	}
}

use self::front_of_house::hosting;

pub fn eat_at_restaurant4() {
	hosting::add_to_waitlist();
}



//We can use 'as' keyword to give new local name
//This can prevent redundent name
use std::fmt::Result;
use std::io::Result as IoResult;



/*
Re - exporting ('pub use')
By using pub use, 
external code can now call the add_to_waitlist function using hosting::add_to_waitlist
*/



//Using nested path
/*
   use std::cmp::Ordering;
   use std::io;

   above two line can be changed to below

   use std::{cmp::Ordering, io};



	use std::io;
	use std::io::Write;

	->

	use std::{self, Write};
*/



//Glob Operator
/*
   use std::collections::*;

   -> bring everything in collection
*/


