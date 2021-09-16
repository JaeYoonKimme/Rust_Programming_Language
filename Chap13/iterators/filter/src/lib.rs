#[derive(PartialEq, Debug)]
struct Shoe {
	size: u32,
	style: String,
}


fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
	shoes.into_iter().filter(|x| x.size == shoe_size).collect()
}
/* 
   filter() takes closure which returns boolean type
   if closure returns false, value won't be included in the resulting iterator
*/


#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn filters_by_size() {
		let shoes = vec![
			Shoe {
				size: 10,
				style: String::from("sneaker"),
			},
			Shoe {
				size: 13,
				style: String::from("sandal"),
			},
			Shoe {
				size: 10,
				style: String::from("boot"),
			}
		];

		let in_my_size = shoes_in_size(shoes, 10);

		assert_eq!(
				in_my_size,
				vec![
					Shoe {
						size: 10,
						style: String::from("sneaker"),
					},
					Shoe {
						size: 10,
						style: String::from("boot"),
					}
				]
		);
	}
}


/*
   What is difference between 'iter()' and 'into_iter()' ??

   iter() -> it takes immutable refference original vector. (borrow).
   into_iter() -> it takes ownership of original vector.



   There also exist 'iter_mut()' -> It takes mutable refference and returns an iterator that allows modifying each value.

*/
