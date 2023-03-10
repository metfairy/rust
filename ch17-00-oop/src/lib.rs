#[derive(Debug)]
pub struct Post{
    content:String,
}

pub struct DraftPost {
    content: String,
}

impl Post {
    pub fn new()->DraftPost {
        DraftPost{
            content:String::from(""),
        }
    }

    pub fn content(&self)->&str {
        &self.content
    }

    
}
impl DraftPost {
    pub fn add_text(&mut self,text:&str) {
        self.content.push_str(text);
    }
    pub fn request_review(self)->PendingReviewPost {
        PendingReviewPost{
            content:self.content
        }
    }
}
#[derive(Debug)]
pub struct PendingReviewPost {
    content: String,
}

impl PendingReviewPost {

    pub fn approve(self)->Post {
        Post{
            content:self.content
        }
    }
}