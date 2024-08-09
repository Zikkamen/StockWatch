mod values_store;
mod web_clients;
mod file_reader;
mod database_clients;
mod data_parsers;
mod data_analysis;

use crate::values_store::credentials_store::CredentialsStore;
use crate::web_clients::finnhub::FinnhubClient;
use crate::database_clients::data_web_client::DataWebClient;
use crate::file_reader::stock_config_reader::StockConfigReader;
use crate::data_analysis::stock_analysis::StockAnalyserWeb;
use crate::web_clients::eodhd::EodhdClient;


fn main() {
    let credentials_store:CredentialsStore = CredentialsStore::new();
    let stock_config_reader: StockConfigReader = StockConfigReader::new();
    let stock_config_list:Vec<String> = stock_config_reader.read_config();

    let data_web_client:DataWebClient = DataWebClient::new("ws://localhost:9003");
    let stock_analysis_web:StockAnalyserWeb = StockAnalyserWeb::new(data_web_client);

    let mut finnhub_client:EodhdClient = EodhdClient::new(credentials_store, stock_analysis_web);
    finnhub_client.print_hello(&stock_config_list);
}
