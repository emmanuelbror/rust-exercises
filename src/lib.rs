//! # Blog Crate
//!
//! `blog` is an example code from The Rust Programming Language book (aka The Book).
//! This particular implementation is my resolution of the 3 suggestions withing the
//! chapter 17.3 in section: Encoding States and Behavior as Types
//!

// Base struct for Post
pub struct Post {
    content: String,
}

// Base struct for Draft
pub struct DraftPost {
    content: String,
}

impl Post {
    /// Creates a new DraftPost with an empty String
    ///
    /// # Use
    /// ```
    /// let post= Post::new();
    /// ```
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }

    /// Shows the content of the Post
    ///
    /// # Use
    /// ```
    /// let post = Post::new();
    /// // snip
    /// post.content();
    /// ```
    pub fn content(&self) -> &str {
        &self.content
    }
}

impl DraftPost {
    /// Adds text to the Post's content
    ///
    /// # Use
    /// ```
    /// let post = Post::new();
    ///
    /// post.add_text("Example text");
    /// ```
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    /// Moves the content to Review
    ///
    /// # Use
    /// ```
    /// let post = Post::new();
    ///
    /// post.add_text("Example text");
    ///
    /// let post = post.request_review();
    /// ```
    pub fn request_review(self) -> PendingReviewPost0 {
        PendingReviewPost0 {
            content: self.content,
        }
    }
}

trait Review {
    // Generic associated type for approval
    type T;

    fn approve_draft(self) -> Self::T;
    fn reject_draft(self) -> DraftPost;
}

// Base struct for first review
pub struct PendingReviewPost0 {
    content: String,
}

impl PendingReviewPost0 {
    /// Moves the content to the second approval
    ///
    /// # Use
    /// ```
    /// let post = Post::new();
    ///
    /// post.add_text("Example text");
    ///
    /// let post = post.request_review();
    /// let post = post.approve();
    /// ```
    pub fn approve(self) -> PendingReviewPost1 {
        self.approve_draft()
    }

    /// Returns the content back to Draft
    ///
    /// # Use
    /// ```
    /// let post = Post::new();
    ///
    /// post.add_text("Example text");
    ///
    /// let post = post.request_review();
    /// let post = post.reject();
    /// ```
    pub fn reject(self) -> DraftPost {
        self.reject_draft()
    }
}

impl Review for PendingReviewPost0 {
    type T = PendingReviewPost1;

    fn approve_draft(self) -> Self::T {
        Self::T {
            content: self.content,
        }
    }

    fn reject_draft(self) -> DraftPost {
        DraftPost {
            content: self.content,
        }
    }
}

// Base struct for the second review
pub struct PendingReviewPost1 {
    content: String,
}

impl PendingReviewPost1 {
    /// Moves the content to a Post
    ///
    /// # Use
    /// ```
    /// let post = Post::new();
    ///
    /// post.add_text("Example text");
    ///
    /// let post = post.request_review();
    /// let post = post.approve();
    /// let post = post.approve();
    /// ```
    pub fn approve(self) -> Post {
        self.approve_draft()
    }

    /// Returns the content back to Draft
    ///
    /// # Use
    /// ```
    /// let post = Post::new();
    ///
    /// post.add_text("Example text");
    ///
    /// let post = post.request_review();
    /// let post = post.approve();
    /// let post = post.reject();
    /// ```
    pub fn reject(self) -> DraftPost {
        self.reject_draft()
    }
}

impl Review for PendingReviewPost1 {
    type T = Post;

    fn approve_draft(self) -> Self::T {
        Self::T {
            content: self.content
        }
    }

    fn reject_draft(self) -> DraftPost {
        DraftPost {
            content: self.content
        }
    }
}