use crate::data_analysis::finnhub_data_row::FinnhubDataRow;

/*
    Find the first occurence of data in string and parse the data from the given index
*/
/*
    Find the first occurence of data in string and parse the data from the given index
*/
pub fn parse_alpaca_data(json_data: &String) -> Vec<FinnhubDataRow> {
    let mut open_brackets: i32 = 0;

    let mut tmp:String = String::new();
    let mut key:String = String::new();
    let mut open_string: bool = false;

    let mut current_datapoint:FinnhubDataRow = FinnhubDataRow::new();
    let mut list_of_datapoints:Vec<FinnhubDataRow> = Vec::new();
    
    for c in json_data.chars() {
        if c == '\n' || c == '\r' || c == ' ' { continue; }

        if c == '"' {
            match open_string {
                true => open_brackets -= 1,
                false => open_brackets += 1,
            };
            
            open_string = !open_string;
            
            continue;
        }
        
        if c == '{' || c == '[' {
            open_brackets += 1;
            continue;
        }

        if c == '}' || c == ']' {
            open_brackets -= 1;
            continue;
        }

        if c == ':' && open_brackets == 2 {
            key = tmp;
            tmp = String::new();
            
            continue;
        }

        if c == ',' && open_brackets <= 2 {
            current_datapoint.set_alpaca_data(&key, &tmp);
            tmp.clear();

            match open_brackets {
                1 => {
                    list_of_datapoints.push(current_datapoint);
                    current_datapoint = FinnhubDataRow::new();
                },
                _ => (),
            }


            continue;
        }

        tmp.push(c);
    }

    current_datapoint.set_alpaca_data(&key, &tmp);
    list_of_datapoints.push(current_datapoint);

    list_of_datapoints
}

#[cfg(test)]
mod tests {
    use crate::data_parsers::alpaca_parser::parse_alpaca_data;

    #[test]
    fn parse_alpaca_data_test() {
        let input = "[{\"T\":\"t\",\"S\":\"TSM\",\"i\":55397666350414,\"x\":\"V\",\"p\":156.97,\"s\":100,\"c\":[\" \"],\"z\":\"A\",\"t\":\"2024-09-06T15:27:56.438925312Z\"}]".to_string();

        let data_row = parse_alpaca_data(&input);

        println!("{:?}", data_row);

        assert!(false);
    }   
}