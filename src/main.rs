mod values_store;
mod web_clients;
mod file_reader;
mod database_clients;

use crate::values_store::credentials_store::CredentialsStore;
use crate::web_clients::finnhub::FinnhubClient;
use crate::database_clients::postgres_client::PostgresClient;

fn main() {
    let credentials_store:CredentialsStore = CredentialsStore::new();

    let finnhub_client:FinnhubClient = FinnhubClient::new(credentials_store);
    finnhub_client.print_hello();

    //let mut postgres_client:PostgresClient = PostgresClient::new();
    //postgres_client.print_hello();
}
