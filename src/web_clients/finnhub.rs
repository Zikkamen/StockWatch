use websocket::{ ClientBuilder, Message, OwnedMessage };

use crate::values_store::credentials_store::CredentialsStore;

pub struct FinnhubClient{
    addr: String,
}

impl FinnhubClient {
    pub fn new(credentials_store: CredentialsStore) -> Self {
        FinnhubClient{ 
            addr: format!("wss://ws.finnhub.io?token={}", credentials_store.get_token("Finnhub.io".to_string())),
        }
    }

    pub fn print_hello(&self) {
        println!("{}", self.addr);

        let mut client = ClientBuilder::new(&self.addr).unwrap()
            .connect(None)
            .unwrap();

        println!("Successfully connected");

        // send messages!
        let message = Message::text("{\"type\":\"subscribe\",\"symbol\":\"AAPL\"}");

        client.send_message(&message).unwrap();

        loop {
            let message:OwnedMessage = client.recv_message().unwrap();

            match message {
                OwnedMessage::Text(txt) => {
                    let text: String = txt.parse().unwrap();
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
