use std::env;
use std::io;
use std::fs::File;
use std::io::Read;
use std::rc::Rc;

// Third party libraries
use chrono::DateTime;

// Custom modules
mod constants;
mod model;

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
fn parse_input(input: &str) -> model::InputType {
    let tokens: Vec<&str> = input.split(" ").collect();
    let num_tokens: u32 = tokens.len() as u32;

    // parse price update
    if num_tokens == constants::NUM_TOKEN_PRICE_UPDATE {
        let datetime: u64 = match DateTime::parse_from_str(tokens[0], constants::DATETIME_FORMAT) {
            Ok(d) => d.timestamp_millis() as u64,
            Err(_) => return model::InputType::Invalid("Invalid date".to_string())
        };
        let exchange = tokens[1].to_string();
        let source_currency = tokens[2].to_string();
        let dest_currency = tokens[3].to_string();
        let forward_ratio: f64 = match tokens[4].parse() {
            Ok(num) => num,
            Err(_) => return model::InputType::Invalid("Invalid forward ratio".to_string())
        };
        let backward_ratio: f64 = match tokens[5].parse() {
            Ok(num) => num,
            Err(_) => return model::InputType::Invalid("Invalid backward ratio".to_string())
        };

        let both_ratio = forward_ratio * backward_ratio;
        if both_ratio <= 0.0 || both_ratio > 1.0 {
            return model::InputType::Invalid("Resultant ratios is invalid".to_string())
        }
        model::InputType::PriceUpdate(model::PriceUpdate::new (
            datetime, exchange, source_currency, dest_currency, forward_ratio, backward_ratio
        ))
    } else if num_tokens == constants::NUM_TOKEN_EXCHANGE_RATE_REQUEST {
        // parse exchange rate request
        let source_exchange = tokens[1].to_string();
        let source_currency = tokens[2].to_string();
        let dest_exchange = tokens[3].to_string();
        let dest_currency = tokens[4].to_string();

        model::InputType::ExchangeRateRequest(model::ExchangeRateRequest::new(
            source_exchange, source_currency, dest_exchange, dest_currency
        ))
    } else {
        model::InputType::Invalid("Input is neither a price update nor exchange rate request".to_string())
    }
}

// 1. Add edges between vertices
// 2. Add vertices
// 3. Add edges for same currency across different exchanges
fn handle_price_update(
    graph: &mut model::Graph, graph_result: &mut model::GraphResult, price_update: model::PriceUpdate
) {
    let from_vertex = model::Vertex::new(
        price_update.get_exchange().to_string(),
        price_update.get_source_currency().to_string()
    );
    let to_vertex = model::Vertex::new(
        price_update.get_exchange().to_string(),
        price_update.get_dest_currency().to_string()
    );

    let arc_from_vertex = Rc::new(from_vertex);
    let arc_to_vertex = Rc::new(to_vertex);

    // Add edges
    graph_result.add_edge_weight(arc_from_vertex.clone(), arc_to_vertex.clone(),
        price_update.get_forward_ratio(), price_update.get_datetime()
    );
    graph_result.add_edge_weight(arc_to_vertex.clone(), arc_from_vertex.clone(),
        price_update.get_backward_ratio(), price_update.get_datetime()
    );

    let arc_from_vertex_clone = arc_from_vertex.clone();
    let arc_to_vertex_clone = arc_to_vertex.clone();

    // Add vertices
    graph.add_vertex(arc_from_vertex);
    graph.add_vertex(arc_to_vertex);

    let vertices = graph.get_vertices();

    // Add edges for same currency across different exchanges
    graph_result.add_edge_weight_for_currency(arc_from_vertex_clone, vertices);
    graph_result.add_edge_weight_for_currency(arc_to_vertex_clone, vertices)
}

// Get best rate between every pair of vertices
// Get the best rate path
fn handle_exchange_rate_request(graph: & model::Graph,
    graph_result: &mut model::GraphResult, exchange_rate_request: model::ExchangeRateRequest
) {
    graph_result.find_best_rates(graph.get_vertices());

    let arc_from_vertex = Rc::new(model::Vertex::new(
        exchange_rate_request.get_source_exchange().to_string(),
        exchange_rate_request.get_source_currency().to_string()
    ));
    let arc_to_vertex = Rc::new(model::Vertex::new(
        exchange_rate_request.get_dest_exchange().to_string(),
        exchange_rate_request.get_dest_currency().to_string()
    ));
    
    // Print result
    println!("BEST_RATES_BEGIN {} {} {} {} {}", exchange_rate_request.get_source_exchange(),
        exchange_rate_request.get_source_currency(), exchange_rate_request.get_dest_exchange(),
        exchange_rate_request.get_dest_currency(), graph_result.get_best_rate(&arc_from_vertex, &arc_to_vertex)
    );

    match graph_result.best_rate_path(&arc_from_vertex, &arc_to_vertex) {
        Some(best_rate_path) => {
            for vertex in best_rate_path {
                println!("<{}, {}>", vertex.get_exchange(), vertex.get_currency());
            }
        },
        None => ()
    }
    println!("BEST_RATES_END");
}


fn main() {
    let args: Vec<String> = env::args().collect();

    let mut graph_result = model::GraphResult::new();
    let mut graph = model::Graph::new();

    if args.len() != 2 {
        panic!("Usage: cargo run <input_file>, e.g. cargo run input.txt");
    }

    let file_content = match read_file(&args[1]) {
        Ok(content) => content,
        Err(e) => {
            panic!("Error encountered while reading file: {}\nExiting...", e);
        }
    };

    let splitted_lines = file_content.split("\n");
    for line in splitted_lines {
        match parse_input(line) {
            model::InputType::PriceUpdate(price_update) => handle_price_update(
                &mut graph, &mut graph_result, price_update
            ),
            model::InputType::ExchangeRateRequest(exchange_rate_request) => handle_exchange_rate_request(
                &graph, &mut graph_result, exchange_rate_request),
            model::InputType::Invalid(_) => continue
        };
    }
}
