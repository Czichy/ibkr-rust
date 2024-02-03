use std::{fmt::Formatter, str::FromStr};

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

// use serde::Deserialize;
use crate::utils::ib_message::{Decodable, Encodable};

pub mod constants {
    pub const CLIENT_VERSION: i32 = 66;
    pub const MIN_SERVER_VER_PRICE_MGMT_ALGO: i32 = 151;
    pub const MIN_CLIENT_VER: i32 = 100;
    pub const MAX_CLIENT_VER: i32 = 163; // MIN_SERVER_VER_PRICE_MGMT_ALGO;
    pub const UNSET_INTEGER: i32 = std::i32::MAX;
}

#[derive(Debug, Clone, Copy)]
pub enum ServerLogLevel {
    System      = 1,
    Error       = 2,
    Warning     = 3,
    Information = 4,
    Detail      = 5,
}

impl Encodable for ServerLogLevel {
    fn encode(&self) -> String {
        match self {
            ServerLogLevel::System => "1\0",
            ServerLogLevel::Error => "2\0",
            ServerLogLevel::Warning => "3\0",
            ServerLogLevel::Information => "4\0",
            ServerLogLevel::Detail => "5\0",
        }
        .to_string()
    }
}

impl FromStr for ServerLogLevel {
    type Err = ParseEnumError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let res = match s {
            "1" => ServerLogLevel::System,
            "2" => ServerLogLevel::Error,
            "3" => ServerLogLevel::Warning,
            "4" => ServerLogLevel::Information,
            "5" => ServerLogLevel::Detail,
            &_ => return Err(ParseEnumError),
        };
        Ok(res)
    }
}

impl Decodable for ServerLogLevel {}

#[derive(Debug, FromPrimitive, Copy, Clone)]
pub enum Incoming {
    TickPrice               = 1,
    TickSize                = 2,
    OrderStatus             = 3,
    ErrMsg                  = 4,
    OpenOrder               = 5,
    AcctValue               = 6,
    PortfolioValue          = 7,
    AcctUpdateTime          = 8,
    NextValidId             = 9,
    ContractData            = 10,
    ExecutionData           = 11,
    MarketDepth             = 12,
    MarketDepthL2           = 13,
    NewsBulletins           = 14,
    ManagedAccts            = 15,
    ReceiveFa               = 16,
    HistoricalData          = 17,
    BondContractData        = 18,
    ScannerParameters       = 19,
    ScannerData             = 20,
    TickOptionComputation   = 21,
    TickGeneric             = 45,
    TickString              = 46,
    TickEfp                 = 47,
    CurrentTime             = 49,
    RealTimeBars            = 50,
    FundamentalData         = 51,
    ContractDataEnd         = 52,
    OpenOrderEnd            = 53,
    AcctDownloadEnd         = 54,
    ExecutionDataEnd        = 55,
    DeltaNeutralValidation  = 56,
    TickSnapshotEnd         = 57,
    MarketDataType          = 58,
    CommissionReport        = 59,
    PositionData            = 61,
    PositionEnd             = 62,
    AccountSummary          = 63,
    AccountSummaryEnd       = 64,
    VerifyMessageApi        = 65,
    VerifyCompleted         = 66,
    DisplayGroupList        = 67,
    DisplayGroupUpdated     = 68,
    VerifyAndAuthMessageApi = 69,
    VerifyAndAuthCompleted  = 70,
    PositionMulti           = 71,
    PositionMultiEnd        = 72,
    AccountUpdateMulti      = 73,
    AccountUpdateMultiEnd   = 74,
    SecurityDefinitionOptionParameter = 75,
    SecurityDefinitionOptionParameterEnd = 76,
    SoftDollarTiers         = 77,
    FamilyCodes             = 78,
    SymbolSamples           = 79,
    MktDepthExchanges       = 80,
    TickReqParams           = 81,
    SmartComponents         = 82,
    NewsArticle             = 83,
    TickNews                = 84,
    NewsProviders           = 85,
    HistoricalNews          = 86,
    HistoricalNewsEnd       = 87,
    HeadTimestamp           = 88,
    HistogramData           = 89,
    HistoricalDataUpdate    = 90,
    RerouteMktDataReq       = 91,
    RerouteMktDepthReq      = 92,
    MarketRule              = 93,
    PnL                     = 94,
    PnlSingle               = 95,
    HistoricalTicks         = 96,
    HistoricalTicksBidAsk   = 97,
    HistoricalTicksLast     = 98,
    TickByTick              = 99,
    OrderBound              = 100,
    CompletedOrder          = 101,
    CompletedOrdersEnd      = 102,
    ReplaceFAEnd            = 103,
    WshMetaData             = 104,
    WshEventData            = 105,
    HistoricalSchedule      = 106,
    UserInfo                = 107,
}

