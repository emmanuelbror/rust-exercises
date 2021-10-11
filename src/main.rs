use improved_blog::Post;

fn main() {
    // New DraftPost is created
    let mut post = Post::new();

    // First text is added
    post.add_text("I ate salad for lunch today");

    // Will panic as DraftPost does not have a content() method like before
    // post.content();

    // Draft is send to the first revision
    let post = post.request_review();

    // Two approves are needed before we can publish a draft
    let post = post.approve();

    // post goes back to Draft
    let post = post.reject();

    // Draft is send to the first revision
    let post = post.request_review();

    // Draft is now fully approved and it's turned into a Post
    let post = post.approve();
    let post = post.approve();

    // Post is published and now we can access the content() method
    assert_eq!("I ate salad for lunch today", post.content());
}
