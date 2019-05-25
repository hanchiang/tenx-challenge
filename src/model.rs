use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use chrono::Utc;

pub enum InputType {
    ExchangeRateRequest(ExchangeRateRequest),
    PriceUpdate(PriceUpdate),
    Invalid(String)
}

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

pub struct Graph {
  vertices: HashSet<Arc<Vertex>>
}

impl Graph {
  pub fn new() -> Graph {
    Graph {
      vertices: HashSet::new()
    }
  }

  pub fn get_vertices(&self) -> &HashSet<Arc<Vertex>> {
    &self.vertices
  }

  pub fn add_vertex(&mut self, vertex: Arc<Vertex>) {
    match self.vertices.get(&vertex) {
      Some(_) => (),
      None => {
        self.vertices.insert(vertex);
      }
    }
  }
}

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

pub struct GraphResult {
  // stores the edge weights between each pair of vertex
  adj_matrix: HashMap<Arc<Vertex>, HashMap<Arc<Vertex>, EdgeWeight>>,
  // stores the best rate between each pair of vertex
  best_rate: HashMap<Arc<Vertex>, HashMap<Arc<Vertex>, f64>>,
  // stores vertices to reconstruct the path for best rate from vertex i to j
  next: HashMap<Arc<Vertex>, HashMap<Arc<Vertex>, Arc<Vertex>>>
}

impl GraphResult {
  pub fn new() -> GraphResult {
    GraphResult {
      adj_matrix: HashMap::new(),
      best_rate: HashMap::new(),
      next: HashMap::new()
    }
  }

  // Update next[i][j] to next[i][k]
  pub fn update_next_vertex(&mut self, i: &Arc<Vertex>, j: &Arc<Vertex>, k: &Arc<Vertex>) {
    let ik_next = self.next.get(i).unwrap().get(k).unwrap().clone();

    match self.next.get_mut(i) {
      Some(inner_map) => {
        inner_map.entry(j.clone())
          .and_modify(|vertex| { *vertex = ik_next.clone() })
          .or_insert(ik_next);
      },
      // vertex `i` will always be found in `next`
      None => ()
    }
  }

  // Add `to_vertex` in next[from_vertex][to_vertex]
  fn add_next_vertex(next: &mut HashMap<Arc<Vertex>, HashMap<Arc<Vertex>, Arc<Vertex>>>,
    from_vertex: &Arc<Vertex>, to_vertex: &Arc<Vertex>
  ) {
    match next.get_mut(from_vertex) {
      Some(inner_map) => {
        match inner_map.get_mut(to_vertex) {
          Some(_) => (),
          None => {
            inner_map.insert(to_vertex.clone(), to_vertex.clone());
          }
        }
      },
      // No record of `from_vertex` in `next`
      None => {
        let mut inner_map: HashMap<Arc<Vertex>, Arc<Vertex>> = HashMap::new();
        inner_map.insert(to_vertex.clone(), to_vertex.clone());
        next.insert(from_vertex.clone(), inner_map);
      }
    }
  }

  // Get the edge weight of adj_matrix[from_vertex][to_vertex]
  pub fn get_edge_weight(&self, from_vertex: &Arc<Vertex>, to_vertex: &Arc<Vertex>) -> f64 {
    match self.adj_matrix.get(from_vertex) {
      Some(inner_map) => {
        match inner_map.get(to_vertex) {
          Some(edge) => edge.get_weight(),
          // Return 0 if there is no edge between `from_vertex` and `to_vertex`
          None => 0.0
        }
      },
      // `from_vertex` will always be found in `adj_matrix`
      None => 0.0
    }
  }

  // Set the edge weight of best_rate[from_vertex][to_vertex]
  fn add_best_rate(best_rate: &mut HashMap<Arc<Vertex>, HashMap<Arc<Vertex>, f64>>,
    from_vertex: &Arc<Vertex>, to_vertex: &Arc<Vertex>, weight: f64
  ) {
    match best_rate.get_mut(from_vertex) {
      Some(inner_map) => {
        inner_map.entry(to_vertex.clone())
          .and_modify(|edge_weight| { *edge_weight = weight })
          .or_insert(weight);
      },
      None => {
        let mut inner_map: HashMap<Arc<Vertex>, f64> = HashMap::new();
        inner_map.insert(to_vertex.clone(), weight);
        best_rate.insert(from_vertex.clone(), inner_map);
      }
    }
  }

  pub fn get_best_rate(&self, from_vertex: &Arc<Vertex>, to_vertex: &Arc<Vertex>) -> f64 {
    *self.best_rate.get(from_vertex).unwrap().get(to_vertex).unwrap()
  }