impl FromStr for Incoming {
    type Err = ParseEnumError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ord = match s.parse::<i32>() {
            Ok(n) => n,
            Err(_) => return Err(ParseEnumError),
        };
        FromPrimitive::from_i32(ord).ok_or(ParseEnumError)
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Outgoing {
    // outgoing message IDs
    ReqMktData                 = 1,
    CancelMktData              = 2,
    PlaceOrder                 = 3,
    CancelOrder                = 4,
    ReqOpenOrders              = 5,
    ReqAcctData                = 6,
    ReqExecutions              = 7,
    ReqIds                     = 8,
    ReqContractData            = 9,
    ReqMktDepth                = 10,
    CancelMktDepth             = 11,
    ReqNewsBulletins           = 12,
    CancelNewsBulletins        = 13,
    SetServerLoglevel          = 14,
    ReqAutoOpenOrders          = 15,
    ReqAllOpenOrders           = 16,
    ReqManagedAccts            = 17,
    ReqFa                      = 18,
    ReplaceFa                  = 19,
    ReqHistoricalData          = 20,
    ExerciseOptions            = 21,
    ReqScannerSubscription     = 22,
    CancelScannerSubscription  = 23,
    ReqScannerParameters       = 24,
    CancelHistoricalData       = 25,
    ReqCurrentTime             = 49,
    ReqRealTimeBars            = 50,
    CancelRealTimeBars         = 51,
    ReqFundamentalData         = 52,
    CancelFundamentalData      = 53,
    ReqCalcImpliedVolat        = 54,
    ReqCalcOptionPrice         = 55,
    CancelCalcImpliedVolat     = 56,
    CancelCalcOptionPrice      = 57,
    ReqGlobalCancel            = 58,
    ReqMarketDataType          = 59,
    ReqPositions               = 61,
    ReqAccountSummary          = 62,
    CancelAccountSummary       = 63,
    CancelPositions            = 64,
    VerifyRequest              = 65,
    VerifyMessage              = 66,
    QueryDisplayGroups         = 67,
    SubscribeToGroupEvents     = 68,
    UpdateDisplayGroup         = 69,
    UnsubscribeFromGroupEvents = 70,
    StartApi                   = 71,
    VerifyAndAuthRequest       = 72,
    VerifyAndAuthMessage       = 73,
    ReqPositionsMulti          = 74,
    CancelPositionsMulti       = 75,
    ReqAccountUpdatesMulti     = 76,
    CancelAccountUpdatesMulti  = 77,
    ReqSecDefOptParams         = 78,
    ReqSoftDollarTiers         = 79,
    ReqFamilyCodes             = 80,
    ReqMatchingSymbols         = 81,
    ReqMktDepthExchanges       = 82,
    ReqSmartComponents         = 83,
    ReqNewsArticle             = 84,
    ReqNewsProviders           = 85,
    ReqHistoricalNews          = 86,
    ReqHeadTimestamp           = 87,
    ReqHistogramData           = 88,
    CancelHistogramData        = 89,
    CancelHeadTimestamp        = 90,
    ReqMarketRule              = 91,
    ReqPnl                     = 92,
    CancelPnl                  = 93,
    ReqPnlSingle               = 94,
    CancelPnlSingle            = 95,
    ReqHistoricalTicks         = 96,
    ReqTickByTickData          = 97,
    CancelTickByTickData       = 98,
    ReqCompletedOrders         = 99,
}

impl Encodable for Outgoing {
    fn encode(&self) -> String {
        let ord = *self as i32;
        ord.to_string() + "\0"
    }
}

use std::fmt::Display;
#[derive(Debug, Clone, Copy)]
pub struct ParseEnumError;
impl Display for ParseEnumError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "Unknown Enum value")
    }
}
// Some enums are only for decoding and implement the FromStr trait
// Some enums are only for encoding and implement the encode method (might make
// it a trait)
#[derive(Debug, Clone, Copy)]
pub enum MarketDataType {
    RealTime      = 1,
    Frozen        = 2,
    Delayed       = 3,
    FrozenDelayed = 4,
}

