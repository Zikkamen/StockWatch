mod values_store;
mod web_clients;
mod file_reader;
mod database_clients;
mod data_parsers;
mod data_analysis;

use crate::values_store::credentials_store::CredentialsStore;
use crate::database_clients::data_web_client::DataWebClient;
use crate::database_clients::trade_web_server::TradeWebServer;
use crate::data_analysis::stock_analysis::StockAnalyserWeb;
use crate::web_clients::eodhd::EodhdClient;
use crate::web_clients::finnhub::FinnhubClient;
use crate::web_clients::alpaca::AlpacaClient;
use crate::web_clients::twelve::TwelveClient;
use crate::web_clients::tiingo::TiingoClient;


fn main() {
    let credentials_store:CredentialsStore = CredentialsStore::new();
    let mut data_web_client:DataWebClient = DataWebClient::new("ws://localhost:9003");
    
    let data_web_client:DataWebClient = DataWebClient::new("ws://localhost:9003");
    let stock_config_list:Vec<String> = data_web_client.start_client();

    let trade_web_server:TradeWebServer = TradeWebServer::new("localhost:9010");
    trade_web_server.start_server();

    let stock_analysis_web:StockAnalyserWeb = StockAnalyserWeb::new(data_web_client, trade_web_server);

    let client_selection:usize = 3;

    match client_selection {
        0 => {
            let mut finnhub_client:FinnhubClient = FinnhubClient::new(credentials_store, stock_analysis_web);
            finnhub_client.print_hello(&stock_config_list);
        },
        1 => {
            let mut eodhd_client:EodhdClient = EodhdClient::new(credentials_store, stock_analysis_web);
            eodhd_client.print_hello(&stock_config_list);
        },
        2 => {
            let mut alpaca_client:AlpacaClient = AlpacaClient::new(credentials_store, stock_analysis_web);
            alpaca_client.print_hello(&stock_config_list);
        },
        3 => {
            let mut twelve_client:TwelveClient = TwelveClient::new(credentials_store, stock_analysis_web);
            twelve_client.print_hello(&stock_config_list);
        }
        4 => {
            let mut tiingo_client:TiingoClient = TiingoClient::new(credentials_store, stock_analysis_web);
            tiingo_client.print_hello(&stock_config_list);
        }
        _ => (),
    };
}
