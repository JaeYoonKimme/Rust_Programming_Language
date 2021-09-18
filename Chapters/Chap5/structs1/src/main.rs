//Basic Structs example
struct User {
	username: String,
	email: String,
	sign_in_count: u64,
	active: bool,
}

//Tuple Structs example
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);


fn main() {
	let user1 = User {
		email: String::from("aa@com"),
		username: String::from("yoon"),
		active: true,
		sign_in_count: 1,
	};

	let user2 = User {
		email: String::from("bb@com"),
		username: String::from("name"),
		..user1
	};
	
	println!("{}",user2.username);

	let black = Color(0, 0, 0);
	let origin = Point(0, 0, 0);

}

fn build_user(email: String, username: String) -> User {
	User {
		email,
		username,
		active: true,
		sign_in_count: 1,
	}
}