impl Encodable for MarketDataType {
    fn encode(&self) -> String {
        match self {
            MarketDataType::RealTime => "1\0",
            MarketDataType::Frozen => "2\0",
            MarketDataType::Delayed => "3\0",
            MarketDataType::FrozenDelayed => "4\0",
        }
        .to_string()
    }
}

impl FromStr for MarketDataType {
    type Err = ParseEnumError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let res = match s {
            "1" => MarketDataType::RealTime,
            "2" => MarketDataType::Frozen,
            "3" => MarketDataType::Delayed,
            "4" => MarketDataType::FrozenDelayed,
            &_ => return Err(ParseEnumError),
        };
        Ok(res)
    }
}

impl Decodable for MarketDataType {}

#[derive(Debug, Clone, Copy)]
pub enum FundamentalDataType {
    Snapshot,
    FinSummary,
    Ratios,
    FinStatements,
    Estimates,
}

impl Encodable for FundamentalDataType {
    fn encode(&self) -> String {
        match self {
            FundamentalDataType::Snapshot => "ReportSnapShot\0",
            FundamentalDataType::FinSummary => "ReportsFinSummary\0",
            FundamentalDataType::Ratios => "ReportRatios\0",
            FundamentalDataType::FinStatements => "ReportsFinStatements\0",
            FundamentalDataType::Estimates => "RESC\0",
        }
        .to_string()
    }
}

// #[derive(Debug, Clone, Copy)]
// /// use regular trading hours only, 1 for yes or 0 for no
// pub enum UseRegularTradingHoursOnly {
//     DontUse,
//     Use,
// }

// impl Encodable for UseRegularTradingHoursOnly {
//     fn encode(&self) -> String {
//         match self {
//             Self::DontUse => "0\0",
//             Self::Use => "1\0",
//         }
//         .to_string()
//     }
// }

// impl FromStr for UseRegularTradingHoursOnly {
//     type Err = ParseEnumError;

//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         match s {
//             "0" => Ok(Self::DontUse),
//             "1" => Ok(Self::Use),
//             &_ => Err(ParseEnumError),
//         }
//     }
// }

// impl Decodable for UseRegularTradingHoursOnly {}

// #[derive(Debug, Clone, Copy)]
// pub enum UsePriceMgmtAlgo {
//     DontUse,
//     Use,
// }

// impl Encodable for UsePriceMgmtAlgo {
//     fn encode(&self) -> String {
//         match self {
//             UsePriceMgmtAlgo::DontUse => "0\0",
//             UsePriceMgmtAlgo::Use => "1\0",
//         }
//         .to_string()
//     }
// }

// impl FromStr for UsePriceMgmtAlgo {
//     type Err = ParseEnumError;

//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         match s {
//             "0" => Ok(UsePriceMgmtAlgo::DontUse),
//             "1" => Ok(UsePriceMgmtAlgo::Use),
//             &_ => Err(ParseEnumError),
//         }
//     }
// }

// impl Decodable for UsePriceMgmtAlgo {}
// #[derive(Debug, Clone, Deserialize, Copy)]
// pub enum Side {
//     #[serde(rename = "BUY")]
//     Buy,
//     #[serde(rename = "SELL")]
//     Sell,
// }

// impl Encodable for Side {
//     fn encode(&self) -> String {
//         match self {
//             Side::Buy => "BOT\0",
//             Side::Sell => "SLD\0",
//         }
//         .to_string()
//     }
// }

// impl FromStr for Side {
//     type Err = ParseEnumError;

//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         match s {
//             "BOT" => Ok(Side::Buy),
//             "SLD" => Ok(Side::Sell),
//             "BUY" => Ok(Side::Sell),
//             "SELL" => Ok(Side::Sell),
//             &_ => Err(ParseEnumError),
//         }
//     }
// }

// impl Decodable for Side {}

// #[derive(FromPrimitive, Debug, Clone, enum_ordinalize::Ordinalize, Copy)]
// pub enum OrderConditionType {
//     Price = 1,
//     Time = 3,
//     Margin = 4,
//     Execution = 5,
//     Volume = 6,
//     PercentChange = 7,
// }

