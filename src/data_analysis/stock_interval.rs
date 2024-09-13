use std::time::{Duration, SystemTime};

type TupStockInfo = (StockInformation, StockInformation, StockInformation);

pub struct StockDataPoint {
    pub timestamp: i64,
    pub price: i64,
    pub volume_moved: i64,
}

pub struct StockInformation {
    trades: VecDeque<StockDataPoint>,
    fenwick_tree: FenwickTree,

    total_price: i64,
    total_trades: i64,
    total_volume: i64,
    last_timestamp: i64,
    
    time_limit_mil: i64,
}

impl StockInformation {
    pub fn new(time_limit_mil: i64) -> Self {
        StockInformation{ 
            trades: VecDeque::new(), 
            fenwick_tree: FenwickTree::new(),
            total_price: 0,
            total_trades: 0,
            total_volume: 0,
            last_timestamp: 0,
            time_limit_mil: time_limit_mil,
        }
    }

    pub fn add_trade(&mut self, data: &FinnhubDataRow) {
        if data.t > self.last_timestamp {
            self.last_timestamp = data.t;
        }

        while self.trades.len() > 0 {
            match self.trades.front() {
                Some(v) => {
                    if self.last_timestamp - v.timestamp > self.time_limit_mil {
                        self.total_volume -= v.volume_moved;
                        self.total_trades -= 1;
                        self.total_price -= v.price * v.volume_moved;

                        self.fenwick_tree.insert(v.price, -v.volume_moved);

                        self.trades.pop_front();
                    } else {
                        break;
                    }
                },
                None => break,
            }
        }

        self.total_price += data.v * data.p;
        self.total_volume += data.v;
        self.total_trades += 1;

        self.fenwick_tree.insert(data.p, data.v);
        self.trades.push_back(StockDataPoint{timestamp: data.t, price: data.p, volume_moved: data.v});
    }
}

fn start_thread(trade_map: Arc<RwLock<HashMap<String, (StockInformation, StockInformation, StockInformation)>>>,
                trade_update: Arc<RwLock<HashSet<String>>>,
                mut data_web_client: DataWebClient) {
    let mut open_price_map:HashMap<(String, usize), VecDeque<f64>> = HashMap::new();
    
    let target_time = SystemTime::now();
    target_time.add(Duration::from_millis(1000));

    loop {
        match target_time.duration_since(&SystemTime::now()) {
            Ok(v) => thread::sleep(v),
            Err(_) => (),
        };

        target_time.add(Duration::from_millis(1000));

        for (key, value) in &mut open_price_map {
            value.push_back(*value.back().unwrap());

            if value.len() > key.1 + 1 {
                value.pop_front().expect("Interval to be > 0");
            }
        }

        let trade_keys:HashSet<String> = trade_update.read().unwrap().clone();
        trade_update.write().unwrap().clear();

        let trade_map_read = trade_map.read().unwrap();

        for key in trade_keys {
            let value = match trade_map_read.get(&key) {
                Some(v) => v,
                None => continue,
            };

            process_stock_info(&key, 60, &value.2, &mut data_web_client, &mut open_price_map);
        }
    }
}

fn process_stock_info(
        name: &str, 
        stock_interval: usize, 
        value: &StockInformation, 
        data_web_client: &mut DataWebClient,
        open_price_map: &mut HashMap<(String, usize), VecDeque<f64>>,
    ) {
    let mut data_trade_model = DataTradeModel::new();

    data_trade_model.timestamp = value.last_timestamp;

    data_trade_model.avg_price = value.total_price as f64 / value.total_volume as f64;
    data_trade_model.min_price = value.fenwick_tree.find(0);
    data_trade_model.max_price = value.fenwick_tree.find(value.total_volume);

    data_trade_model.volume_moved = value.total_volume;
    data_trade_model.num_of_trades = value.total_trades;

    let key = (name.to_string(), stock_interval);

    match open_price_map.get_mut(&key) {
        Some(v) => {
            if v.len() == stock_interval + 1 {
                data_trade_model.avg_price_open = *v.front().unwrap();
            }

            v.pop_back();
            v.push_back(data_trade_model.avg_price);
        },
        None => {
            open_price_map.insert(key, VecDeque::from_iter([data_trade_model.avg_price]));
        }
    };

    match data_web_client.add_finnhub_data(name, stock_interval, data_trade_model) {
        Ok(_v) => (),
        Err(e) => panic!("Error sending data using webclient {}", e),
    };
}