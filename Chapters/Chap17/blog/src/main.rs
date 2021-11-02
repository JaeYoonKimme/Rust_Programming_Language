use blog::Post;

fn main() {
	let mut post = Post::new();

	post.add_text("I read a short fiction last night");
	assert_eq!("", post.content());

	post.request_review();
	assert_eq!("", post.content());

	post.approve();
	assert_eq!("I read a short fiction last night", post.content());
}


