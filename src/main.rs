use blog::Post;

fn main() {
    // New Draft is created
    let mut post = Post::new();

    // First text is added. Content is not Published therefore is content() returns an empty &str
    post.add_text("I ate a salad for lunch today"); // 1
    assert_eq!("", post.content());

    // Post is moved to Review. content() still returns an empty &str
    post.request_review();
    assert_eq!("", post.content());

    // Won't be added because Post is in Review
    post.add_text("\nIt was delicious!"); // 2

    // Post goes back to Draft
    post.reject();

    // This time text will be added as Post is in Draft state
    post.add_text("\nIt was delicious!"); // 3

    // Post is moved back to Review
    post.request_review();

    // Two approvals required before the Post can be Published
    post.approve();
    post.approve();

    /*
        Two approvals were given; Post is now Published.
        The left string is the concatenation of the previous string slices (1 & 3).
        post.content() now returns the correct string slice since all the criteria
        is met for the Post to be in Publish state.
    */
    assert_eq!("I ate a salad for lunch today\nIt was delicious!", post.content());
}
