//! # Blog Crate
//!
//! `blog` is an example code from The Rust Programming Language book (aka The Book).
//! This particular implementation is my resolution of the 3 suggestions withing the
//! chapter 17 in section: Trade-offs of the State Pattern
//!

/// Base structure for Post
pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    /// Creates a new Post with Draft as initial State
    ///
    /// # Use
    /// ```
    /// let post: Post = Post::new();
    /// ```
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    /// Adds text to the Post's content
    ///
    /// # Use
    ///
    /// ```
    /// let post: Post = Post::new();
    ///
    /// post.add_text("Initial text");
    /// ```
    pub fn add_text(&mut self, text: &str) {
        // If add_text() can be deconstructed into Some(&str) push the &str to content
        if let Some(t) = self.state.as_ref().unwrap().add_text(text) {
            self.content.push_str(t);
        }
    }

    /// Moves the Post from Draft to Review state
    ///
    /// # Use
    ///
    /// ```
    /// let post: Post = Post::new();
    ///
    /// post.add_text("Initial text");
    ///
    /// post.request_review();
    /// ```
    pub fn request_review(&mut self) {
        // 1*
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }

    /// Moves the Post from Review back to Draft
    ///
    /// # Use
    ///
    /// ```
    /// let post: Post = Post::new();
    ///
    /// post.add_text("Initial text");
    ///
    /// post.request_review();
    ///
    /// post.reject();
    /// ```
    pub fn reject(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.reject())
        }
    }

    /// Moves the Post to Published once two approvals has been granted
    ///
    /// # Use
    ///
    /// ```
    /// let post: Post = Post::new();
    ///
    /// post.add_text("Initial text");
    ///
    /// post.request_review();
    ///
    /// post.approve(); // One more approval would be needed to publish the Post
    /// ```
    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }

    /// Shows the content of the Post
    ///
    /// # Use
    ///
    /// ```
    /// let post: Post = Post::new();
    ///
    /// post.add_text("Initial text");
    ///
    /// post.request_review();
    ///
    /// post.approve();
    /// post.approve(); // Two approvals are required to be published
    ///
    /// post.content();
    /// ```
    pub fn content(&self) -> &str {
        // Returns &str only if state is Published
        self.state.as_ref().unwrap().content(self)
    }
}

trait State {
    /*
        All methods with Box<Self> are created this way so the Post.State can be changed.
        Consumed and replaced to be precise.
    */

    //
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn reject(self: Box<Self>) -> Box<dyn State>;

    // Returns Some(&str) or None by default. Being &str the text to be added/omitted depending
    // on the implementation.
    fn add_text<'a>(&self, text: &'a str) -> Option<&'a str> {
        None
    }

    // Returns the content of the Post or and empty &str by default.
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }
}

// Empty struct that represents the Draft state
struct Draft {}

impl State for Draft {
    // Returns a new review State for the first approval
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview0 {})
    }

    // Returns self as Draft cannot be implemented without approval
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    // Returns self, as Post requires to be in Review to be rejected
    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }

    // Returns the text to be added to the Post's content
    fn add_text<'a>(&self, text: &'a str) -> Option<&'a str> {
        Some(text)
    }
}

// Empty struct that represents the first Review state
struct PendingReview0 {}

impl State for PendingReview0 {
    // Returns self as Post is already in Review state
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    // Returns a second Review state since first approval was provided
    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview1 {})
    }

    // Returns Post back to Draft state
    fn reject(self: Box<Self>) -> Box<dyn State> {
        Box::new(Draft {})
    }
}

// Empty struct that represents the second Review state
struct PendingReview1 {}

impl State for PendingReview1 {
    // Returns self as Post is already in Review state
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    // Returns a Published state for Post as this is the second and last approval required
    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }

    // Returns Post back to Draft state
    fn reject(self: Box<Self>) -> Box<dyn State> {
        Box::new( Draft {})
    }
}

// Empty struct that represents the Published state
struct Published {}

impl State for Published {
    // Returns self as Published Posts do not require further revisions
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    // Returns self as Published Posts cannot be approved again
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    // Returns self as Published Posts cannot be turn back to Drafts
    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }

    // Returns the content in Post
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}

/*
    1*: Since Rust doesn't allow unpopulated values in fields we use Option for state so
        we can left None as value during the .take() method call; this is needed because in
        request_review() the state is consumed (ownership changes) and replaced
*/