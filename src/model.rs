use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
use std::sync::Arc;

use chrono::Utc;

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

impl Default for EdgeWeight {
  fn default() -> Self {
    EdgeWeight {
      weight: 0.0,
      last_updated: Utc::now().timestamp_millis() as u64
    }
  }
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

  pub fn set_weight(&mut self, weight: f64) {
    self.weight = weight;
  }

  pub fn get_last_updated(&self) -> u64 {
    self.last_updated
  }

  pub fn set_last_updated(&mut self, last_updated: u64) {
    self.last_updated = last_updated;
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
        match inner_map.get_mut(&to_vertex) {
          Some(edge) => {
            if datetime > edge.get_last_updated() {
              edge.set_weight(weight);
              edge.set_last_updated(datetime);
            }
          },
          // No record of edge from `from_vertex` to `to_vertex`
          None => {
            let inner: HashMap<Arc<Vertex>, EdgeWeight> = HashMap::new();
            inner_map.insert(to_vertex, EdgeWeight::new(weight, datetime));
          }
        }
      },
      // No record of `from_vertex` in `rate`
      None => {
        let mut inner_map: HashMap<Arc<Vertex>, EdgeWeight> = HashMap::new();
        inner_map.insert(to_vertex, EdgeWeight::new(weight, datetime));
        self.rate.insert(from_vertex, inner_map);
      }
    }
  }

  // Get a list of vertices with the same currency as the vertex that was just inserted
  // Add edge weight of 1 from vertexInserted to other vertices and vice versa
  pub fn add_edge_weight_for_currency(
    &mut self, vertexInserted: Arc<Vertex>, vertices: &HashSet<Arc<Vertex>>
  ) {
    let currenncy_to_match = vertexInserted.get_currency();
    let mut vertices_for_currency: HashSet<Arc<Vertex>> = vertices.clone();
    // O(V)
    vertices_for_currency.retain(|v| v.get_currency() == currenncy_to_match);

    // O(V2 < V)
    for vertex in vertices_for_currency {
      // Do not set edge to link to the same vertex
      if vertex != vertexInserted {
        // Set edge from vertexInserted to vertex
        match self.rate.get_mut(&vertexInserted) {
          Some(inner_map) => {
            inner_map.insert(vertex.clone(), EdgeWeight::new(1.0, Utc::now().timestamp_millis() as u64));
          },
          None => ()
        }
        // Set edge from vertex to vertexInserted
        match self.rate.get_mut(&vertex) {
          Some(inner_map) => {
            inner_map.insert(vertexInserted.clone(), EdgeWeight::new(1.0, Utc::now().timestamp_millis() as u64));
          },
          None => ()
        }
      }
    }
  }
}