// impl Encodable for OrderConditionType {
//     fn encode(&self) -> String {
//         let ord = self.ordinal();
//         ord.to_string() + "\0"
//     }
// }

// impl FromStr for OrderConditionType {
//     type Err = ParseEnumError;

//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         let ord = match s.parse::<i32>() {
//             Ok(n) => n,
//             Err(_) => return Err(ParseEnumError),
//         };
//         match FromPrimitive::from_i32(ord) {
//             Some(en_type) => Ok(en_type),
//             None => Err(ParseEnumError),
//         }
//     }
// }

// #[derive(Debug, Clone, Copy)]
// pub enum OrderStatus {
//     /// indicates that you have transmitted the order, but have not yet
// received     /// confirmation that it has been accepted by the order
// destination.     /// This order status is not sent by TWS and should be
// explicitly set by the     /// API developer when an order is submitted.
//     PendingSubmit,

//     /// PendingCancel - indicates that you have sent a request to cancel the
//     /// order but have not yet received cancel confirmation from the order
//     /// destination. At this point, your order is not confirmed canceled.
//     /// You may still receive an execution while your cancellation request
//     /// is pending. This order status is not sent by TWS and should be
//     /// explicitly set by the API developer when an order is canceled.
//     PendingCancel,

//     /// indicates that a simulated order type has been accepted by the IB
// system     /// and that this order has yet to be elected. The order is held
// in the     /// IB system (and the status remains DARK BLUE) until the
// election     /// criteria are met. At that time the order is transmitted to
// the order     /// destination as specified (and the order status color will
// change).     PreSubmitted,

//     /// indicates that your order has been accepted at the order destination
// and     /// is working.
//     Submitted,

//     /// indicates that the balance of your order has been confirmed canceled
// by     /// the IB system. This could occur unexpectedly when IB or the
//     /// destination has rejected your order.
//     Cancelled,

//     /// The order has been completely filled.
//     Filled,

//     /// The Order is inactive
//     Inactive,

//     /// The order is Partially Filled
//     PartiallyFilled,

//     /// Api Pending
//     ApiPending,

//     /// Api Cancelled
//     ApiCancelled,

//     /// Indicates that there is an error with this order
//     /// This order status is not sent by TWS and should be explicitly set by
// the     /// API developer when an error has occured.
//     Error,

//     /// No Order Status
//     None,
// }

// impl Default for OrderStatus {
//     fn default() -> Self {
//         OrderStatus::None
//     }
// }

// impl Encodable for OrderStatus {
//     fn encode(&self) -> String {
//         match self {
//             OrderStatus::PendingSubmit => "PendingSubmit\0",

//             OrderStatus::PendingCancel => "PendingCancel\0",

//             OrderStatus::PreSubmitted => "PreSubmitted\0",

//             OrderStatus::Submitted => "Submitted\0",

//             OrderStatus::Cancelled => "Cancelled\0",

//             OrderStatus::Filled => "Filled\0",

//             OrderStatus::Inactive => "Inactive\0",

//             OrderStatus::PartiallyFilled => "PartiallyFilled\0",

//             OrderStatus::ApiPending => "ApiPending\0",

//             OrderStatus::ApiCancelled => "ApiCancelled\0",

//             OrderStatus::Error => "Error\0",

//             OrderStatus::None => "\0",
//         }
//         .to_string()
//     }
// }

// impl FromStr for OrderStatus {
//     type Err = ParseEnumError;

//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         match s {
//             "PendingSubmit" => Ok(OrderStatus::PendingSubmit),

//             "PendingCancel" => Ok(OrderStatus::PendingCancel),

//             "PreSubmitted" => Ok(OrderStatus::PreSubmitted),

//             "Submitted" => Ok(OrderStatus::Submitted),

//             "Cancelled" => Ok(OrderStatus::Cancelled),

//             "Filled" => Ok(OrderStatus::Filled),

//             "Inactive" => Ok(OrderStatus::Inactive),

//             "PartiallyFilled" => Ok(OrderStatus::PartiallyFilled),

//             "ApiPending" => Ok(OrderStatus::ApiPending),

//             "ApiCancelled" => Ok(OrderStatus::ApiCancelled),

//             "Error" => Ok(OrderStatus::Error),

