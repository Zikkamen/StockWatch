use std::{ error };

use websocket::{ ClientBuilder, Message, OwnedMessage, sync::Client, stream::sync::NetworkStream};
use crate::data_analysis::stock_analysis::StockAnalyserWeb;

pub struct DataTradeModel {
    pub first_trade:i64,
    pub num_of_trades: i32,
    pub volume_moved: i32,
    pub avg_price:i64,
    pub min_price:i64,
    pub max_price:i64,
}


pub struct DataWebClient {
    client: Client<Box<(dyn NetworkStream + std::marker::Send + 'static)>>,
}

impl DataWebClient {
    pub fn new(addr: &str) -> Self {
        DataWebClient{ client: ClientBuilder::new(addr).unwrap().connect(None).expect("Connection") }
    }

    pub fn add_finnhub_data(&mut self, stock_name: &String, database_model:DataTradeModel) -> Result<(), Box<dyn error::Error + 'static>>{
        self.client.send_message(&Message::text(&stockdata_to_json(stock_name, database_model))).unwrap();

        Ok(())
    }
}

fn stockdata_to_json(stock_name: &String, update: DataTradeModel) -> String {
    format!("{{
            \"name\": \"{}\", 
            \"avg_price\": {}, 
            \"min_price\": {}, 
            \"max_price\": {}, 
            \"volume_moved\": {}, 
            \"num_of_trades\": {}, 
            \"time\": {}
        }}",
        stock_name,
        update.avg_price as f64 / 100.0,
        update.min_price as f64 / 100.0,
        update.max_price as f64 / 100.0,
        update.volume_moved,
        update.num_of_trades,
        update.first_trade,
    )
}

