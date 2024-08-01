use std::error;
use postgres::{Client, NoTls};

pub struct PostgresClient {
    client: Client,
}

impl PostgresClient {
    pub fn new() -> Self {
        let client = match Client::connect("host=localhost user=postgres password=postgres", NoTls) {
            Ok(client) => client,
            Err(e) => panic!("Error creating PostgreClient {}", e),
        };

        PostgresClient{ client: client }
    }

    pub fn print_hello(&mut self) -> Result<(), Box<dyn error::Error + 'static>>{ 
        self.client.batch_execute("
            CREATE TABLE IF NOT EXISTS person (
                id      SERIAL PRIMARY KEY,
                name    TEXT NOT NULL,
                data    BYTEA
            )
        ")?;

        let name = "Ferris";
        let data = None::<&[u8]>;
        self.client.execute(
            "INSERT INTO person (name, data) VALUES ($1, $2)",
            &[&name, &data],
        )?;

        for row in self.client.query("SELECT id, name, data FROM person", &[])? {
            let id: i32 = row.get(0);
            let name: &str = row.get(1);
            let data: Option<&[u8]> = row.get(2);

            println!("found person: {} {} {:?}", id, name, data);
        }

        Ok(())
    }
}