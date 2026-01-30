// 二进制数据流处理
// 对应原项目的 DataStream.js

pub struct DataStream {
    buffer: Vec<u8>,
    position: usize,
}

impl DataStream {
    pub fn new(buffer: Vec<u8>) -> Self {
        Self {
            buffer,
            position: 0,
        }
    }

    pub fn position(&self) -> usize {
        self.position
    }

    pub fn len(&self) -> usize {
        self.buffer.len()
    }

    pub fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }

    pub fn has_more(&self) -> bool {
        self.position < self.buffer.len()
    }

    pub fn read_byte(&mut self) -> u8 {
        if self.position >= self.buffer.len() {
            panic!("Attempted to read beyond buffer length");
        }
        let value = self.buffer[self.position];
        self.position += 1;
        value
    }

    pub fn read_bytes(&mut self, count: usize) -> Vec<u8> {
        if self.position + count > self.buffer.len() {
            panic!("Attempted to read beyond buffer length");
        }
        let value = self.buffer[self.position..self.position + count].to_vec();
        self.position += count;
        value
    }

    pub fn read_int16(&mut self) -> i16 {
        let bytes = self.read_bytes(2);
        i16::from_le_bytes([bytes[0], bytes[1]])
    }

    pub fn read_uint16(&mut self) -> u16 {
        let bytes = self.read_bytes(2);
        u16::from_le_bytes([bytes[0], bytes[1]])
    }

    pub fn read_int32(&mut self) -> i32 {
        let bytes = self.read_bytes(4);
        i32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
    }

    pub fn read_uint32(&mut self) -> u32 {
        let bytes = self.read_bytes(4);
        u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
    }

    pub fn read_int64(&mut self) -> i64 {
        let bytes = self.read_bytes(8);
        i64::from_le_bytes([
            bytes[0], bytes[1], bytes[2], bytes[3],
            bytes[4], bytes[5], bytes[6], bytes[7],
        ])
    }

    pub fn read_float(&mut self) -> f32 {
        let bytes = self.read_bytes(4);
        f32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
    }

    pub fn read_double(&mut self) -> f64 {
        let bytes = self.read_bytes(8);
        f64::from_le_bytes([
            bytes[0], bytes[1], bytes[2], bytes[3],
            bytes[4], bytes[5], bytes[6], bytes[7],
        ])
    }

    pub fn read_bool(&mut self) -> bool {
        self.read_byte() != 0
    }

    pub fn read_string(&mut self) -> String {
        let length = self.read_uint32() as usize;
        let bytes = self.read_bytes(length);
        String::from_utf8_lossy(&bytes).to_string()
    }

    pub fn skip(&mut self, count: usize) {
        if self.position + count > self.buffer.len() {
            panic!("Attempted to skip beyond buffer length");
        }
        self.position += count;
    }

    pub fn seek(&mut self, position: usize) {
        if position > self.buffer.len() {
            panic!("Attempted to seek beyond buffer length");
        }
        self.position = position;
    }
}