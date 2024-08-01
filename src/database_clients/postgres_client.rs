use std::{ error };
use postgres::{Client, NoTls};

use crate::data_parsers::finnhub_data_row::FinnhubDataRow;
use crate::data_parsers::finnhub_parser::parse_finnhub_data;

pub struct PostgresClient {
    client: Client,
}

impl PostgresClient {
    pub fn new() -> Self {
        let client = match Client::connect("host=localhost user=postgres password=postgres", NoTls) {
            Ok(client) => client,
            Err(e) => panic!("Error creating PostgreClient {}", e),
        };

        let mut postgres_client:PostgresClient = PostgresClient{ client: client };

        match postgres_client.initialize_database() {
            Ok(_) => (),
            Err(e) => panic!("Error initialzing database {}", e),
        };

        postgres_client
    }

    pub fn add_finnhub_data(&mut self, json_data: &String) -> Result<(), Box<dyn error::Error + 'static>>{
        let finnhub_data_rows:Vec<FinnhubDataRow> = parse_finnhub_data(json_data);

        for data_row in finnhub_data_rows {
            self.client.execute(
                format!("INSERT INTO Finnhub_{} (price, conditions, time, volume) VALUES ($1, $2, $3, $4)", data_row.get_stockname()).as_str(),
                &[data_row.get_price(), data_row.get_conditions(), data_row.get_time(), data_row.get_volume()],
            )?;
        }

        Ok(())
    }

    fn initialize_database(&mut self) -> Result<(), Box<dyn error::Error + 'static>> {
        self.client.batch_execute(format!("
            CREATE TABLE IF NOT EXISTS Finnhub_{} (
                id      SERIAL PRIMARY KEY,
                price    BIGINT,
                conditions    BIGINT,
                time BIGINT,
                volume BIGINT
            )
        ", "AAPL").as_str())?;

        Ok(())
    }
}