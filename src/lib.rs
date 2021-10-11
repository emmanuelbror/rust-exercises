pub struct Post {
    content: String,
}

pub struct DraftPost {
    content: String,
}

impl Post {
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn request_review(self) -> PendingReviewPost0 {
        PendingReviewPost0 {
            content: self.content,
        }
    }
}

trait Review {
    type T;

    fn approve_draft(self) -> Self::T;
    fn reject_draft(self) -> DraftPost;
}

pub struct PendingReviewPost0 {
    content: String,
}

impl PendingReviewPost0 {
    pub fn approve(self) -> PendingReviewPost1 {
        self.approve_draft()
    }

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

pub struct PendingReviewPost1 {
    content: String,
}

impl PendingReviewPost1 {
    pub fn approve(self) -> Post {
        self.approve_draft()
    }

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