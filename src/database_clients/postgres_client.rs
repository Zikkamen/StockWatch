use std::{ error };
use postgres::{Client, NoTls};

pub struct DatabaseTradeModel {
    pub first_trade:i64,
    pub num_of_trades: i32,
    pub volume_moved: i32,
    pub avg_price:i64,
    pub min_price:i64,
    pub max_price:i64,
}

pub struct PostgresClient {
    client: Client,
}

impl PostgresClient {
    pub fn new(list_of_stocks:&Vec<String>) -> Self {
        let client = match Client::connect("host=postgresql-main user=postgres password=postgres", NoTls) {
            Ok(client) => client,
            Err(e) => panic!("Error creating PostgreClient {}", e),
        };

        let mut postgres_client:PostgresClient = PostgresClient{ client: client };

        match postgres_client.initialize_database(list_of_stocks) {
            Ok(_) => (),
            Err(e) => panic!("Error initialzing database {}", e),
        };

        println!("Connected to PostgreSQL");

        postgres_client
    }

    pub fn add_finnhub_data(&mut self, stock_name:&String, database_model:DatabaseTradeModel) -> Result<(), Box<dyn error::Error + 'static>>{
        self.client.execute(
            format!("INSERT INTO Trades_{} (time, num_of_trades, volume_moved, avg_price, min_price, max_price) VALUES ($1, $2, $3, $4, $5, $6)", stock_name).as_str(),
            &[&database_model.first_trade,
              &database_model.num_of_trades,
              &database_model.volume_moved,
              &database_model.avg_price,
              &database_model.min_price,
              &database_model.max_price,
            ],
        )?;

        Ok(())
    }

    fn initialize_database(&mut self, list_of_stocks: &Vec<String>) -> Result<(), Box<dyn error::Error + 'static>> {
        for stock in list_of_stocks.iter() {
            self.client.batch_execute(format!("
                CREATE TABLE IF NOT EXISTS Trades_{} (
                    time    BIGINT PRIMARY KEY,
                    num_of_trades INT,
                    volume_moved INT,
                    avg_price BIGINT,
                    min_price BIGINT,
                    max_price BIGINT
                )
            ", stock).as_str())?;
        }

        Ok(())
    }
}