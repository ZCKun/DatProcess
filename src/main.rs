use crate::dat_reader::DatReader;

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

fn main() {
    let file_path = "D:/20211011160501/DATA/dat/202110110705.dat";
    println!("{}", file_path);
    let mut reader = DatReader::new(file_path);
    reader.read();

    println!("process done!")
}
