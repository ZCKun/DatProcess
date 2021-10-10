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
    let file_path = "/home/x2h1z/Downloads/DATA/dat/202109300705.dat";
    let mut reader = DatReader::new(file_path);
    reader.read();

    println!("process done!")
}
