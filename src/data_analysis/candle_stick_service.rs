use crate::database_clients::data_web_client::DataTradeModel;
use crate::data_analysis::finnhub_data_row::FinnhubDataRow;

pub struct CandleStickGraph {
    open: f64,

    pub total_volume: i64,
    pub total_trades: i64,
    pub total_price: i64,
    pub min_price: i64,
    pub max_price: i64,
    pub timestamp: i64,

    stock_name: String,
    current_interval: usize,
    interval_seconds: usize,
}

impl CandleStickGraph {
    pub fn new(interval_seconds: usize, stock_name: String) -> Self {
        CandleStickGraph {
            open: 0.0,
            total_trades: 0,
            total_volume: 0,
            total_price: 0,
            min_price: std::i64::MAX,
            max_price: std::i64::MIN,
            timestamp: 0,
            
            stock_name: stock_name,
            current_interval: 0,
            interval_seconds: interval_seconds,
        }
    }

    pub fn add_trade_candle(&mut self, trade: &CandleStickGraph) -> Option<DataTradeModel> {
        self.total_volume += trade.total_volume;
        self.total_trades += trade.total_trades;
        self.total_price += trade.total_price;
        self.min_price = self.min_price.min(trade.min_price);
        self.max_price = self.max_price.max(trade.max_price);
        self.timestamp = self.timestamp.max(trade.timestamp);

        self.current_interval += 1;

        match self.current_interval >= self.interval_seconds {
            true => Some(self.get_data_trade()),
            false => None,
        }
    }

    pub fn add_trade_main(&mut self, trade: &FinnhubDataRow) {
        self.total_volume += trade.v;
        self.total_trades += 1;
        self.total_price += trade.p * trade.v;
        self.min_price = self.min_price.min(trade.p);
        self.max_price = self.max_price.max(trade.p);
        self.timestamp = self.timestamp.max(trade.t);
    }

    fn get_data_trade(&mut self) -> DataTradeModel {
        let data_trade_model = match self.total_volume {
            0 => {
                self.timestamp += self.interval_seconds as i64 * 1000;
                DataTradeModel {
                    timestamp: self.timestamp,
                    stock_name: self.stock_name.clone(),
                    stock_interval: self.interval_seconds,
                    avg_price: self.open,
                    avg_price_open: self.open,
                    min_price: self.open,
                    max_price: self.open,
                    volume_moved: 0,
                    num_of_trades: 0,
                }
            },
            _ => DataTradeModel {
                timestamp: self.timestamp,
                stock_name: self.stock_name.clone(),
                stock_interval: self.interval_seconds,
                avg_price: self.total_price as f64 / self.total_volume as f64,
                avg_price_open: match self.open {
                    0.0 => self.total_price as f64 / self.total_volume as f64,
                    _ => self.open,
                },
                min_price: self.min_price as f64,
                max_price: self.max_price as f64,
                volume_moved: self.total_volume,
                num_of_trades: self.total_trades,
            }
        };

        self.reset();

        data_trade_model
    }

    fn reset(&mut self) {
        self.open = match self.total_volume {
            0 => self.open,
            _ => self.total_price as f64 / self.total_volume as f64,
        };
        self.total_price = 0;
        self.total_volume = 0;
        self.total_trades = 0;
        self.min_price = std::i64::MAX;
        self.max_price = std::i64::MIN;
        self.current_interval = 0;
    }
}


pub struct CandleStickService {
    cs_graph_main: CandleStickGraph,
    cs_graphs: Vec<CandleStickGraph>,
}

impl CandleStickService {
    pub fn new(stock_name: String) -> Self {
        CandleStickService {
            cs_graph_main: CandleStickGraph::new(1, stock_name.clone()),
            cs_graphs: vec![
                CandleStickGraph::new(10, stock_name.clone()),
                CandleStickGraph::new(60, stock_name.clone()),
                CandleStickGraph::new(300, stock_name.clone()),
                CandleStickGraph::new(600, stock_name.clone())
            ],
        }
    }

    pub fn add_trade(&mut self, trade: &FinnhubDataRow) {
        self.cs_graph_main.add_trade_main(trade);
    }

    pub fn get_trades(&mut self) -> Vec<DataTradeModel> {
        let mut list_of_trades:Vec<DataTradeModel> = Vec::new();

        for cs_graph in self.cs_graphs.iter_mut() {
            match cs_graph.add_trade_candle(&self.cs_graph_main) {
                Some(v) => list_of_trades.push(v),
                None => (),
            };
        }

        list_of_trades.push(self.cs_graph_main.get_data_trade());

        list_of_trades
    }
}