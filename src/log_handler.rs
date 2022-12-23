use ringbuffer::{ConstGenericRingBuffer, RingBufferRead};

#[derive(Default)]
pub struct LogHandler {
    log_buffer: ConstGenericRingBuffer::<u8, 1024>
}

impl LogHandler {
    pub fn add_logs(&mut self, logs: Vec<u8>) {
        self.log_buffer.extend(logs)
    }

    pub fn take_logs(&mut self) -> Vec<u8> {
        self.log_buffer.drain().collect()
    }
}