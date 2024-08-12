use std::{ error };

use websocket::{ ClientBuilder, Message, sync::Client, stream::sync::NetworkStream};

pub struct DataTradeModel {
    pub timestamp:i64,
    pub last_price: i64,
    pub num_of_trades: i64,
    pub volume_moved: i64,
    pub avg_price:i64,
    pub min_price:i64,
    pub max_price:i64,
    pub min_pos: i64,
    pub max_pos: i64,
}

impl DataTradeModel {
    pub fn new() -> Self {
        DataTradeModel {
            timestamp: 0,
            last_price: 0,
            num_of_trades: 0,
            volume_moved: 0,
            avg_price: 0,
            min_price: 0,
            max_price: 0,
            min_pos: 0,
            max_pos: 0,
        }
    }
}


pub struct DataWebClient {
    addr: String,
    client: Client<Box<(dyn NetworkStream + std::marker::Send + 'static)>>,
}

impl DataWebClient {
    pub fn new(addr: &str) -> Self {
        DataWebClient{ addr: addr.to_string(), client: ClientBuilder::new(addr).unwrap().connect(None).expect("Connection") }
    }

    pub fn add_finnhub_data(&mut self, stock_name: &String, database_model:DataTradeModel) -> Result<(), Box<dyn error::Error + 'static>>{
        
        match self.client.send_message(&Message::text(&stockdata_to_json(stock_name, database_model))){
            Ok(v) => v,
            Err(e) => panic!("Error sending Message {}", e),
        }

        Ok(())
    }
}

fn stockdata_to_json(stock_name: &String, update: DataTradeModel) -> String {
    format!("{{
            \"name\": \"{}\",
            \"last_price\": \"{}\",
            \"avg_price\": {},
            \"min_price\": {},
            \"max_price\": {},
            \"volume_moved\": {},
            \"num_of_trades\": {},
            \"min_pos\": {},
            \"max_pos\": {},
            \"time\": {}
        }}",
        stock_name,
        update.last_price as f64 / 100.0,
        update.avg_price as f64 / 100.0,
        update.min_price as f64 / 100.0,
        update.max_price as f64 / 100.0,
        update.volume_moved,
        update.num_of_trades,
        update.min_pos,
        update.max_pos,
        update.timestamp,
    )
}

