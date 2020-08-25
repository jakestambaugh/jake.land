use chrono::{DateTime, Utc};

#[derive(serde::Serialize)]
pub struct BlogPost {
    pub author: String,
    pub date: DateTime<Utc>,
    pub headline: String,
    pub body: String,
}

impl BlogPost {
    pub fn from_path(Path) -> Result<BlogPost> {
        Ok(Post {
            author: "Jake".into(),
            date: 
            headline: "".into(),
            body: "".into(),
        })
    }
}


