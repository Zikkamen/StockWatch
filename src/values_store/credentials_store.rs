use std:: { 
    collections::HashMap,
};

use crate::file_reader::credentials_reader::CredentialsReader;

pub struct CredentialsStore {
    credentials_map: HashMap<String, String>,
}

impl CredentialsStore {
    pub fn new() -> Self {
        let mut credentials_store = CredentialsStore{ credentials_map:HashMap::new() };

        let credentials_reader:CredentialsReader = CredentialsReader::new("./credentials/apikeys.xml".to_string());
        credentials_store.insert_values_into_map(&credentials_reader.get_credentials());

        credentials_store
    }

    pub fn get_token(&self, key: &str) -> String {
        match self.credentials_map.get(key) {
            Some(p) => p.to_string(),
            None => panic!("Couldn't find Token for key: {}", key),
        }
    }

    fn insert_values_into_map(&mut self, map_of_values: &HashMap<String, String>) {
        for (key, value) in map_of_values {
            self.credentials_map.insert(key.clone(), value.clone());
        }
    }
}