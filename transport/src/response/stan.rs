#[derive(Debug)]
pub struct Response {
    pub topic_res: String,
    pub response: Vec<u8>,
}

impl Response {
    pub fn new(topic_res: &str, response: Vec<u8>) -> Self {
        Self {
            topic_res: topic_res.to_string(),
            response,
        }
    }
}