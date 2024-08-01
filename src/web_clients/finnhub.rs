use websocket::{ ClientBuilder, Message, OwnedMessage, sync::Client, stream::sync::NetworkStream};
use std::{thread, time};

use crate::values_store::credentials_store::CredentialsStore;
use crate::database_clients::postgres_client::PostgresClient;

pub struct FinnhubClient{
    addr: String,
    postgres_client: PostgresClient,
}

impl FinnhubClient {
    pub fn new(credentials_store: CredentialsStore, postgres_client: PostgresClient) -> Self {
        FinnhubClient{ 
            addr: format!("wss://ws.finnhub.io?token={}", credentials_store.get_token("Finnhub.io".to_string())),
            postgres_client: postgres_client,
        }
    }

    pub fn print_hello(&mut self) {
        let mut retry_count: i32 = 0;

        while retry_count <= 2 {
            let client = ClientBuilder::new(&self.addr).unwrap().connect(None);

            if client.is_ok() {
                self.start_websocket(&mut client.unwrap());
                retry_count = 0;
            }

            thread::sleep(time::Duration::from_millis(1000));

            retry_count += 1;
            println!("Retry {}", retry_count);
        }
    }

    fn start_websocket(&mut self, client: &mut Client<Box<(dyn NetworkStream + std::marker::Send + 'static)>>) {
        let message = Message::text("{\"type\":\"subscribe\",\"symbol\":\"AAPL\"}");

        client.send_message(&message).unwrap();

        loop {
            let message:OwnedMessage = match client.recv_message() {
                Ok(p) => p,
                Err(e) => {
                    println!("Error receiving message {} \n Closing Client", e);
                    let _ = client.send_message(&Message::close());
                    break;
                },
            };

            match message {
                OwnedMessage::Text(txt) => {
                    let text: String = txt.parse().unwrap();
                    let _ = self.postgres_client.add_finnhub_data(&text);
                    println!("{}", text);
                }
                OwnedMessage::Close(_) => {
                    let _ = client.send_message(&Message::close());
                    break;
                }
                OwnedMessage::Ping(data) => {
                    client.send_message(&OwnedMessage::Pong(data)).unwrap();
                }
                _ => (),
            }
        }
    }
}
