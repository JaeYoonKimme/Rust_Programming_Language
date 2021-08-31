use aggregator::{self, Summary, Tweet};


fn main() {
	let tweet = Tweet {
	username: String::from("horse_ebooks"),
	content: String::from(
			"of course, as you probably already know, people",
			),
	reply: false,
	retweet: false,
	};

	println!("1 new tweet: {}", tweet.summarize());


	notify(&tweet);
}


//But we can't implement external traits on external types.



//Traits as Parameters
//We can define a 'notify' function that calls the 'summarize' method on its 'item' parameter
pub fn notify(item: &impl Summary) {
	println!("Breaking news! {}",item.summarize());
}



