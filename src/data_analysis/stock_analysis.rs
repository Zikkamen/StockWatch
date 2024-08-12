use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;

use crate::data_parsers::finnhub_parser::parse_finnhub_data;
use crate::data_parsers::eodhd_parser::parse_eodhd_data;
use crate::data_analysis::finnhub_data_row::FinnhubDataRow;
use crate::database_clients::data_web_client::DataWebClient;
use crate::database_clients::data_web_client::DataTradeModel;
use crate::data_analysis::fenwick_tree::FenwickTree;

pub struct StockDataPoint {
    pub timestamp: i64,
    pub price: i64,
    pub volume_moved: i64,
}

pub struct StockInformation {
    trades: VecDeque<StockDataPoint>,
    fenwick_tree: FenwickTree,
    
    total_trades: i64,
    total_price: i64,
    total_volume: i64,
    last_price: VecDeque<(i64, i64)>,
    last_timestamp: i64,
    
    time_limit_mil: i64,
}

impl StockInformation {
    pub fn new(time_limit_mil: i64) -> Self {
        StockInformation{ 
            trades: VecDeque::new(), 
            fenwick_tree: FenwickTree::new(),
            total_trades: 0,
            total_price: 0,
            total_volume: 0,
            last_price: Vec::new().into(),
            last_timestamp: 0,
            time_limit_mil: time_limit_mil,
        }
    }

    pub fn add_trade(&mut self, data: &FinnhubDataRow) {
        if data.t > self.last_timestamp {
            self.last_timestamp = data.t;
        }

        while self.last_price.len() > 0 && self.last_timestamp - self.last_price.front().unwrap().0 > 1000 {
            self.last_price.pop_front();
        } 

        self.last_price.push_back((data.t, data.p));

        while self.trades.len() > 0 {
            match self.trades.front() {
                Some(v) => {
                    if self.last_timestamp - v.timestamp > self.time_limit_mil {
                        self.total_price -= v.price * v.volume_moved;
                        self.total_volume -= v.volume_moved;
                        self.total_trades -= 1;

                        self.fenwick_tree.insert(v.price, -v.volume_moved);

                        self.trades.pop_front();
                    } else {
                        break;
                    }
                },
                None => break,
            }
        }

        self.trades.push_back(StockDataPoint{ timestamp: data.t, price:data.p, volume_moved:data.v });

        self.total_price += data.p * data.v;
        self.total_volume += data.v;
        self.total_trades += 1;

        self.fenwick_tree.insert(data.p, data.v);
    }
} 

pub struct StockAnalyserWeb {
    trade_update: Arc<RwLock<HashSet<String>>>,
    trade_map: Arc<RwLock<HashMap<String, StockInformation>>>,
}

impl StockAnalyserWeb {
    pub fn new(data_web_client: DataWebClient) -> Self {
        let trade_map_arc = Arc::new(RwLock::new(HashMap::new()));
        let trade_map_arc_clone = Arc::clone(&trade_map_arc);

        let trade_update_arc = Arc::new(RwLock::new(HashSet::new()));
        let trade_update_arc_clone = Arc::clone(&trade_update_arc);

        thread::spawn(|| {
            start_thread(trade_map_arc_clone, trade_update_arc_clone, data_web_client)
        });

        StockAnalyserWeb{ 
            trade_map: trade_map_arc,
            trade_update: trade_update_arc,
        }
    }

    pub fn add_finnhub_data(&mut self, json_data: &String) -> bool {
        let finnhub_data:Vec<FinnhubDataRow> = parse_finnhub_data(json_data);

        match finnhub_data.len() {
            0 => false,
            _ => { self.add_data(finnhub_data); true },
        }
    }

    pub fn add_eodhd_data(&mut self, json_data: &String) {
        self.add_data(parse_eodhd_data(json_data));
    }

    fn add_data(&mut self, data_rows: Vec<FinnhubDataRow>) {
        for data_row in data_rows {
            if !self.trade_map.read().unwrap().contains_key(&data_row.s) {
                self.trade_map.write().unwrap().insert(data_row.s.clone(), StockInformation::new(60_000));
            }

            let mut tmp_trade_map = self.trade_map.write().unwrap();
            let stock_info:&mut StockInformation = tmp_trade_map.get_mut(&data_row.s).unwrap();

            (*stock_info).add_trade(&data_row);
            self.trade_update.write().unwrap().insert(data_row.s.clone());
        }
    }
}

fn start_thread(trade_map: Arc<RwLock<HashMap<String, StockInformation>>>,
                trade_update: Arc<RwLock<HashSet<String>>>,
                mut data_web_client: DataWebClient) {
    loop {
        thread::sleep(Duration::from_millis(1000));

        let trade_keys:HashSet<String> = trade_update.read().unwrap().clone();
        trade_update.write().unwrap().clear();

        let trade_map_read = trade_map.read().unwrap();

        for key in trade_keys {
            let value = match trade_map_read.get(&key) {
                Some(v) => v,
                None => continue,
            };

            let mut data_trade_model = DataTradeModel::new();

            let n = value.last_price.len() as i64;
            let mut last_price_sum: i64 = 0;

            for (_time, price) in value.last_price.clone().iter() {
                last_price_sum += price;
            }

            data_trade_model.timestamp = value.last_timestamp;
            data_trade_model.last_price = last_price_sum / n;
            data_trade_model.num_of_trades = value.total_trades;
            data_trade_model.volume_moved = value.total_volume;
            data_trade_model.avg_price = match value.total_volume > 0 {
                true => value.total_price / value.total_volume as i64,
                false => -1,
            };
            data_trade_model.min_price = value.fenwick_tree.find_min();
            data_trade_model.max_price = value.fenwick_tree.find_max();

            let (sm_than, sm_eq_than) = value.fenwick_tree.find_num(data_trade_model.last_price);
            data_trade_model.min_pos = sm_than;
            data_trade_model.max_pos = value.total_volume - sm_eq_than;

            match data_web_client.add_finnhub_data(&key, data_trade_model) {
                Ok(_v) => (),
                Err(e) => panic!("Error sending data using webclient {}", e),
            };
        }
    }
}