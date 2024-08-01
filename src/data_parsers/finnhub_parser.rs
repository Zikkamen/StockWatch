use std::collections::HashMap;

use crate::data_parsers::finnhub_data_row::FinnhubDataRow;


/*
    Find the first occurence of data in string and parse the data from the given index
*/
pub fn parse_finnhub_data(json_data: &String) -> Vec<FinnhubDataRow> {
    let n: usize = json_data.len();
    let json_bytes: &[u8] = json_data.as_bytes();

    let mut open_brackets: i32 = 0;
    let mut is_left: bool = true;

    let mut tmp:String = String::new();
    let mut open_string: bool = false;
    
    for i in 0..n {
        if json_bytes[i] == b'\n' || json_bytes[i] == b'\r' || json_bytes[i] == b' ' { continue; }
        
        if json_bytes[i] == b'{' || json_bytes[i] == b'[' {
            open_brackets += 1;
            continue;
        }

        if json_bytes[i] == b'}' || json_bytes[i] == b']' {
            open_brackets -= 1;
            continue;
        }

        if open_brackets == 1 && json_bytes[i] == b',' {
            is_left = true;
            continue;
        }

        if !is_left { continue; }

        if json_bytes[i] == b'"'{
            match open_string {
                false => open_string = true,
                true => {
                    if tmp == "data" { return parse_data_field(json_bytes, i+2); }
                    
                    open_string = false;
                    is_left = false;
                    tmp.clear();
                }
            };

            continue;
        }

        if open_string {
            tmp.push(char::from(json_bytes[i]));
        }
    }

    Vec::new()
}

fn parse_data_field(json_bytes: &[u8], pos: usize) -> Vec<FinnhubDataRow> {
    let n: usize = json_bytes.len();

    let mut open_brackets: i32 = 0;

    let mut tmp:String = String::new();
    let mut key:String = String::new();
    let mut open_string: bool = false;

    let mut current_datapoint:FinnhubDataRow = FinnhubDataRow::new();
    let mut list_of_datapoints:Vec<FinnhubDataRow> = Vec::new();
    
    for i in pos..n {
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

        if json_bytes[i] == b':' && open_brackets == 2 {
            key = tmp;
            tmp = String::new();
            
            continue;
        }

        if json_bytes[i] == b',' && open_brackets <= 2 {
            current_datapoint.set_data(&key, &tmp);
            tmp.clear();

            match open_brackets {
                0 => break,
                1 =>{
                    list_of_datapoints.push(current_datapoint);
                    current_datapoint = FinnhubDataRow::new();
                },
                _ => (),
            }


            continue;
        }

        tmp.push(char::from(json_bytes[i]));
    }

    list_of_datapoints.push(current_datapoint);

    list_of_datapoints
}