//             "" => Ok(OrderStatus::None),
//             &_ => Err(ParseEnumError),
//         }
//     }
// }

// impl Decodable for OrderStatus {}

// impl Decodable for OrderConditionType {}
#[derive(Clone, Debug, Default)]
pub struct TagValue {
    pub tag:   String,
    pub value: String,
}

impl TagValue {
    pub const fn new(tag: String, value: String) -> Self { TagValue { tag, value } }
}
// #[derive(Debug, PartialEq, Eq, Clone, Copy)]
// pub enum HistoricalDataType {
//     Trades,
//     Midpoint,
//     Bid,
//     Ask,
//     BidAsk,
//     AdjustedLast,
//     HistoricalVolatility,
//     OptionImpliedVolatility,
//     RebateRate,
//     FeeRate,
//     YieldBid,
//     YieldAsk,
//     YieldBidAsk,
//     YieldLast,
//     Schedule,
// }

// impl Encodable for HistoricalDataType {
//     fn encode(&self) -> String {
//         match self {
//             HistoricalDataType::Trades => "TRADES\0",
//             HistoricalDataType::Midpoint => "MIDPOINT\0",
//             HistoricalDataType::Bid => "BID\0",
//             HistoricalDataType::Ask => "ASK\0",
//             HistoricalDataType::BidAsk => "BID_ASK\0",
//             HistoricalDataType::AdjustedLast => "ADJUSTED_LAST\0",
//             HistoricalDataType::HistoricalVolatility =>
// "HISTORICAL_VOLATILITY\0",
// HistoricalDataType::OptionImpliedVolatility => "OPTION_IMPLIED_VOLATILITY\0",
//             HistoricalDataType::RebateRate => "REBATE_RATE\0",
//             HistoricalDataType::FeeRate => "FEE_RATE\0",
//             HistoricalDataType::YieldBid => "YIELD_BID\0",
//             HistoricalDataType::YieldAsk => "YIELD_ASK\0",
//             HistoricalDataType::YieldBidAsk => "YIELD_BID_ASK\0",
//             HistoricalDataType::YieldLast => "YIELD_LAST\0",
//             HistoricalDataType::Schedule => "SCHEDULE\0",
//         }
//         .to_string()
//     }
// }
// impl Default for HistoricalDataType {
//     fn default() -> Self {
//         Self::Trades
//     }
// }

// impl FromStr for HistoricalDataType {
//     type Err = ParseEnumError;

//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         let res = match s {
//             "TRADES" => HistoricalDataType::Trades,
//             "MIDPOINT" => HistoricalDataType::Midpoint,
//             "BID" => HistoricalDataType::Bid,
//             "ASK" => HistoricalDataType::Ask,
//             "BID_ASK" => HistoricalDataType::BidAsk,
//             "ADJUSTED_LAST" => HistoricalDataType::AdjustedLast,
//             "HISTORICAL_VOLATILITY" =>
// HistoricalDataType::HistoricalVolatility,
// "OPTION_IMPLIED_VOLATILITY" => HistoricalDataType::OptionImpliedVolatility,
//             "REBATE_RATE" => HistoricalDataType::RebateRate,
//             "FEE_RATE" => HistoricalDataType::FeeRate,
//             "YIELD_BID" => HistoricalDataType::YieldBid,
//             "YIELD_ASK" => HistoricalDataType::YieldAsk,
//             "YIELD_BID_ASK" => HistoricalDataType::YieldBidAsk,
//             "YIELD_LAST" => HistoricalDataType::YieldLast,
//             "SCHEDULE" => HistoricalDataType::Schedule,
//             &_ => return Err(ParseEnumError),
//         };
//         Ok(res)
//     }
// }

// impl Display for HistoricalDataType {
//     fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
//         write!(f, "{:?}", self)
//     }
// }

// impl Decodable for HistoricalDataType {}

// #[derive(Debug, PartialEq, Eq, Clone, Copy)]
// pub enum BarSize {
//     _1Secs,
//     _5Secs,
//     _10Secs,
//     _15Secs,
//     _30Secs,
//     _1Min,
//     _2Mins,
//     _3Mins,
//     _5Mins,
//     _10Mins,
//     _15Mins,
//     _20Mins,
//     _30Mins,
//     _1Hour,
//     _4Hours,
//     _1Day,
//     _1Week,
//     _1Month,
// }

