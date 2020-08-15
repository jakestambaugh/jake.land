pub struct Post {
    pub id: u64,
    pub author: String,
    pub headline: String,
    pub body: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            id: 0,
            author: "Jake".into(),
            headline: "".into(),
            body: "".into(),
        }
    }
}
