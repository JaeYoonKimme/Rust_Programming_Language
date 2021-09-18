fn main() {
	let v = vec![1, 2, 3, 4, 5];
	let v_iter = v.iter();

	for val in v_iter {
		println!("Got: {}", val);
	}
}


#[test]
fn iterator_demonstration() {
	let v = vec![1, 2, 3];
	let mut v_iter = v.iter(); //We have to make it mutable because 'next()' changes iterators internal state.

	assert_eq!(v_iter.next(), Some(&1));
	assert_eq!(v_iter.next(), Some(&2));
	assert_eq!(v_iter.next(), Some(&3));
	assert_eq!(v_iter.next(), None);
}

#[test]
fn iterator_sum() {
	let v = vec![1, 2, 3];
	let mut v_iter = v.iter();

	let total = v_iter.sum(); //Sum() takes ownership of 'v_iter' because it internally uses 'next()'

	assert_eq!(6, total);

	//We can not access 'v_iter' anymore because its ownership moved so 'sum()' function scope
	//Maybe.... sum(self) ?
}
