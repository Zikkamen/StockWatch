use std::{ error, net::TcpStream };

use tungstenite::{
    connect,
    Message,
    WebSocket,
    stream::MaybeTlsStream
};


pub struct DataTradeModel {
    pub timestamp:i64,
    pub stock_name: String,
    pub stock_interval: usize,

    pub avg_price: f64,
    pub avg_price_open: f64,
    pub min_price: f64,
    pub max_price: f64,

    pub volume_moved: i64,
    pub num_of_trades: i64,
}

impl DataTradeModel {
    pub fn new() -> Self {
        DataTradeModel {
            timestamp: 0,
            stock_name: String::new(),
            stock_interval: 1,

            avg_price: 0.0,
            avg_price_open: -1.0,
            min_price: 0.0,
            max_price: 0.0,
            volume_moved: 0,

            num_of_trades: 0,
        }
    }
}

pub struct DataWebClient {
    client: WebSocket<MaybeTlsStream<TcpStream>>,
}

impl DataWebClient {
    pub fn new(addr: &str) -> Self {
        let (mut client, _response) = connect(addr).unwrap();

        DataWebClient{ client: client }
    }

    pub fn add_finnhub_data(&mut self, database_model:DataTradeModel) -> Result<(), Box<dyn error::Error + 'static>> {    
        match self.client.send(Message::text(&stockdata_to_json(database_model))){
            Ok(v) => v,
            Err(e) => panic!("Error sending Message {}", e),
        }

        Ok(())
    }

    pub fn get_stocklist(&mut self) -> Vec<String> {
        let msg = match self.client.read() {
            Ok(p) => p,
            Err(e) => {
                panic!("Error receiving message {} \n Closing Client", e);
            },
        };

        match msg {
            msg @ Message::Text(_) => {
                let text: String = msg.into_text().unwrap();

                return text.split('|').map(|s| s.to_string()).filter(|s| s.len() > 0).collect();
            }
            _ => {
                panic!("Received wrong message");
            },
        }
    }
}

fn stockdata_to_json(update: DataTradeModel) -> String {
    format!("{{
            \"si\": {},
            \"sn\": \"{}\",
            \"ap\": \"{:.6}\",
            \"op\": {:.6},
            \"mn\": {:.6},
            \"mx\": {:.6},
            \"vm\": {},
            \"nt\": {},
            \"t\": {}
        }}",
        update.stock_interval,
        update.stock_name,
        update.avg_price / 100.0,
        update.avg_price_open / 100.0,
        update.min_price / 100.0,
        update.max_price / 100.0,
        update.volume_moved,
        update.num_of_trades,
        update.timestamp,
    )
}

