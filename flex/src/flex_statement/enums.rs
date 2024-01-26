use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub enum BuySell {
    #[serde(rename = "BUY")]
    BUY,
    #[serde(rename = "SELL")]
    SELL,
    #[serde(rename = "SELL (Ca.)")]
    SELLCa,
}
#[derive(Debug, Deserialize)]
pub enum PutCall {
    P,
    C,
}

#[derive(Debug, Deserialize)]
pub enum AssetCategory {
    STK,
    OPT,
    FOP,
    CFD,
    FUT,
    CASH,
    FXCFD,
    BOND,
}

#[derive(Debug, Deserialize)]
pub enum Currency {
    EUR,
    USD,
    JPY,
    CHF,
    GBP,
    NZD,
    AUD,
    CAD,
    SEK,
    HKD,
    MXN,
    RUB,
    NOK,
    ZAR,
}
