/**
* Memory metadata:
*
* importance: from 0 to 100, how significant the memory is (e.g. a conflictive memory)
* positivity: from 0 to 100, is it a good memory or not (if it will try not to remember)
* is_foundational: whether is one of the first memories created, will serve as a foundation
*/
pub struct Metadata {
    importance: u8,
    positivity: u8,
    is_foundational: bool
}

impl Metadata {
    fn new(importance: u8, positivity: u8, is_foundational: bool) -> Self {
        Self {importance, positivity, is_foundational}
    }
}

pub trait Memory {
    fn get_data(&self) -> &Metadata;

    fn get_id(&self) -> i64;
}
