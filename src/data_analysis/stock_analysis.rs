use std::collections::HashMap;

use crate::database_clients::postgres_client::PostgresClient;
use crate::database_clients::postgres_client::DatabaseTradeModel;
use crate::data_parsers::finnhub_parser::parse_finnhub_data;
use crate::data_parsers::eodhd_parser::parse_eodhd_data;
use crate::data_parsers::finnhub_data_row::FinnhubDataRow;

pub struct StockInformation {
    trades: HashMap<i64, i64>,
    first_trade: i64,
    num_of_trades: i32,
}

impl StockInformation {
    pub fn new() -> Self {
        StockInformation{ trades: HashMap::new(), first_trade:-1, num_of_trades:0 }
    }

    pub fn add_trade(&mut self, data: &FinnhubDataRow) -> Option<DatabaseTradeModel> {
        if self.first_trade == -1 { self.first_trade = *data.get_time(); }

        let mut output:Option<DatabaseTradeModel> = None;

        if data.get_time() - self.first_trade > 10000 {
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

    fn convert_data_to_model(&self) -> DatabaseTradeModel {
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
        

        DatabaseTradeModel {
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

pub struct StockAnalyser {
    trade_map: HashMap<String, StockInformation>,
    postgres_client: PostgresClient,
}

impl StockAnalyser {
    pub fn new(postgres_client: PostgresClient) -> Self {
        StockAnalyser{ postgres_client: postgres_client, trade_map: HashMap::new() }
    }

    pub fn add_finnhub_data(&mut self, json_data: &String) {
        self.add_data(&parse_finnhub_data(json_data));
    }

    pub fn add_eodhd_data(&mut self, json_data: &String) {
        self.add_data(&parse_eodhd_data(json_data));
    }

    fn add_data(&mut self, data_rows: &Vec<FinnhubDataRow>) {
        for data_row in data_rows {
            if !self.trade_map.contains_key(data_row.get_stockname()) {
                self.trade_map.insert(data_row.get_stockname().clone(), StockInformation::new());
            }

            let stock_info:&mut StockInformation = self.trade_map.get_mut(data_row.get_stockname()).unwrap();

            match (*stock_info).add_trade(&data_row) {
                Some(v) =>
                     match self.postgres_client.add_finnhub_data(data_row.get_stockname(), v) {
                        Ok(()) => (),
                        Err(e) => panic!("Error inserting into database {}", e),
                     },
                None => return,
            };
        }
    }
}