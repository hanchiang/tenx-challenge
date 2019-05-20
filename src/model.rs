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

