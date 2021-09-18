struct Counter {
	count: u32,
}

impl Counter {
	fn new() -> Counter {
		Counter {
			count: 0
		}
	}
}


impl Iterator for Counter { //Implement ITerator trait for our Counter
	
	type Item = u32; //Associated types. It will be covered at Chapter19

	fn next(&mut self) -> Option<Self::Item> {
		if self.count < 5 {
			self.count += 1;
			Some(self.count)
		} else {
			None
		}
	}
}


#[test]
fn calling_next_directly() {
	let mut counter = Counter::new();

	assert_eq!(counter.next(), Some(1));
	assert_eq!(counter.next(), Some(2));
	assert_eq!(counter.next(), Some(3));
	assert_eq!(counter.next(), Some(4));
	assert_eq!(counter.next(), Some(5));
	assert_eq!(counter.next(), None);
}



#[test]
fn using_other_iterator_trait_methods() {
	let sum: u32 = Counter::new()
		.zip(Counter::new().skip(1)) 
		.map(|(a, b)| a * b) // (0,1) (1,2) (2,3) (3,4) -> 0 2 6 12! -> (5, none) is never produced because zip returns 'None' when either one is 'None'
		.filter(|x| x % 3 == 0) //6 12 remains
		.sum();// 18
	assert_eq!(18, sum);
}

/*
   zip() -> returns a new iterator that will iterate over two other iterators, returning a tuple
   skip() -> creates an iterator that skips the first n elements (It uses 'next' method!!)
*/




