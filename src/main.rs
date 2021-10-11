use improved_blog::Post;

fn main() {
    // New DraftPost is created
    let mut post = Post::new();

    // First text is added
    post.add_text("I ate salad for lunch today");

    // Will panic as DraftPost does not have a content() method like before
    // post.content();

    let post = post.request_review();

    // Two approves are needed before we can publish a post
    let post = post.approve();

    let post = post.reject();

    let post = post.request_review();
    let post = post.approve();
    let post = post.approve();

    assert_eq!("I ate salad for lunch today", post.content());
}
