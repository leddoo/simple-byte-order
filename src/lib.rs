pub trait ByteOrder {
    fn read_u8( bytes: [u8; 1]) -> u8;
    fn read_u16(bytes: [u8; 2]) -> u16;
    fn read_u32(bytes: [u8; 4]) -> u32;
    fn read_u64(bytes: [u8; 8]) -> u64;
    fn read_i8( bytes: [u8; 1]) -> i8;
    fn read_i16(bytes: [u8; 2]) -> i16;
    fn read_i32(bytes: [u8; 4]) -> i32;
    fn read_i64(bytes: [u8; 8]) -> i64;
    fn read_f32(bytes: [u8; 4]) -> f32;
    fn read_f64(bytes: [u8; 8]) -> f64;
    
    fn write_u8( value: u8)  -> [u8; 1];
    fn write_u16(value: u16) -> [u8; 2];
    fn write_u32(value: u32) -> [u8; 4];
    fn write_u64(value: u64) -> [u8; 8];
    fn write_i8( value: i8)  -> [u8; 1];
    fn write_i16(value: i16) -> [u8; 2];
    fn write_i32(value: i32) -> [u8; 4];
    fn write_i64(value: i64) -> [u8; 8];
    fn write_f32(value: f32) -> [u8; 4];
    fn write_f64(value: f64) -> [u8; 8];

    fn swap_bytes<const N: usize>(bytes: [u8; N]) -> [u8; N];
}



pub enum LittleEndian {}

impl ByteOrder for LittleEndian {
    fn read_u8( bytes: [u8; 1]) -> u8  { u8::from_le_bytes(bytes)  }
    fn read_u16(bytes: [u8; 2]) -> u16 { u16::from_le_bytes(bytes) }
    fn read_u32(bytes: [u8; 4]) -> u32 { u32::from_le_bytes(bytes) }
    fn read_u64(bytes: [u8; 8]) -> u64 { u64::from_le_bytes(bytes) }
    fn read_i8( bytes: [u8; 1]) -> i8  { i8::from_le_bytes(bytes)  }
    fn read_i16(bytes: [u8; 2]) -> i16 { i16::from_le_bytes(bytes) }
    fn read_i32(bytes: [u8; 4]) -> i32 { i32::from_le_bytes(bytes) }
    fn read_i64(bytes: [u8; 8]) -> i64 { i64::from_le_bytes(bytes) }
    fn read_f32(bytes: [u8; 4]) -> f32 { f32::from_le_bytes(bytes) }
    fn read_f64(bytes: [u8; 8]) -> f64 { f64::from_le_bytes(bytes) }

    fn write_u8( value: u8)  -> [u8; 1] { u8::to_le_bytes(value) }
    fn write_u16(value: u16) -> [u8; 2] { u16::to_le_bytes(value) }
    fn write_u32(value: u32) -> [u8; 4] { u32::to_le_bytes(value) }
    fn write_u64(value: u64) -> [u8; 8] { u64::to_le_bytes(value) }
    fn write_i8( value: i8)  -> [u8; 1] { i8::to_le_bytes(value) }
    fn write_i16(value: i16) -> [u8; 2] { i16::to_le_bytes(value) }
    fn write_i32(value: i32) -> [u8; 4] { i32::to_le_bytes(value) }
    fn write_i64(value: i64) -> [u8; 8] { i64::to_le_bytes(value) }
    fn write_f32(value: f32) -> [u8; 4] { f32::to_le_bytes(value) }
    fn write_f64(value: f64) -> [u8; 8] { f64::to_le_bytes(value) }

    fn swap_bytes<const N: usize>(bytes: [u8; N]) -> [u8; N] {
        #[cfg(target_endian = "little")] {
            bytes
        }
        #[cfg(target_endian = "big")] {
            let mut bytes = bytes;
            bytes.reverse();
            bytes
        }
    }
}



pub enum BigEndian {}

impl ByteOrder for BigEndian {
    fn read_u8( bytes: [u8; 1]) -> u8  { u8::from_be_bytes(bytes)  }
    fn read_u16(bytes: [u8; 2]) -> u16 { u16::from_be_bytes(bytes) }
    fn read_u32(bytes: [u8; 4]) -> u32 { u32::from_be_bytes(bytes) }
    fn read_u64(bytes: [u8; 8]) -> u64 { u64::from_be_bytes(bytes) }
    fn read_i8( bytes: [u8; 1]) -> i8  { i8::from_be_bytes(bytes)  }
    fn read_i16(bytes: [u8; 2]) -> i16 { i16::from_be_bytes(bytes) }
    fn read_i32(bytes: [u8; 4]) -> i32 { i32::from_be_bytes(bytes) }
    fn read_i64(bytes: [u8; 8]) -> i64 { i64::from_be_bytes(bytes) }
    fn read_f32(bytes: [u8; 4]) -> f32 { f32::from_be_bytes(bytes) }
    fn read_f64(bytes: [u8; 8]) -> f64 { f64::from_be_bytes(bytes) }

    fn write_u8( value: u8)  -> [u8; 1] { u8::to_be_bytes(value) }
    fn write_u16(value: u16) -> [u8; 2] { u16::to_be_bytes(value) }
    fn write_u32(value: u32) -> [u8; 4] { u32::to_be_bytes(value) }
    fn write_u64(value: u64) -> [u8; 8] { u64::to_be_bytes(value) }
    fn write_i8( value: i8)  -> [u8; 1] { i8::to_be_bytes(value) }
    fn write_i16(value: i16) -> [u8; 2] { i16::to_be_bytes(value) }
    fn write_i32(value: i32) -> [u8; 4] { i32::to_be_bytes(value) }
    fn write_i64(value: i64) -> [u8; 8] { i64::to_be_bytes(value) }
    fn write_f32(value: f32) -> [u8; 4] { f32::to_be_bytes(value) }
    fn write_f64(value: f64) -> [u8; 8] { f64::to_be_bytes(value) }

    fn swap_bytes<const N: usize>(bytes: [u8; N]) -> [u8; N] {
        #[cfg(target_endian = "little")] {
            let mut bytes = bytes;
            bytes.reverse();
            bytes
        }
        #[cfg(target_endian = "big")] {
            bytes
        }
    }
}


#[cfg(target_endian = "little")]
    pub type NativeEndian = LittleEndian;
#[cfg(target_endian = "big")]
    pub type NativeEndian = BigEndian;


pub mod aliases {
    use super::*;

    pub type LE = LittleEndian;
    pub type BE = BigEndian;
    pub type NE = NativeEndian;
}

