#[derive(Debug)]
struct Rectangle {
	width: u32,
	height: u32,
}

fn main() {
	//ver1
	let width1 = 30;
	let height1 = 50;

	println!(
			"The area of the rectangle is {} square pixels.", 
			area(width1, height1)
	);


	//ver2
	let rect1 = (30, 50);
	println!(
			"The area of the rectangle is {} square pixels.", 
			area_tuple(rect1)
	);


	//ver3
	let rect2 = Rectangle {
		width: 30,
		height: 50,
	};

	println!(
			"The area of the rectangle is {} square pixels.", 
			area_struct(&rect2)
	);


	println!("rect2 is {:#?}",rect2);
}

fn area(width: u32, height: u32) -> u32 {
	width * height
}

fn area_tuple(dimensions: (u32, u32)) -> u32 {
	dimensions.0 * dimensions.1
}

fn area_struct(rectangle: &Rectangle) -> u32 {
	rectangle.width * rectangle.height
}
