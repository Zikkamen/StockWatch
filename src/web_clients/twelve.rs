use std::thread;
use std::time::Duration;
use std::net::TcpStream;
use std::collections::HashMap;
use std::io::ErrorKind;

use native_tls::TlsConnector;
use native_tls::TlsStream;

use tungstenite::{
    Message,
    WebSocket,
    client,
    Error::Io,
};

use crate::values_store::credentials_store::CredentialsStore;
use crate::data_analysis::stock_analysis::StockAnalyserWeb;

pub struct TwelveClient{
    addr: String,
    stock_analysis_web: StockAnalyserWeb,
    last_data: HashMap<String, i64>,
}

impl TwelveClient {
    pub fn new(credentials_store: CredentialsStore, stock_analysis_web: StockAnalyserWeb) -> Self {
        TwelveClient{ 
            addr: format!("wss://ws.twelvedata.com/v1/quotes/price?apikey={}", credentials_store.get_token("twelvedata.com")),
            stock_analysis_web: stock_analysis_web,
            last_data: HashMap::new(),
        }
    }

    pub fn print_hello(&mut self, list_of_stocks: &Vec<String>) {
        loop {
            let connector = TlsConnector::new().unwrap();

            let stream = match TcpStream::connect("ws.twelvedata.com:443") {
                Ok(v) => v,
                Err(e) => {
                    println!("Error connecting TcpStream: {}", e);
                    thread::sleep(Duration::from_millis(20_000));
                    continue;
                },
            };
            
            let stream = match connector.connect("ws.twelvedata.com", stream) {
                Ok(v) => v,
                Err(e) => {
                    println!("Error establishing TLS Connection: {}", e);
                    thread::sleep(Duration::from_millis(20_000));
                    continue;
                },
            };

            let (mut client, _response) = match client(self.addr.clone(), stream) {
                Ok(v) => v,
                Err(e) => {
                    println!("Error creating Twelve Data Client: {}", e);
                    thread::sleep(Duration::from_millis(20_000));
                    continue;
                },
            };

            let _ = client.get_mut().get_mut().set_nonblocking(true);

            self.start_websocket(client, list_of_stocks);

            thread::sleep(Duration::from_millis(1000));
        }
    }

    fn start_websocket(&mut self, mut client: WebSocket<TlsStream<TcpStream>>, stock_config_list: &Vec<String>) {
        let mut stock_list = String::new();
        
        for stock in stock_config_list.into_iter() {
            if stock_list.len() > 0 {
                stock_list.push(',');
            }

            stock_list.push_str(stock);
        }

        println!("{}", format!("{{\"action\":\"subscribe\",\"params\"{{\"symbols\":\"{}\"}}}}", stock_list));

        let msg = Message::Text(format!("{{\"action\":\"subscribe\",\"params\": {{\"symbols\":\"{}\"}}}}", stock_list));
        let _ = client.send(msg).unwrap();
        
        println!("Subscribed to {}", stock_list);

        let mut last_ping: u32 = 0;
        
        loop {
            if last_ping >= 200 {
                let _ = client.send(Message::Text("{\"action\": \"heartbeat\"}".to_string()));
                last_ping = 0;
            }

            last_ping += 1;

            let msg = match client.read() {
                Ok(p) => p,
                Err(e) => match e {
                    Io(ref error) => {
                        match error.kind() {
                            ErrorKind::WouldBlock => {
                                thread::sleep(Duration::from_millis(50));
                                continue;
                            },
                            _ => {
                                println!("Error receiving message {} \n Closing Client", e);
                                let _ = client.send(Message::Close(None));
                                break;
                            }
                        }
                    },
                    _ => {
                        println!("Error receiving message {} \n Closing Client", e);
                        let _ = client.send(Message::Close(None));
                        break;
                    },
                },
            };

            match msg {
                msg @ Message::Text(_) => {
                    let text: String = msg.into_text().unwrap();
                    let _ = self.stock_analysis_web.add_twelve_data(&text, &mut self.last_data);
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
