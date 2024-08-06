use crate::data_parsers::finnhub_data_row::FinnhubDataRow;

/*
    Find the first occurence of data in string and parse the data from the given index
*/
pub fn parse_eodhd_data(json_data: &String) -> Vec<FinnhubDataRow> {
    let n: usize = json_data.len();
    let json_bytes: &[u8] = json_data.as_bytes();

    let mut open_brackets: i32 = 0;

    let mut tmp:String = String::new();
    let mut key:String = String::new();
    let mut open_string: bool = false;

    let mut current_datapoint:FinnhubDataRow = FinnhubDataRow::new();
    let mut list_of_datapoints:Vec<FinnhubDataRow> = Vec::new();
    
    for i in 0..n {
        if json_bytes[i] == b'\n' || json_bytes[i] == b'\r' || json_bytes[i] == b' ' { continue; }

        if json_bytes[i] == b'"' {
            match open_string {
                true => open_brackets -= 1,
                false => open_brackets += 1,
            };
            
            open_string = !open_string;
            
            continue;
        }
        
        if json_bytes[i] == b'{' || json_bytes[i] == b'[' {
            open_brackets += 1;
            continue;
        }

        if json_bytes[i] == b'}' || json_bytes[i] == b']' {
            open_brackets -= 1;
            continue;
        }

        if json_bytes[i] == b':' && open_brackets == 1 {
            key = tmp;
            tmp = String::new();
            
            continue;
        }

        if json_bytes[i] == b',' && open_brackets <= 1 {
            current_datapoint.set_data(&key, &tmp);
            tmp.clear();

            match open_brackets {
                0 =>{
                    list_of_datapoints.push(current_datapoint);
                    current_datapoint = FinnhubDataRow::new();
                },
                _ => (),
            }


            continue;
        }

        tmp.push(char::from(json_bytes[i]));
    }

    current_datapoint.set_data(&key, &tmp);
    list_of_datapoints.push(current_datapoint);

    list_of_datapoints
}