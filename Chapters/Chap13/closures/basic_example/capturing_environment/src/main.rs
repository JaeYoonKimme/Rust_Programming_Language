fn main() {
	let x = 4;

	let equal_to_x = |z| z == x;

	let y = 4;

	assert!(equal_to_x(y));

}


/*
Here, even though x is not one of the parameters of equal_to_x, 
the equal_to_x closure is allowed to use the x variable 
that’s defined in the same scope that equal_to_x is defined in.

Otherwise, this will make error..

fn main() {
    let x = 4;

    fn equal_to_x(z: i32) -> bool {
        z == x
    }

    let y = 4;

    assert!(equal_to_x(y));
}
*/
