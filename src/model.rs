#[derive(Debug)]
pub enum InputType {
    ExchangeRateRequest(ExchangeRateRequest),
    PriceUpdate(PriceUpdate),
    Invalid(String)
}

#[derive(Debug)]
pub struct PriceUpdate {
    pub datetime: u64,
    pub exchange: String,
    pub source_currency: String,
    pub dest_currency: String,
    pub forward_ratio: f64,
    pub backward_ratio: f64
}

#[derive(Debug)]
pub struct ExchangeRateRequest {
    pub source_exchange: String,
    pub source_currency: String,
    pub dest_exchange: String,
    pub dest_currency: String
}