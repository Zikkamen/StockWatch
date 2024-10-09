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

pub struct TiingoClient {
    addr: String,
    token: String,
    stock_analysis_web: StockAnalyserWeb,
}

impl TiingoClient {
    pub fn new(credentials_store: CredentialsStore, stock_analysis_web: StockAnalyserWeb) -> Self {
        TiingoClient{ 
            addr: "wss://api.tiingo.com/iex".to_owned(),
            token: credentials_store.get_token("tiingo.com"),
            stock_analysis_web: stock_analysis_web,
        }
    }

    pub fn print_hello(&mut self, list_of_stocks: &Vec<String>) {
        loop {
            let (client, _response) = match connect(self.addr.clone()) {
                Ok(v) => v,
                Err(e) => panic!("Error creating Tiingo Client: {}", e),
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

            stock_list.push('"');
            stock_list.push_str(stock);
            stock_list.push('"');
        }

        let mut msg_txt = String::new();

        msg_txt.push_str("{");
        msg_txt.push_str("\"eventName\":\"subscribe\",");
        msg_txt.push_str(&format!("\"authorization\":\"{}\",", self.token));
        msg_txt.push_str("\"eventData\": {");
        msg_txt.push_str("\"thresholdLevel\": 0,");
        msg_txt.push_str(&format!("\"tickers\": [{}]", stock_list));
        msg_txt.push_str("}}");

        println!("{}", msg_txt);

        let _ = client.send(Message::Text(msg_txt)).unwrap();
        
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
                    //let _ = self.stock_analysis_web.add_finnhub_data(&text);
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
