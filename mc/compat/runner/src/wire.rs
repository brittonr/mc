use super::{
    VARINT_CONTINUATION_BIT, VARINT_MAX_SHIFT_EXCLUSIVE, VARINT_SEGMENT_BITS,
    VARINT_SEGMENT_BITS_USIZE, VARINT_SEGMENT_MASK,
};
use std::io::{Read, Write};

pub(crate) trait McWrite: Write {
    fn write_varint(&mut self, value: u32) -> Result<(), String> {
        let mut remaining = value;
        loop {
            let mut byte = (remaining & VARINT_SEGMENT_MASK) as u8;
            remaining >>= VARINT_SEGMENT_BITS;
            if remaining != 0 {
                byte |= VARINT_CONTINUATION_BIT;
            }
            self.write_all(&[byte]).map_err(|e| e.to_string())?;
            if remaining == 0 {
                return Ok(());
            }
        }
    }

    fn write_mc_string(&mut self, value: &str) -> Result<(), String> {
        let len = u32::try_from(value.len()).map_err(|e| e.to_string())?;
        self.write_varint(len)?;
        self.write_all(value.as_bytes()).map_err(|e| e.to_string())
    }

    fn write_packet(&mut self, id: u32, payload: &[u8]) -> Result<(), String> {
        let mut body = Vec::new();
        body.write_varint(id)?;
        body.extend_from_slice(payload);
        let mut packet = Vec::new();
        packet.write_varint(u32::try_from(body.len()).map_err(|e| e.to_string())?)?;
        packet.extend_from_slice(&body);
        self.write_all(&packet).map_err(|e| e.to_string())
    }
}

impl<T> McWrite for T where T: Write + ?Sized {}

pub(crate) trait McRead: Read {
    fn read_varint(&mut self) -> Result<u32, String> {
        let mut value = 0u32;
        for shift in (0..VARINT_MAX_SHIFT_EXCLUSIVE).step_by(VARINT_SEGMENT_BITS_USIZE) {
            let mut byte = [0u8; 1];
            self.read_exact(&mut byte).map_err(|e| e.to_string())?;
            value |= u32::from(byte[0] & (VARINT_SEGMENT_MASK as u8)) << shift;
            if byte[0] & VARINT_CONTINUATION_BIT == 0 {
                return Ok(value);
            }
        }
        Err("varint too long".to_string())
    }

    fn read_mc_string(&mut self) -> Result<String, String> {
        let string_len = self.read_varint()? as usize;
        let mut buf = vec![0; string_len];
        self.read_exact(&mut buf).map_err(|e| e.to_string())?;
        String::from_utf8(buf).map_err(|e| e.to_string())
    }
}

impl<T> McRead for T where T: Read + ?Sized {}

#[cfg(test)]
#[path = "wire_colocated_tests.rs"]
mod root_colocated_tests;
