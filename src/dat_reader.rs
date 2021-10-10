use std::io::{Read, BufRead, BufReader};
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
    total_len: i16,
    // 4 bytes
    r#type: i32,
    // 2 bytes
    data_len: i16,
} // 8 bytes

fn buf_reader<T: Read, const size: usize>(reader: &mut T) -> [u8; size] {
    let mut buf = [0; size];
    reader.read(&mut buf[..]);
    buf
}

impl Header {
    pub(crate) fn new<T: Read>(reader: &mut T) -> Header {
        Self {
            total_len: LittleEndian::read_i16(&buf_reader::<T, 2>(reader)),
            r#type: LittleEndian::read_i32(&buf_reader::<T, 4>(reader)),
            data_len: LittleEndian::read_i16(&buf_reader::<T, 2>(reader)),
        }
    }
}

pub struct DatReader {
    buf_reader: BufReader<File>,
}

unsafe fn cast_ref<'a, T>(bytes: &'a [u8]) -> &'a T {
    // assert correct endianness somehow
    if bytes.len() != std::mem::size_of::<T>() {
        assert_eq!(bytes.len(), std::mem::size_of::<T>());
    }
    let ptr: *const u8 = bytes.as_ptr();
    assert_eq!(ptr.align_offset(std::mem::align_of::<T>()), 0);

    ptr.cast::<T>().as_ref().unwrap()
}

impl DatReader {
    pub(crate) fn new(file_path: &str) -> DatReader {
        let file = OpenOptions::new()
            .read(true)
            .open(file_path)
            .expect(format!("can't open file {}", file_path).as_str());
        Self {
            buf_reader: BufReader::new(file)
        }
    }

    pub fn read(&mut self) {
        let mut count = 0i64;
        let mut byte_size = 0i64;

        while !self.buf_reader.fill_buf().unwrap().is_empty() {
            let header = Header::new(&mut self.buf_reader);
            if count >= 119057731 {
                let buf_ref = self.buf_reader.by_ref();
                println!("Count:{}, ByteSize:{}, CurrTotalLen:{},CurrDataLen:{}, buffer bytes:{}",
                         count, byte_size, header.total_len, header.data_len, buf_ref.bytes().count());
            }
            if header.data_len > 0 {
                let mut data = vec![0; header.data_len as usize];
                self.buf_reader.read(&mut data).unwrap();

                if header.r#type == DataType::SZSE_L2_Order as i32 {
                    let order = unsafe { cast_ref::<SzL2Order>(&data) };
                    println!("Order => T:{}, Symbol:{}, ChannelNo:{}, RecID:{}, count:{}",
                             order.time, String::from_utf8_lossy(&order.symbol_code[0..6]), order.channel_no, order.order_id, count);
                    // println!("Order => {:?}", order);
                } else if header.r#type == DataType::SZSE_L2_Transaction as i32 {
                    let trade = unsafe { cast_ref::<SzL2Trans>(&data) };
                    println!("Trade => T:{}, Symbol:{}, ChannelNo:{}, RecID:{}, count:{}",
                             trade.trade_time, String::from_utf8_lossy(&trade.symbol_code[0..6]), trade.set_id, trade.trade_id, count);
                    // println!("Trade => {:?}", trade);
                }
            }
            byte_size += header.total_len as i64;
            count += 1;
        }
    }
}