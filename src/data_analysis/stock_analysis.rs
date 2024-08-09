use std::collections::HashMap;

use crate::data_parsers::finnhub_parser::parse_finnhub_data;
use crate::data_parsers::eodhd_parser::parse_eodhd_data;
use crate::data_parsers::finnhub_data_row::FinnhubDataRow;
use crate::database_clients::data_web_client::DataWebClient;
use crate::database_clients::data_web_client::DataTradeModel;

pub struct StockInformation {
    trades: HashMap<i64, i64>,
    first_trade: i64,
    num_of_trades: i32,
    time_limit_mil: i64,
}

impl StockInformation {
    pub fn new(time_limit_mil: i64) -> Self {
        StockInformation{ trades: HashMap::new(), first_trade:-1, num_of_trades:0, time_limit_mil: time_limit_mil }
    }

    pub fn add_trade(&mut self, data: &FinnhubDataRow) -> Option<DataTradeModel> {
        if self.first_trade == -1 { self.first_trade = *data.get_time(); }

        let mut output:Option<DataTradeModel> = None;

        if data.get_time() - self.first_trade > self.time_limit_mil {
            output = Some(self.convert_data_to_model());

            self.reset_information(*data.get_time());
        }

        match self.trades.get(data.get_price()) {
            Some(v) => self.trades.insert(*data.get_price(), v + *data.get_volume()),
            None => self.trades.insert(*data.get_price(), *data.get_volume()),
        };

        self.num_of_trades += 1;

        output
    }

    fn convert_data_to_model(&self) -> DataTradeModel {
        let mut list_of_trades:Vec<Vec<i64>> = Vec::new();

        let mut total_price: i64 = 0;
        let mut total_volume: i64 = 0;

        for (key, value) in self.trades.iter() {
            total_price += key * value;
            total_volume += value;

            list_of_trades.push(vec![*key, *value]);
        }

        list_of_trades.sort_by(|a, b| a[0].cmp(&b[0]));

        let n:usize = list_of_trades.len();
        
        let avg_price:i64 = total_price / total_volume;
        let min_price:i64 = list_of_trades[0][0];
        let max_price:i64 = list_of_trades[n-1][0];
        

        DataTradeModel {
            first_trade: self.first_trade,
            num_of_trades: self.num_of_trades,
            volume_moved: total_volume as i32,
            avg_price: avg_price,
            min_price: min_price,
            max_price: max_price,
        }
    }

    fn reset_information(&mut self, time:i64) {
        self.trades.clear();
        self.num_of_trades = 0;
        self.first_trade = time;
    }
} 

pub struct StockAnalyserWeb {
    trade_map: HashMap<String, StockInformation>,
    data_web_client: DataWebClient,
}

impl StockAnalyserWeb {
    pub fn new(data_web_client: DataWebClient) -> Self {
        StockAnalyserWeb{ data_web_client: data_web_client, trade_map: HashMap::new() }
    }

    pub fn add_finnhub_data(&mut self, json_data: &String) -> bool {
        let finnhub_data:Vec<FinnhubDataRow> = parse_finnhub_data(json_data);

        match finnhub_data.len() {
            0 => false,
            _ => { self.add_data(&finnhub_data); true },
        }
    }

    pub fn add_eodhd_data(&mut self, json_data: &String) {
        self.add_data(&parse_eodhd_data(json_data));
    }

    fn add_data(&mut self, data_rows: &Vec<FinnhubDataRow>) {
        for data_row in data_rows {
            if !self.trade_map.contains_key(data_row.get_stockname()) {
                self.trade_map.insert(data_row.get_stockname().clone(), StockInformation::new(1000));
            }

            let stock_info:&mut StockInformation = self.trade_map.get_mut(data_row.get_stockname()).unwrap();

            match (*stock_info).add_trade(&data_row) {
                Some(v) =>
                     match self.data_web_client.add_finnhub_data(data_row.get_stockname(), v) {
                        Ok(()) => (),
                        Err(e) => panic!("Error transmitting to server {}", e),
                     },
                None => return,
            };
        }
    }
}