// impl Encodable for BarSize {
//     fn encode(&self) -> String {
//         match self {
//             BarSize::_1Secs => "1 secs\0",
//             BarSize::_5Secs => "5 secs\0",
//             BarSize::_10Secs => "10 secs\0",
//             BarSize::_15Secs => "15 secs\0",
//             BarSize::_30Secs => "30 secs\0",
//             BarSize::_1Min => "1 min\0",
//             BarSize::_2Mins => "2 mins\0",
//             BarSize::_3Mins => "3 mins\0",
//             BarSize::_5Mins => "5 mins\0",
//             BarSize::_10Mins => "10 mins\0",
//             BarSize::_15Mins => "15 mins\0",
//             BarSize::_20Mins => "20 mins\0",
//             BarSize::_30Mins => "30 mins\0",
//             BarSize::_1Hour => "1 hour\0",
//             BarSize::_4Hours => "4 hours\0",
//             BarSize::_1Day => "1 day\0",
//             BarSize::_1Week => "1 week\0",
//             BarSize::_1Month => "1 month\0",
//         }
//         .to_string()
//     }
// }
// impl Default for BarSize {
//     fn default() -> Self {
//         Self::_1Min
//     }
// }

// impl FromStr for BarSize {
//     type Err = ParseEnumError;

//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         let res = match s {
//             "1 secs" => BarSize::_1Secs,
//             "5 secs" => BarSize::_5Secs,
//             "10 secs" => BarSize::_10Secs,
//             "15 secs" => BarSize::_15Secs,
//             "30 secs" => BarSize::_30Secs,
//             "1 min" => BarSize::_1Min,
//             "2 mins" => BarSize::_2Mins,
//             "3 mins" => BarSize::_3Mins,
//             "5 mins" => BarSize::_5Mins,
//             "10 mins" => BarSize::_10Mins,
//             "15 mins" => BarSize::_15Mins,
//             "20 mins" => BarSize::_20Mins,
//             "30 mins" => BarSize::_30Mins,
//             "1 hour" => BarSize::_1Hour,
//             "4 hours" => BarSize::_4Hours,
//             "1 day" => BarSize::_1Day,
//             "1 week" => BarSize::_1Week,
//             "1 month" => BarSize::_1Month,
//             &_ => return Err(ParseEnumError),
//         };
//         Ok(res)
//     }
// }

// impl Display for BarSize {
//     fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
//         write!(f, "{:?}", self)
//     }
// }
// impl Decodable for BarSize {}

// #[derive(Debug, PartialEq, Eq, Clone, Copy)]
// pub enum Duration {
//     Seconds(u32),
//     Day(u32),
//     Week(u32),
//     Month(u32),
//     Year(u32),
// }

// impl Encodable for Duration {
//     fn encode(&self) -> String {
//         match self {
//             Duration::Seconds(value) => format!("{} {}\0", value, 'S'),
//             Duration::Day(value) => format!("{} {}\0", value, 'D'),
//             Duration::Week(value) => format!("{} {}\0", value, 'W'),
//             Duration::Month(value) => format!("{} {}\0", value, 'M'),
//             Duration::Year(value) => format!("{} {}\0", value, 'Y'),
//         }
//     }
// }
// impl Default for Duration {
//     fn default() -> Self {
//         Self::Day(1)
//     }
// }

// impl FromStr for Duration {
//     type Err = ParseEnumError;

//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         let mut s = s.split_whitespace();
//         // if s.len() != 2 {
//         //     Err(ParseEnumError)
//         // } else {
//         // let res = if let Some(value) =
//         s.next()
//             .and_then(|v| v.parse::<u32>().ok())
//             .and_then(|value| {
//                 s.next().and_then(|d| d.chars().next()).map(|d| match d {
//                     'S' => Ok(Duration::Seconds(value)),
//                     'D' => Ok(Duration::Day(value)),
//                     'W' => Ok(Duration::Week(value)),
//                     'M' => Ok(Duration::Month(value)),
//                     'Y' => Ok(Duration::Year(value)),
//                     _ => Err(ParseEnumError),
//                 })
//             })
//             .unwrap_or(Err(ParseEnumError))
//     }
// }

// impl Display for Duration {
//     fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
//         write!(f, "{:?}", self)
//     }
// }
// impl Decodable for Duration {}
