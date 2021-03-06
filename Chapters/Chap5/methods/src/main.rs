#[derive(Debug)]
struct Rectangle {
	width: u32,
	height: u32,
}

impl Rectangle {
	//basic method
	fn area(&self) -> u32 {
		self.width * self.height
	}

	//Get other parameter (not a self)
	fn can_hold(&self, other: &Rectangle) -> bool {
		self.width > other.width && self.height > other.height
	}

	//Associated functions
	fn square(size: u32) -> Rectangle {
		Rectangle {
			width: size,
			height: size,
		}
	}
}

fn main() {
	let rect1 = Rectangle {
		width: 30,
		height: 50,
	};

	println!(
		"The area of the rect1 is {} square pixels.",
		rect1.area()
	);


	let rect2 = Rectangle {
		width: 10,
		height: 40,
	};

	println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));


	let sqr1 = Rectangle::square(30);
	println!(
		"The area of the sqr1 is {} square pixels.",
		sqr1.area()
	);



}
