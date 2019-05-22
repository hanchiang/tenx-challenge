use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
use std::sync::Arc;

#[derive(Debug)]
pub enum InputType {
    ExchangeRateRequest(ExchangeRateRequest),
    PriceUpdate(PriceUpdate),
    Invalid(String)
}

#[derive(Debug)]
pub struct PriceUpdate {
    datetime: u64,  // millisecond
    exchange: String,
    source_currency: String,
    dest_currency: String,
    forward_ratio: f64,
    backward_ratio: f64
}

impl PriceUpdate {
  pub fn new(
    datetime: u64, exchange: String, source_currency: String, dest_currency: String,
    forward_ratio: f64, backward_ratio: f64
  ) -> PriceUpdate {
    PriceUpdate {
      datetime, exchange, source_currency, dest_currency, forward_ratio, backward_ratio
    }
  }

  pub fn get_datetime(&self) -> u64 {
    self.datetime
  }

  pub fn get_exchange(&self) -> &str {
    &self.exchange[..]
  }

  pub fn get_source_currency(&self) -> &str {
    &self.source_currency[..]
  }

  pub fn get_dest_currency(&self) -> &str {
    &self.dest_currency[..]
  }

  pub fn get_forward_ratio(&self) -> f64 {
    self.forward_ratio
  }

  pub fn get_backward_ratio(&self) -> f64 {
    self.backward_ratio
  }
}

#[derive(Debug)]
pub struct ExchangeRateRequest {
    source_exchange: String,
    source_currency: String,
    dest_exchange: String,
    dest_currency: String
}

impl ExchangeRateRequest {
  pub fn new(source_exchange: String, source_currency: String,
    dest_exchange: String, dest_currency: String
  ) -> ExchangeRateRequest {
    ExchangeRateRequest {
      source_exchange, source_currency, dest_exchange, dest_currency
    }
  }

  pub fn get_source_exchange(&self) -> &str {
    &self.source_exchange[..]
  }

  pub fn get_source_currency(&self) -> &str {
    &self.source_currency[..]
  }

  pub fn get_dest_exchange(&self) -> &str {
    &self.dest_exchange[..]
  }

  pub fn get_dest_currency(&self) -> &str {
    &self.dest_currency[..]
  }
}

#[derive(Debug)]
#[derive(Hash, PartialEq, Eq, Clone)]
pub struct Vertex {
  exchange: String,
  currency: String
}

impl Vertex {
  pub fn new(exchange: String, currency: String) -> Vertex {
    Vertex {
      exchange, currency
    }
  }

  pub fn get_exchange(&self) -> &str {
    &self.exchange
  }

  pub fn get_currency(&self) -> &str {
    &self.currency
  }
}

#[derive(Debug)]
pub struct Graph {
  vertices: HashSet<Arc<Vertex>>,
  currencies: HashSet<String>
}

impl Graph {
  pub fn new() -> Graph {
    Graph {
      vertices: HashSet::new(),
      currencies: HashSet::new()
    }
  }

  pub fn get_vertices(&self) -> &HashSet<Arc<Vertex>> {
    &self.vertices
  }

  pub fn get_currencies(&self) -> &HashSet<String> {
    &self.currencies
  }

  pub fn add_vertex(&mut self, vertex: Arc<Vertex>) {
    match self.vertices.get(&vertex) {
      Some(_) => (),
      None => {
        self.currencies.insert(vertex.get_currency().to_string());
        self.vertices.insert(vertex);
      }
    }
  }
}

#[derive(Debug)]
pub struct EdgeWeight {
  weight: f64,
  last_updated: u64
}

impl EdgeWeight {
  pub fn new(weight: f64, last_updated: u64) -> EdgeWeight {
    EdgeWeight {
      weight, last_updated
    }
  }

  pub fn get_weight(&self) -> f64 {
    self.weight
  }

  pub fn get_last_updated(&self) -> u64 {
    self.last_updated
  }

  pub fn set_weight(&mut self, weight: f64) {
    self.weight = weight;
  }
}

#[derive(Debug)]
pub struct GraphResult {
  // stores the edge weights between each pair of vertex
  rate: HashMap<Arc<Vertex>, HashMap<Arc<Vertex>, EdgeWeight>>,
  // stores vertices to reconstruct the path for best rate from vertex i to j
  next: HashMap<Arc<Vertex>, HashMap<Arc<Vertex>, Arc<Vertex>>>
}

impl GraphResult {
  pub fn new() -> GraphResult {
    GraphResult {
      rate: HashMap::new(),
      next: HashMap::new()
    }
  }

  pub fn add_edge_weight(
    &mut self, from_vertex: Arc<Vertex>, to_vertex: Arc<Vertex>,
    weight: f64, datetime: u64
  ) {
    match self.rate.get_mut(&from_vertex) {
      Some(inner_map) => {
        match inner_map.get(&to_vertex) {
          Some(edge) => {
            if datetime > edge.get_last_updated() {
              inner_map.entry(to_vertex).and_modify(|e| {
                e.set_weight(weight);
              });
            }
          },
          None => {
            let inner: HashMap<Arc<Vertex>, EdgeWeight> = HashMap::new();
            inner_map.insert(to_vertex, EdgeWeight::new(weight, datetime));
          }
        }

      },
      None => {
        let mut inner_map: HashMap<Arc<Vertex>, EdgeWeight> = HashMap::new();
        // // 0 edge weight for linking to same vertex
        // inner_map.insert(from_vertex.clone(), EdgeWeight::new(0.0, datetime));
        inner_map.insert(to_vertex, EdgeWeight::new(weight, datetime));
        self.rate.insert(from_vertex, inner_map);
      }
    }
  }

  // TODO: call this in exchange rate request part
  pub fn add_edge_weight_for_currency(
    &self, vertices: &HashSet<Arc<Vertex>>, currencies: &HashSet<String>
  ) {
    // Get list of unique currencies
    // For each currency, add edge weight of 1 to other all other exchanges with same currency
    for currency in currencies.iter() {
      println!("Get vertices for {}", currency);
      let mut vertices_for_currency: HashSet<Arc<Vertex>> = vertices.iter().cloned().collect();
      vertices_for_currency.retain(|v| v.get_currency() == currency);
      println!("{:#?}", vertices_for_currency);
    }
  }
}