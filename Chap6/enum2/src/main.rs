enum Message {
	Quit,					//No data associated
	Move { x: i32, y: i32}, //Anonymous struct
	Write(String),			//String
	ChangeColor(i32, i32, i32), //i32
}

//Enum also can have method using "impl"
impl Message {
	fn call(&self) {
		//action
	}
}


fn main() {
	let m = Message::Write(String::from("hi"));
	m.call();

	let mut n = Message::Move{x: 5, y:5};

	match n {
		Message::Move{x,y} => {	x = 10;
								println!("{}",x);
		},
		_ => (),
	}

}
