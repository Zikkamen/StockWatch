use std::thread;
use std::time::Duration;

use websocket::{ClientBuilder, OwnedMessage, sync::Client, stream::sync::NetworkStream};

use crate::values_store::credentials_store::CredentialsStore;
use crate::data_analysis::stock_analysis::StockAnalyserWeb;

pub struct FinnhubClient {
    addr: String,
    stock_analysis_web: StockAnalyserWeb,
}

impl FinnhubClient {
    pub fn new(credentials_store: CredentialsStore, stock_analysis_web: StockAnalyserWeb) -> Self {
        FinnhubClient{ 
            addr: format!("wss://ws.finnhub.io?token={}", credentials_store.get_token("Finnhub.io")),
            stock_analysis_web: stock_analysis_web,
        }
    }

    pub fn print_hello(&mut self, list_of_stocks: &Vec<String>) {
        println!("{}", self.addr);
        loop {
            let mut client = match ClientBuilder::new(&self.addr) {
                Ok(v) => v,
                Err(e) => {
                    println!("Error creating builder {e}");

                    thread::sleep(Duration::from_millis(20_000));

                    continue;
                }
            };

            let client = match client.connect(None) {
                Ok(v) => v,
                Err(e) => {
                    println!("Error connecting client {e}");

                    thread::sleep(Duration::from_millis(20_000));

                    continue;
                }
            };

            self.start_websocket(client, list_of_stocks);

            thread::sleep(Duration::from_millis(1000));
        }
    }

    fn start_websocket(&mut self, mut client: Client<Box<dyn NetworkStream + std::marker::Send>>, stock_config_list: &Vec<String>) {
        for stock in stock_config_list.into_iter() {
            let message = OwnedMessage::Text(format!("{{\"type\":\"subscribe\",\"symbol\":\"{}\"}}", stock));
            client.send_message(&message).unwrap();
            println!("Subscribed to {}", stock);
        }
        

        loop {
            let msg = match client.recv_message() {
                Ok(p) => p,
                Err(e) => {
                    println!("Error receiving message {} \n Closing Client", e);
                    let _ = client.send_message(&OwnedMessage::Close(None));
                    break;
                },
            };

            match msg {
                OwnedMessage::Text(text) => {
                    let _ = self.stock_analysis_web.add_finnhub_data(&text);
                    println!("{}", text);
                }
                OwnedMessage::Close(_) => {
                    let _ = client.send_message(&OwnedMessage::Close(None));
                    break;
                }
                OwnedMessage::Ping(_) => {
                    println!("Received Ping. Sending Pong");
                    client.send_message(&OwnedMessage::Pong(Vec::new())).unwrap();
                }
                _ => {
                    println!("Sending Ping");
                    client.send_message(&OwnedMessage::Ping(Vec::new())).unwrap();
                },
            }
        }
    }
}
