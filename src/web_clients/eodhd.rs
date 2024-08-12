use websocket::{ ClientBuilder, Message, OwnedMessage, sync::Client, stream::sync::NetworkStream};

use std::thread;
use std::time::Duration;

use crate::values_store::credentials_store::CredentialsStore;
use crate::data_analysis::stock_analysis::StockAnalyserWeb;

pub struct EodhdClient{
    addr: String,
    stock_analysis_web: StockAnalyserWeb,
}

impl EodhdClient {
    pub fn new(credentials_store: CredentialsStore, stock_analysis_web: StockAnalyserWeb) -> Self {
        EodhdClient{ 
            addr: format!("wss://ws.eodhistoricaldata.com/ws/us?api_token={}", credentials_store.get_token("eodhd.com".to_string())),
            stock_analysis_web: stock_analysis_web,
        }
    }

    pub fn print_hello(&mut self, list_of_stocks: &Vec<String>) {
        loop {
            match ClientBuilder::new(&self.addr).unwrap().connect(None) {
                Ok(mut client) => self.start_websocket(&mut client, list_of_stocks),
                Err(e) => panic!("Error creating Eodhd Client: {}", e),
            };
            thread::sleep(Duration::from_millis(1000));
        }
    }

    fn start_websocket(&mut self, 
        client: &mut Client<Box<(dyn NetworkStream + std::marker::Send + 'static)>>,
        stock_config_list: &Vec<String>) {
        
        for stock in stock_config_list.into_iter() {
            let message = Message::text(format!("{}\"action\":\"subscribe\",\"symbols\":\"{}\"{}", "{", stock, "}"));

            client.send_message(&message).unwrap();

            println!("Subscribed to {}", stock);
        }
        

        loop {
            let message:OwnedMessage = match client.recv_message() {
                Ok(p) => p,
                Err(e) => {
                    println!("Error receiving message {} \n Closing Client", e);
                    let _ = client.send_message(&Message::close());
                    break;
                },
            };

            match message {
                OwnedMessage::Text(txt) => {
                    let text: String = txt.parse().unwrap();
                    let _ = self.stock_analysis_web.add_eodhd_data(&text);
                    println!("{}", text);
                }
                OwnedMessage::Close(_) => {
                    let _ = client.send_message(&Message::close());
                    break;
                }
                OwnedMessage::Ping(data) => {
                    client.send_message(&OwnedMessage::Pong(data)).unwrap();
                }
                _ => (),
            }
        }
    }
}
