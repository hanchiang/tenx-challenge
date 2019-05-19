use std::env;
use std::io;
use std::process;
use std::fs::File;
use std::io::Read;
use std::error;

use chrono::DateTime;

const EXCHANGE_RATE_REQUEST: &str = "EXCHANGE_RATE_REQUEST";
const PRICE_UPDATE: &str = "PRICE_UPDATE";
const NUM_TOKEN_PRICE_UPDATE: u32 = 6;
const NUM_TOKEN_EXCHANGE_RATE_REQUEST: u32 = 5;
const DATETIME_FORMAT: &str = "%+";

#[derive(Debug)]
enum InputType {
    ExchangeRateRequest(ExchangeRateRequest),
    PriceUpdate(PriceUpdate),
    Invalid(String)
}

#[derive(Debug)]
struct PriceUpdate {
    datetime: u64,
    exchange: String,
    source_currency: String,
    dest_currency: String,
    forward_ratio: f64,
    backward_ratio: f64
}

#[derive(Debug)]
struct ExchangeRateRequest {
    source_exchange: String,
    source_currency: String,
    dest_exchange: String,
    dest_currency: String
}


fn read_file(file_name: &str) ->  Result<String, io::Error> {
    let mut file = match File::open(file_name) {
        Ok(file) => file,
        Err(e) => return Err(e)
    };

    let mut buffer = String::new();
    match file.read_to_string(&mut buffer) {
        Ok(_) => (),
        Err(e) => return Err(e)
    };
    Ok(buffer)
}

/// Determine whether an input line is a price update or exchange rate request or invalid
/// If it is a valid input, parse into the respective input type and return it, else invalid type
fn parse_input(input: &str) -> InputType {
    let tokens: Vec<&str> = input.split(" ").collect();
    let num_tokens: u32 = tokens.len() as u32;

    if num_tokens == NUM_TOKEN_PRICE_UPDATE {
        let datetime: u64 = match DateTime::parse_from_str(tokens[0], DATETIME_FORMAT) {
            Ok(d) => d.timestamp_millis() as u64,
            Err(e) => return InputType::Invalid("Invalid date".to_string())
        };
        let exchange = tokens[1].to_string();
        let source_currency = tokens[2].to_string();
        let dest_currency = tokens[3].to_string();
        let forward_ratio: f64 = match tokens[4].parse() {
            Ok(num) => num,
            Err(_) => return InputType::Invalid("Invalid forward ratio".to_string())
        };
        let backward_ratio: f64 = match tokens[5].parse() {
            Ok(num) => num,
            Err(_) => return InputType::Invalid("Invalid backward ratio".to_string())
        };

        InputType::PriceUpdate(PriceUpdate {
            datetime,
            exchange,
            source_currency,
            dest_currency,
            forward_ratio,
            backward_ratio
        })
    } else if num_tokens == NUM_TOKEN_EXCHANGE_RATE_REQUEST {
        let source_exchange = tokens[0].to_string();
        let source_currency = tokens[1].to_string();
        let dest_exchange = tokens[2].to_string();
        let dest_currency = tokens[3].to_string();

        InputType::ExchangeRateRequest(ExchangeRateRequest {
            source_exchange,
            source_currency,
            dest_exchange,
            dest_currency
        })
    } else {
        InputType::Invalid("Input is neither a price update nor exchange rate request".to_string())
    }
}


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: cargo run <input_file>, e.g. cargo run input.txt");
        process::exit(1);
    }

    let file_content = match read_file(&args[1]) {
        Ok(content) => content,
        Err(e) => {
            println!("Error encountered while reading file: {}\nExiting...", e);
            process::exit(1);
        }
    };

    let splitted_lines = file_content.split("\n");
    for line in splitted_lines {
        let tokens = match parse_input(line) {
            InputType::PriceUpdate(p) => println!("{:#?}", p),
            InputType::ExchangeRateRequest(e) => println!("{:#?}", e),
            InputType::Invalid(m) => println!("{:#?}", m)
        };
    }
}
