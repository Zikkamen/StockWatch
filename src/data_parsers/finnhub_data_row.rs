pub struct FinnhubDataRow {
    c: i64, //Trade Conditions
    p: i64, //Price in cents
    s: String, //Stockprice name
    t: i64, //trade time in unix milliseconds
    v: i64, //volume
}

impl FinnhubDataRow {
    pub fn new() -> Self {
        FinnhubDataRow{ c: -1, p: -1, s: String::new(), t: 0, v: -1 }
    }

    pub fn set_data(&mut self, key: &String, val: &String) {
        match key.as_str() {
            "p" => self.set_price(val),
            "c" => self.set_conditions(val),
            "s" => self.set_stockname(val),
            "t" => self.set_time(val),
            "v" => self.set_volume(val),
            _ => (),
        }
    }

    pub fn get_stockname(&self) -> &String {
        return &self.s;
    }

    pub fn get_price(&self) -> &i64 {
        return &self.p;
    }

    pub fn get_time(&self) -> &i64 {
        return &self.t;
    }

    pub fn get_conditions(&self) -> &i64 {
        return &self.c;
    }

    pub fn get_volume(&self) -> &i64 {
        return &self.v;
    }

    fn set_price(&mut self, raw_value: &String) {
        match raw_value.parse::<f64>() {
            Ok(v) => self.p = (v * 100.0) as i64,
            Err(e) => println!("Error parsing {} with message: {}", raw_value, e),
        };
    }

    fn set_conditions(&mut self, raw_value: &String) {
        let mut conditions: i64 = 0;

        for condition in raw_value.split(',').into_iter() {
            if condition.len() == 0{
                break;
            }

            let num = match raw_value.parse::<i32>() {
                Ok(v) => v,
                Err(_) => 64,
            };

            if num > 63 { continue; }

            conditions += 1 << num;
        }

        self.c = conditions;
    }

    fn set_stockname(&mut self, raw_value: &String) {
        self.s = raw_value.clone();
    }

    fn set_time(&mut self, raw_value: &String) {
        self.t = raw_value.parse::<i64>().unwrap();
    }

    fn set_volume(&mut self, raw_value: &String) {
        match raw_value.parse::<f64>() {
            Ok(v) => self.v = v as i64,
            Err(e) => println!("Error parsing {} with message: {}", raw_value, e),
        };
    }
}