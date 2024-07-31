use websocket::{ ClientBuilder, Message, OwnedMessage, };

use crate::values_store::credentials_store::CredentialsStore;

pub struct FinnhubClient{
    addr: String,
}

impl FinnhubClient {
    pub fn new(credentials_store: CredentialsStore) -> Self {
        FinnhubClient{ 
            addr: format!("wss://ws.finnhub.io?={}", credentials_store.get_token("Finnhub.io".to_string())),
        }
    }

    pub fn print_hello(&self) {
        println!("{}", self.addr);
    }
}
