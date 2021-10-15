use crate::dat_reader::{DatReader, Header};
use crate::mdt_struct::{SzL2Order, SzL2Trans};
use crate::mdt_type::DataType;

mod mdt_struct;
mod mdt_type;
mod dat_reader;

#[derive(serde::Serialize, serde::Deserialize)]
struct Item {
    data_type: i32,
    time: i64,
    id: i64,
    side: Option<String>,
    price: f64,
    qty: i64,
    bid_id: Option<i64>,
    ask_id: Option<i64>
}

#[derive(serde::Serialize, serde::Deserialize)]
struct Data {
    local_time:String,
    item:Item
}

struct TickDump {
    file_path: String,
    symbol: String,
    orders: Vec<String>,
    trades: Vec<String>
}

unsafe fn cast_ref<'a, T>(bytes: &'a [u8]) -> &'a T {
    // assert correct endianness somehow
    assert_eq!(bytes.len(), std::mem::size_of::<T>());

    let ptr: *const u8 = bytes.as_ptr();
    assert_eq!(ptr.align_offset(std::mem::align_of::<T>()), 0);

    ptr.cast::<T>().as_ref().unwrap()
}

impl TickDump {

    pub fn new(file_path: &str, symbol: &str) -> Self {
        Self {
            file_path: file_path.to_string(),
            symbol: symbol.to_string(),
            orders: Vec,
            trades: Vec
        }
    }

    pub fn start(&mut self) {
        let mut reader = DatReader::new(file_path, call_back);
        reader.read();
    }

    fn call_back(&mut self, header: &Header, data: &Vec<u8>) {
        if header.r#type == DataType::SZSE_L2_Order as i32 {
            let order = unsafe { cast_ref::<SzL2Order>(&data) };
            let symbol_code = String::from_utf8_lossy(&order.symbol_code[0..6]);

            // if order.symbol_code == self.symbol {
            //     let side =
            //     let item = format!("{},{},{},{},{},{},{},{},{}",
            //                        order.time, order.rec_time, symbol_code,
            //                        order.price, order.qty, order.r#type, order.order_id, order.channel_no);
            //     self.orders.push(item);
            // }

            println!("Order => T:{}, Symbol:{}, ChannelNo:{}, RecID:{}, Type:{}, Code:{}",
                     order.time, symbol_code, order.channel_no, order.order_id, order.r#type, order.code);
            // println!("Order => {:?}", order);
        } else if header.r#type == DataType::SZSE_L2_Transaction as i32 {
            let trade = unsafe { cast_ref::<SzL2Trans>(&data) };
            let symbol_code = String::from_utf8_lossy(&trade.symbol_code[0..6]);
            // println!("Trade => T:{}, Symbol:{}, ChannelNo:{}, RecID:{}, count:{}",
            //          trade.trade_time, String::from_utf8_lossy(&trade.symbol_code[0..6]), trade.set_id, trade.trade_id, count);
            // println!("Trade => {:?}", trade);
        }
    }
}

fn main() {
    let file_path = "D:/20211011160501/DATA/dat/202110110705.dat";
    println!("{}", file_path);

    println!("process done!")
}
