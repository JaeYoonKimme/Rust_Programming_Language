#[derive(Debug)] 
enum UsState {
	Alabama,
	Alaska,
}

#[derive(Debug)]
enum Coin {
	Penny,
	Nickel,
	Dime,
	Quarter(UsState),
}

fn main() {
	let some_u8_value = Some(0u8);
	if let Some(3) = some_u8_value {
		println!("three");
	}
	
	let coin = Coin::Quarter(UsState::Alabama);

	let mut count = 0;
	match coin {
		Coin::Quarter(UsState::Alabama) => println!("State quarter from {:?}!", coin),
		_ => count += 1,
    }


	if let Coin::Quarter(UsState::Alaska) = coin {
		println!("State quarter from {:?}!", coin);
	} else {
		count += 1;
	}

	println!("count : {}",count);
}
