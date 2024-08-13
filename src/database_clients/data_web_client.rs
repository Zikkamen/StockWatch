use std::{ error };

use websocket::{ ClientBuilder, OwnedMessage, Message, sync::Client, stream::sync::NetworkStream};

pub struct DataTradeModel {
    pub timestamp:i64,

    pub min_price: i64,
    pub bottom_25p: i64,
    pub median_price: i64,
    pub top_25p: i64,
    pub max_price: i64,

    pub volume_moved: i64,
    pub num_of_trades: i64,
}

impl DataTradeModel {
    pub fn new() -> Self {
        DataTradeModel {
            timestamp: 0,
            min_price: 0,
            bottom_25p: 0,
            median_price: 0,
            top_25p: 0,
            max_price: 0,
            volume_moved: 0,
            num_of_trades: 0,
        }
    }
}

pub struct DataWebClient {
    client: Client<Box<(dyn NetworkStream + std::marker::Send + 'static)>>,
}

impl DataWebClient {
    pub fn new(addr: &str) -> Self {
        DataWebClient{ client: ClientBuilder::new(addr).unwrap().connect(None).expect("Connection") }
    }

    pub fn add_finnhub_data(&mut self, stock_name: &str, database_model:DataTradeModel) -> Result<(), Box<dyn error::Error + 'static>> {    
        match self.client.send_message(&Message::text(&stockdata_to_json(stock_name, database_model))){
            Ok(v) => v,
            Err(e) => panic!("Error sending Message {}", e),
        }

        Ok(())
    }

    pub fn get_stocklist(&mut self) -> Vec<String> {
        let message:OwnedMessage = match self.client.recv_message() {
            Ok(p) => p,
            Err(e) => {
                panic!("Error receiving message {} \n Closing Client", e);
            },
        };

        match message {
            OwnedMessage::Text(txt) => {
                let text: String = txt.parse().unwrap();

                return text.split('|').map(|s| s.to_string()).filter(|s| s.len() > 0).collect();
            }
            _ => {
                panic!("Received wrong message");
            },
        }
    }
}

fn stockdata_to_json(stock_name: &str, update: DataTradeModel) -> String {
    format!("{{
            \"name\": \"{}\",
            \"min_price\": \"{}\",
            \"bottom_25p\": {},
            \"median_price\": {},
            \"top_25p\": {},
            \"max_price\": {},
            \"volume_moved\": {},
            \"num_of_trades\": {},
            \"timestamp\": {}
        }}",
        stock_name,
        update.min_price as f64/ 100.0,
        update.bottom_25p as f64/ 100.0,
        update.median_price as f64 / 100.0,
        update.top_25p as f64/ 100.0,
        update.max_price as f64 / 100.0,
        update.volume_moved,
        update.num_of_trades,
        update.timestamp,
    )
}

