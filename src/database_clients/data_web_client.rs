use std::{ 
    thread, 
    net::TcpStream, 
    time::Duration,
    collections::VecDeque,
    sync::{RwLock, Arc}
};

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

pub struct DataWebClient {
    addr: String,
    update_queue: Arc<RwLock<VecDeque<String>>>,
}

impl DataWebClient {
    pub fn new(addr: &str) -> Self {
        let update_queue = Arc::new(RwLock::new(VecDeque::new()));

        DataWebClient{ addr: addr.to_owned(), update_queue: update_queue }
    }

    pub fn add_finnhub_data(&mut self, list_of_trades:Vec<DataTradeModel>) {
        let mut update_queue = self.update_queue.write().unwrap();

        for database_model in list_of_trades.into_iter() {
            update_queue.push_back(stockdata_to_json(database_model));
        }
    }

    pub fn start_client(&self) -> Vec<String> {
        let (mut client, _response) = connect(&self.addr).unwrap();
        let stock_list = init_client(&mut client);
        
        let addr_clone = self.addr.clone();
        let update_queue_clone = self.update_queue.clone();

        thread::spawn(move || {
            loop {
                update_polling(&mut client, &update_queue_clone);

                thread::sleep(Duration::from_millis(1000));

                client = match connect(&addr_clone) {
                    Ok((c, _r)) => c,
                    Err(_) => {
                        println!("Error connecting to StockDatastore");

                        continue;
                    },
                };

                let _ = init_client(&mut client);
            }
        });

        stock_list
    }
}

fn init_client(client: &mut WebSocket<MaybeTlsStream<TcpStream>>) -> Vec<String> {
    let msg = match client.read() {
        Ok(p) => p,
        Err(e) => {
            panic!("Error receiving message {} \n Closing Client", e);
        },
    };

    match msg {
        msg @ Message::Text(_) => {
            let text: String = msg.into_text().unwrap();

            text.split('|').map(|s| s.to_string()).filter(|s| s.len() > 0).collect()
        }
        _ => {
            panic!("Received wrong message");
        },
    }
}

fn update_polling(client: &mut WebSocket<MaybeTlsStream<TcpStream>>, update_queue: &Arc<RwLock<VecDeque<String>>>) {
    loop {
        let update = update_queue.write().unwrap().pop_front();

        let update = match update {
            Some(v) => v,
            None => {
                thread::sleep(Duration::from_millis(5));

                continue;
            },
        };

        match client.send(Message::text(&update)){
            Ok(v) => v,
            Err(e) => {
                println!("Error sending Message {}", e);

                update_queue.write().unwrap().push_front(update);

                return;
            },
        };
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

