use std::{
    fs,
    collections::{HashMap},
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

mod web_clients;

use crate::web_clients::finnhub;

fn main() {
    finnhub::print_hello();

    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: HashMap<String, String> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .map(|line| split_string_into_pairs(&line))
        .collect::<HashMap<String, String>>();

    let status_line = "HTTP/1.1 200 OK";
    let contents = fs::read_to_string("hello.html").unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();

    for (key, value) in http_request {
        println!("{key} {value}");
    }
}

fn split_string_into_pairs(s: &String) -> (String, String) {
    let n: usize = s.len();
    let sep_pos: Option<usize> = s.find(':');
    
    if sep_pos.is_none() {
        return ("HEAD_REQUEST:".to_string(), s.clone());
    }

    let sep_pos = sep_pos.unwrap();
    let char_array = s.chars();
    
    return (char_array.clone().take(sep_pos).collect(), char_array.clone().skip(sep_pos+1).take(n-sep_pos).collect());
}
