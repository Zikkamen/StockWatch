use std::thread;
use std::time::Duration;
use std::net::TcpStream;

use tungstenite::{
    connect,
    Message,
    WebSocket,
    stream::MaybeTlsStream
};

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
            let (client, _response) = match connect(self.addr.clone()) {
                Ok(v) => v,
                Err(e) => panic!("Error creating Finnhub Client: {}", e),
            };

            self.start_websocket(client, list_of_stocks);

            thread::sleep(Duration::from_millis(1000));
        }
    }

    fn start_websocket(&mut self, mut client: WebSocket<MaybeTlsStream<TcpStream>>, stock_config_list: &Vec<String>) {
        for stock in stock_config_list.into_iter() {
            let message = Message::Text(format!("{{\"type\":\"subscribe\",\"symbol\":\"{}\"}}", stock));
            client.send(message).unwrap();
            println!("Subscribed to {}", stock);
        }
        

        loop {
            let msg = match client.read() {
                Ok(p) => p,
                Err(e) => {
                    println!("Error receiving message {} \n Closing Client", e);
                    let _ = client.send(Message::Close(None));
                    break;
                },
            };

            match msg {
                msg @ Message::Text(_) => {
                    let text: String = msg.into_text().unwrap();
                    let _ = self.stock_analysis_web.add_finnhub_data(&text);
                    println!("{}", text);
                }
                _msg @ Message::Close(_) => {
                    let _ = client.send(Message::Close(None));
                    break;
                }
                _msg @ Message::Ping(_) => {
                    println!("Received Ping. Sending Pong");
                    client.send(Message::Pong(Vec::new())).unwrap();
                }
                _ => {
                    println!("Sending Ping");
                    client.send(Message::Ping(Vec::new())).unwrap();
                },
            }
        }
    }
}