  // Add edge weight in adj_matrix[from_vertex][to_vertex]
  pub fn add_edge_weight(
    &mut self, from_vertex: Arc<Vertex>, to_vertex: Arc<Vertex>,
    weight: f64, datetime: u64
  ) {
    // Add edge from `from_vertex` to `to_vertex`
    match self.adj_matrix.get_mut(&from_vertex) {
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
            inner_map.insert(to_vertex.clone(), EdgeWeight::new(weight, datetime));
          }
        }
      },
      // No record of `from_vertex` in `adj_matrix`
      None => {
        let mut inner_map: HashMap<Arc<Vertex>, EdgeWeight> = HashMap::new();
        inner_map.insert(to_vertex.clone(), EdgeWeight::new(weight, datetime));
        self.adj_matrix.insert(from_vertex.clone(), inner_map);
      }
    }
  }

  // 1. Get a list of vertices with the same currency as the vertex that was just inserted
  // 2. Add edge weight of 1 from vertex_inserted to other vertices[v1..vn] and vice versa
  // Runtime: O(V + V2), V2 < V
  pub fn add_edge_weight_for_currency(
    &mut self, vertex_inserted: Arc<Vertex>, vertices: &HashSet<Arc<Vertex>>
  ) {
    let currenncy_to_match = vertex_inserted.get_currency();
    let mut vertices_for_currency: HashSet<Arc<Vertex>> = vertices.clone();
    // O(V)
    vertices_for_currency.retain(|v| { v.get_currency() == currenncy_to_match });

    // O(V2 < V)
    for vertex in vertices_for_currency {
      // Do not set edge to link to the same vertex
      if vertex != vertex_inserted {
        // Set edge from vertex_inserted to vertex
        match self.adj_matrix.get_mut(&vertex_inserted) {
          Some(inner_map) => {
            inner_map.entry(vertex.clone())
              .or_insert(EdgeWeight::new(1.0, Utc::now().timestamp_millis() as u64));
          },
          // `vertex_inserted` will always be found in `adj_matrix` because it was just inserted
          None => ()
        }
        // Set edge from vertex to vertex_inserted
        match self.adj_matrix.get_mut(&vertex) {
          Some(inner_map) => {
            inner_map.entry(vertex_inserted.clone())
              .or_insert(EdgeWeight::new(1.0, Utc::now().timestamp_millis() as u64));
          },
          // `vertex` will always be found in `adj_matrix` because edges and vertices were added before this step
          None => ()
        }
      }
    }
  }

  // Modified floyd warshall to get the best rate for every pair of vertices
  pub fn find_best_rates(&mut self, vertices: &HashSet<Arc<Vertex>>) {
    // For all edges, add edge in rate[i][j], add j in next[i][j]
    for (i, inner_map) in self.adj_matrix.iter_mut() {
      for (j, edge) in inner_map.iter() {
        let edge_weight = edge.get_weight();

        GraphResult::add_best_rate(&mut self.best_rate, i, j, edge_weight);
        GraphResult::add_next_vertex(&mut self.next, i, j);
      }
    }

    for i in vertices.iter().cloned() {
        for j in vertices.iter().cloned() {
            for k in vertices.iter().cloned() {
                // Skip weight comparison if any pair of ij, ik,, kj are the same vertices
                if i != j && i != k && k != j {
                    let ij_weight = self.get_edge_weight(&i, &j);
                    let ik_weight = self.get_edge_weight(&i, &k);
                    let kj_weight = self.get_edge_weight(&k, &j);

                    if ij_weight < ik_weight * kj_weight {
                        GraphResult::add_best_rate(&mut self.best_rate, &i, &j, ik_weight * kj_weight);
                        self.update_next_vertex(&i, &j, &k);
                    }
                }
            }
        }
    }
  }

  pub fn best_rate_path(&self, from_vertex: &Arc<Vertex>, to_vertex: &Arc<Vertex>) -> Option<Vec<Arc<Vertex>>> {
    match self.next.get(from_vertex) {
      Some(inner_map) => {
        match inner_map.get(to_vertex) {
          Some(_) => (),
          None => return None
        }
      },
      None => return None
    }

    let mut path = Vec::new();
    let mut from = from_vertex.clone();
    path.push(from.clone());
    
    while from != to_vertex.clone() {
      from = self.next.get(&from).unwrap().get(to_vertex).unwrap().clone();
      path.push(from.clone());
    }
    Some(path)
  }

}