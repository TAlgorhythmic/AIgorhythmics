use super::memory::{Memory, MemoryData};

pub struct AudioMemory {
    data: MemoryData,
    id: i64
}

impl Memory for AudioMemory {

    fn get_data(&self) -> &MemoryData {
        &self.data
    }

    fn get_id(&self) -> i64 {
        self.id
    }
}
