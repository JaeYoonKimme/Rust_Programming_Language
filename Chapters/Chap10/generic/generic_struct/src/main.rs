//We can define structs to use generic type
struct Point<T> { //mutiple generic type can be used.
	x: T,
	y: T,
}

struct Point2<T, U> { //mutiple generic type can be used.
	x: T,
	y: U,
}

//We can implement methods of structs and enum using generic
impl<T> Point<T> {
	fn x(&self) -> &T {
		&self.x
	}
}

//We can limit specific type 
//Point<f32> will have this method, but others don't
impl Point<f32> {
	fn distance_from_origin(&self) -> f32 {
		(self.x.powi(2) + self.y.powi(2)).sqrt()
	}
}

//We can take another genernic Point! differnt from <T,U>
impl<T, U> Point2<T, U> {
	fn mixup<V, W>(self, other: Point2<V, W>) -> Point2<T, W> {
		Point2 {
			x: self.x,
			y: other.y,
		}
	}
}

fn main() {
	let integer = Point { x: 5, y: 10};
	let float = Point { x: 1.0, y: 4.0};

	//let wont_work = Point {x: 5, y: 4.0}; -> it will make compile error
	let integer_and_float = Point2 {x: 5, y: 4.0};


	println!("integer.x = {}, float.x = {}",integer.x(),float.x());

	let p1 = Point2 { x: 5, y: 10.4 };
	let p2 = Point2 { x: "Hello", y: 'c' };

	let p3 = p1.mixup(p2);

	println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}

//We can define enum to holds generic!
//Recall the definition of Option and Result enum.



/*
Generics are used at compile time. so it is not slower than concrete types. 
This is called 
'Monomorphization' 
->Process of turning generic code into specific code 
by filling in the concrete types that are used when compiled.
*/
