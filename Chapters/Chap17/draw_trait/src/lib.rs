pub trait Draw {
	fn draw(&self);
}

pub struct Screen {
	pub components: Vec<Box<dyn Draw>>,
	//Vector is of type Box<dyn Draw>, which is trait object
	//its for any type inside a Box that implements the Draw triat

	/*
	   It is useful to use triat object rather than generic type parameter.
	   Because generic type parameter can only be used for one concrete type at a time.
	*/
}

impl Screen {
	pub fn run(&self) {
		for component in self.components.iter() {
			component.draw();
		}
	}
}

pub struct Button {
	pub width: u32,
	pub height: u32,
	pub label: String,
}

impl Draw for Button {
	fn draw(&self) {
		//Code to actually draw a button on screen
	}
}
