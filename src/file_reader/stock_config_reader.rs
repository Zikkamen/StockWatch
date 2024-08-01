use std:: {
    fs, error,
};

pub struct StockConfigReader {
    file: String,
}

impl StockConfigReader {
    pub fn new() -> Self {
        StockConfigReader{ file: "Stocklist.txt".to_string() }
    }

    pub fn read_config(&self) -> Vec<String> {
        let data: Vec<char> = match self.read_config_file() {
            Ok(v) => v,
            Err(e) => panic!("Problems reading config file {}", e),
        };

        let mut config_companies:Vec<String> = Vec::new();
        let mut tmp:String = String::new();

        for c in data.into_iter() {
            if c == ' ' || c == '\r' || c == '\t' || c == ' ' { continue; }

            if c == '\n' {
                if tmp.len() > 0 {
                    config_companies.push(tmp);
                }

                tmp = String::new();
                continue;
            }

            tmp.push(c);
        }

        if tmp.len() > 0 {
            config_companies.push(tmp);
        }

        config_companies
    }

    fn read_config_file(&self) -> Result<Vec<char>, Box<dyn error::Error + 'static>> {
        let raw_data: Vec<u8> = match fs::read(&self.file){
            Ok(v) => v,
            Err(e) => panic!("Cannot find file: {} {}", self.file, e),
        };

        Ok(raw_data.into_iter().map(|byte| byte as char).collect())
    }
}