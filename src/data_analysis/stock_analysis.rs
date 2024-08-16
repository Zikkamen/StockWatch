use std::collections::{HashMap};
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;

use crate::data_parsers::finnhub_parser::parse_finnhub_data;
use crate::data_parsers::eodhd_parser::parse_eodhd_data;

use crate::database_clients::data_web_client::DataWebClient;
use crate::database_clients::data_web_client::DataTradeModel;

use crate::data_analysis::finnhub_data_row::FinnhubDataRow;
use crate::data_analysis::candle_stick_service::CandleStickService;

pub struct StockAnalyserWeb {
    trade_map: Arc<RwLock<HashMap<String, CandleStickService>>>,
}

impl StockAnalyserWeb {
    pub fn new(data_web_client: DataWebClient) -> Self {
        let trade_map_arc = Arc::new(RwLock::new(HashMap::new()));
        let trade_map_arc_clone = Arc::clone(&trade_map_arc);

        thread::spawn(|| {
            start_thread(trade_map_arc_clone, data_web_client);
        });

        StockAnalyserWeb{ 
            trade_map: trade_map_arc,
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
                self.trade_map.write().unwrap().insert(
                    data_row.s.clone(),
                    CandleStickService::new(data_row.s.clone()),
                );
            }

            let mut tmp_trade_map = self.trade_map.write().unwrap();
            let candle_stick_service:&mut CandleStickService = tmp_trade_map.get_mut(&data_row.s).unwrap();

            candle_stick_service.add_trade(&data_row);
        }
    }
}

fn start_thread(trade_map: Arc<RwLock<HashMap<String, CandleStickService>>>, mut data_web_client: DataWebClient) {
    loop {
        thread::sleep(Duration::from_millis(1000));

        let mut list_of_trades:Vec<DataTradeModel> = Vec::new();

        for (key, value) in trade_map.write().unwrap().iter_mut() {
            for trade in value.get_trades().into_iter() {
                list_of_trades.push(trade);
            }
        }

        for trade in list_of_trades.into_iter() {
            match data_web_client.add_finnhub_data(trade) {
                Ok(_v) => (),
                Err(e) => panic!("Error sending data using webclient {}", e),
            };
        }
    }
}
