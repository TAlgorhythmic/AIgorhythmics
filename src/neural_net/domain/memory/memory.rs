pub struct MemoryData {
    importance: u8,
    positivity: u8
}

impl MemoryData {
    fn new(importance: u8, positivity: u8) -> Self {
        Self {importance, positivity}
    }
}

pub trait Memory {
    fn get_data(&self) -> &MemoryData;

    fn get_id(&self) -> i64;
}
