use crate::topic::Topic;

pub trait Loader {
    fn load(self) -> Result<Vec<Topic>, ()>;
}
