mod values_store;
mod web_clients;
mod file_reader;
mod database_clients;
mod data_parsers;

use crate::values_store::credentials_store::CredentialsStore;
use crate::web_clients::finnhub::FinnhubClient;
use crate::database_clients::postgres_client::PostgresClient;
use crate::file_reader::stock_config_reader::StockConfigReader;

fn main() {
    let credentials_store:CredentialsStore = CredentialsStore::new();
    let stock_config_reader: StockConfigReader =StockConfigReader::new();
    let stock_config_list:Vec<String> = stock_config_reader.read_config();

    let postgres_client:PostgresClient = PostgresClient::new(&stock_config_list);

    let mut finnhub_client:FinnhubClient = FinnhubClient::new(credentials_store, postgres_client);
    finnhub_client.print_hello(&stock_config_list);
}
