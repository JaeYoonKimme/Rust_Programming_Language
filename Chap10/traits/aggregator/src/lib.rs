//Delclaring 'Summary' traits
//Each type implementing this trait must provide its own methods('Summarize')
pub trait Summary {
	fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
		format!("(Read more from {} ...)",self.summarize_author())
		//You can define default behavior, and also can call other methods in the same trait
		//If you don't want it, you can just write only method signature and semicolon
	}
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
	fn summarize_author(&self) -> String {
		format!("{}",self.author)
	}

    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
	fn summarize_author(&self) -> String {
		format!("@{}", self.username)
	}
}
