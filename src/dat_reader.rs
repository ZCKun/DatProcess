use std::io::{Read, BufRead, BufReader, SeekFrom, Seek};
use byteorder::{LittleEndian, ByteOrder};
use std::fs::{File, OpenOptions};
use crate::mdt_struct::{SzL2Order, SzL2Trans};
use crate::mdt_type::DataType;
use serde_json::error::Category::Data;

/// dat 头结构
#[derive(Debug)]
#[repr(align(1))]
pub struct Header {
    // 2 bytes
    pub total_len: i16,
    // 4 bytes
    pub r#type: i32,
    // 2 bytes
    pub data_len: i16,
    pub data_len_bytes: [u8; 2]
} // 8 bytes

fn buf_reader<T: Read, const size: usize>(reader: &mut T) -> [u8; size] {
    let mut buf = [0; size];
    reader.read(&mut buf[..]);
    buf
}

impl Header {
    pub(crate) fn new<T: Read>(reader: &mut T) -> Header {

        let total_len_bytes = &buf_reader::<T, 2>(reader);
        let r#type_bytes = &buf_reader::<T, 4>(reader);
        let data_len_bytes = buf_reader::<T, 2>(reader);
        Self {
            total_len: LittleEndian::read_i16(total_len_bytes),
            r#type: LittleEndian::read_i32(r#type_bytes),
            data_len: LittleEndian::read_i16(&data_len_bytes),
            data_len_bytes,
        }
    }
}

pub struct DatReader<CB>
where
    CB: FnMut(&Header, &Vec<u8>),
{
    buf_reader: BufReader<File>,
    callback: CB,
}

impl<CB> DatReader<CB>
where
    CB: FnMut(&Header, &Vec<u8>),
{
    pub(crate) fn new(file_path: &str, c: CB) -> Self {
        let file = OpenOptions::new()
            .read(true)
            .open(file_path)
            .expect(format!("can't open file {}", file_path).as_str());
        Self {
            buf_reader: BufReader::new(file),
            callback: c,
        }
    }

    pub fn read(&mut self) {
        let mut count = 0i64;
        let mut byte_size = 0i64;

        while !self.buf_reader.fill_buf().unwrap().is_empty() {
            let header = Header::new(&mut self.buf_reader);

            if header.data_len > 0 {
                let mut data = vec![0; header.data_len as usize];
                self.buf_reader.read(&mut data).unwrap();

                (self.callback)(&header, &data);
            }
            byte_size += header.data_len as i64 + 8;
            count += 1;
        }
    }
}