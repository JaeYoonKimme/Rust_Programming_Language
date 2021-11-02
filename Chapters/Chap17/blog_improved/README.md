#Blog_Example_Improved

- Add a reject method that changes the post’s state from PendingReview back to Draft.

- Require two calls to approve before the state can be changed to Published.

- Allow users to add text content only when a post is in the Draft state.  
Hint: have the state object responsible for what might change about the content but not responsible for modifying the Post.
