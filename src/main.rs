mod values_store;
mod web_clients;
mod file_reader;
mod database_clients;
mod data_parsers;
mod data_analysis;

use crate::values_store::credentials_store::CredentialsStore;
use crate::web_clients::finnhub::FinnhubClient;
use crate::database_clients::postgres_client::PostgresClient;
use crate::file_reader::stock_config_reader::StockConfigReader;
use crate::data_analysis::stock_analysis::StockAnalyser;

fn main() {
    let credentials_store:CredentialsStore = CredentialsStore::new();
    let stock_config_reader: StockConfigReader =StockConfigReader::new();
    let stock_config_list:Vec<String> = stock_config_reader.read_config();

    let postgres_client:PostgresClient = PostgresClient::new(&stock_config_list);
    let stock_analysis:StockAnalyser = StockAnalyser::new(postgres_client);

    let mut finnhub_client:FinnhubClient = FinnhubClient::new(credentials_store, stock_analysis);
    finnhub_client.print_hello(&stock_config_list);
}
