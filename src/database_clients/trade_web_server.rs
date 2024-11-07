use std::{
    thread,
    sync::{Arc, RwLock},
    time::Duration,
    collections::VecDeque,
    net::TcpListener,
};

use tungstenite::{accept, Message};

use crate::data_analysis::finnhub_data_row::FinnhubDataRow;

pub struct TradeWebServer {
    ip_server: String,
    update_queue: Arc<RwLock<VecDeque<String>>>,
}

impl TradeWebServer {
    pub fn new(ip_server: &str) -> Self {
        TradeWebServer { 
            ip_server: ip_server.to_string(), 
            update_queue: Arc::new(RwLock::new(VecDeque::new())),
        }
    }

    pub fn add_trade(&mut self, trade:FinnhubDataRow) {
        let n = self.update_queue.read().unwrap().len();

        if n > 10000 {
            let _ = self.update_queue.write().unwrap().pop_front();
        }

        self.update_queue.write().unwrap().push_back(trade.to_string());
    }

    pub fn start_server(&self) {
        let server = TcpListener::bind(self.ip_server.clone()).unwrap();
        let update_queue_clone = self.update_queue.clone();

        thread::spawn(move || {
            for stream in server.incoming() {
                let stream = match stream {
                    Ok(v) => v,
                    Err(_) => continue,
                };

                let mut websocket = match accept(stream) {
                    Ok(v) => v,
                    Err(_) => continue,
                };

                loop {
                    let update = update_queue_clone.write().unwrap().pop_front();
            
                    let update = match update {
                        Some(v) => v,
                        None => {
                            thread::sleep(Duration::from_millis(1));
            
                            continue;
                        },
                    };
            
                    match websocket.send(Message::text(&update)){
                        Ok(v) => v,
                        Err(e) => {
                            println!("Error sending Message {}", e);
            
                            update_queue_clone.write().unwrap().push_front(update);
            
                            break;
                        },
                    };
                }
            }
        });
    }
}