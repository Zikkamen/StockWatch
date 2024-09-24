mod values_store;
mod web_clients;
mod file_reader;
mod database_clients;
mod data_parsers;
mod data_analysis;

use crate::values_store::credentials_store::CredentialsStore;
use crate::database_clients::data_web_client::DataWebClient;
use crate::data_analysis::stock_analysis::StockAnalyserWeb;
use crate::web_clients::eodhd::EodhdClient;
use crate::web_clients::finnhub::FinnhubClient;
use crate::web_clients::alpaca::AlpacaClient;
use crate::web_clients::twelve::TwelveClient;


fn main() {
    let credentials_store:CredentialsStore = CredentialsStore::new();

    let mut data_web_client:DataWebClient = DataWebClient::new("ws://localhost:9003");
    let stock_config_list:Vec<String> = data_web_client.get_stocklist();

    let stock_analysis_web:StockAnalyserWeb = StockAnalyserWeb::new(data_web_client);

    let client_selection:usize = 3;

    if client_selection == 0 {
        let mut finnhub_client:FinnhubClient = FinnhubClient::new(credentials_store, stock_analysis_web);
        finnhub_client.print_hello(&stock_config_list);
    }
    else if client_selection == 1 {
        let mut eodhd_client:EodhdClient = EodhdClient::new(credentials_store, stock_analysis_web);
        eodhd_client.print_hello(&stock_config_list);
    }
    else if client_selection == 2 {
        let mut alpaca_client:AlpacaClient = AlpacaClient::new(credentials_store, stock_analysis_web);
        alpaca_client.print_hello(&stock_config_list);
    }
    else if client_selection == 3 {
        let mut twelve_client:TwelveClient = TwelveClient::new(credentials_store, stock_analysis_web);
        twelve_client.print_hello(&stock_config_list);
    }

}
