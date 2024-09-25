use std::thread;
use std::time::Duration;
use std::net::TcpStream;
use std::collections::HashMap;

use tungstenite::{
    connect,
    Message,
    WebSocket,
    stream::MaybeTlsStream
};

use crate::values_store::credentials_store::CredentialsStore;
use crate::data_analysis::stock_analysis::StockAnalyserWeb;

pub struct TwelveClient{
    addr: String,
    stock_analysis_web: StockAnalyserWeb,
}

impl TwelveClient {
    pub fn new(credentials_store: CredentialsStore, stock_analysis_web: StockAnalyserWeb) -> Self {
        TwelveClient{ 
            addr: format!("wss://ws.twelvedata.com/v1/quotes/price?apikey={}", credentials_store.get_token("twelvedata.com".to_string())),
            stock_analysis_web: stock_analysis_web,
        }
    }

    pub fn print_hello(&mut self, list_of_stocks: &Vec<String>) {
        loop {
            let (client, _response) = match connect(self.addr.clone()) {
                Ok(v) => v,
                Err(e) => {
                    println!("Error creating Twelve Data Client: {}", e);
                    thread::sleep(Duration::from_millis(20_000));
                    continue;
                },
            };

            self.start_websocket(client, list_of_stocks);

            thread::sleep(Duration::from_millis(1000));
        }
    }

    fn start_websocket(&mut self, mut client: WebSocket<MaybeTlsStream<TcpStream>>, stock_config_list: &Vec<String>) {
        let mut stock_list = String::new();
        
        for stock in stock_config_list.into_iter() {
            if stock_list.len() > 0 {
                stock_list.push(',');
            }

            stock_list.push_str(stock);
        }

        println!("{}", format!("{{\"action\":\"subscribe\",\"params\"{{\"symbols\":\"{}\"}}}}", stock_list));

        let msg = Message::Text(format!("{{\"action\":\"subscribe\",\"params\": {{\"symbols\":\"{}\"}}}}", stock_list));
        client.send(msg).unwrap();
        println!("Subscribed to {}", stock_list);

        let mut last_data = HashMap::<String, i64>::new();
        
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
                    let _ = self.stock_analysis_web.add_twelve_data(&text, &mut last_data);
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
