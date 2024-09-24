use crate::data_analysis::finnhub_data_row::FinnhubDataRow;

/*
    Find the first occurence of data in string and parse the data from the given index
*/
pub fn parse_twelve_data(json_data: &String) -> FinnhubDataRow {
    let n: usize = json_data.len();
    let json_chars = json_data.chars();

    let mut open_brackets: i32 = 0;
    let mut open_string: bool = false;

    let mut tmp:String = String::new();
    let mut key:String = String::new();

    let mut current_datapoint:FinnhubDataRow = FinnhubDataRow::new();
    
    for c in json_chars.into_iter() {
        let mut push_char: bool = false;

        match c {
            '\n' | '\r' | ' ' => (),
            '"' => {
                match open_string {
                    true => open_brackets -= 1,
                    false => open_brackets += 1,
                };
                
                open_string = !open_string;
            },
            '{' | '[' => open_brackets += 1,
            '}' | ']' => open_brackets -= 1,
            ':' => {
                match open_brackets {
                    1 => {
                        key = tmp;
                        tmp = String::new();
                    },
                    _ => push_char = true,
                }
            },
            ',' => {
                if open_brackets <= 1 {
                    current_datapoint.set_twelve_data(&key, &tmp);
                    tmp.clear();
                } else {
                    push_char = true;
                }
            },
            _ => push_char = true,
        }

        if push_char {
            tmp.push(c);
        }
    }

    current_datapoint.set_twelve_data(&key, &tmp);
    current_datapoint
}