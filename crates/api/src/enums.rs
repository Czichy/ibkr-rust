use std::{fmt::Formatter, str::FromStr};

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use serde::{Deserialize, Serialize};

// use serde::Deserialize;
use crate::utils::ib_message::{Decodable, Encodable};

pub mod constants {
    pub const CLIENT_VERSION: i32 = 203;
    pub const MIN_SERVER_VER_PRICE_MGMT_ALGO: i32 = 151;
    pub const MIN_SERVER_VER_PROTOBUF: i32 = 203;

    pub const MIN_CLIENT_VER: i32 = 100;
    pub const MAX_CLIENT_VER: i32 = 197; // MIN_SERVER_VER_PRICE_MGMT_ALGO;
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

#[derive(Debug, FromPrimitive, Copy, Clone, PartialEq, Eq, Hash)]
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
    HistoricalDataEnd       = 108,
    CurrentTimeInMillis     = 109,
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

#[derive(Clone, Debug, Default)]
pub struct TagValue {
    pub tag:   String,
    pub value: String,
}

impl TagValue {
    pub const fn new(tag: String, value: String) -> Self { TagValue { tag, value } }
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ComboAction {
    Buy,
    Sell,
    ShortSell,
}

impl Encodable for ComboAction {
    fn encode(&self) -> String {
        match self {
            ComboAction::Buy => "BUY\0",
            ComboAction::Sell => "SELL\0",
            ComboAction::ShortSell => "SSELL\0",
        }
        .to_string()
    }
}

impl FromStr for ComboAction {
    type Err = ParseEnumError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "BUY" => Ok(ComboAction::Buy),
            "SELL" => Ok(ComboAction::Sell),
            "SSELL" => Ok(ComboAction::ShortSell),
            &_ => Err(ParseEnumError),
        }
    }
}

impl Decodable for ComboAction {}

// #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
// #[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OptionOpenClose {
    Same,
    Open,
    Close,
    Unknown,
}

impl Encodable for OptionOpenClose {
    fn encode(&self) -> String {
        match self {
            OptionOpenClose::Same => "0\0",
            OptionOpenClose::Open => "1\0",
            OptionOpenClose::Close => "2\0",
            OptionOpenClose::Unknown => "3\0",
        }
        .to_string()
    }
}

impl FromStr for OptionOpenClose {
    type Err = ParseEnumError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(OptionOpenClose::Same),
            "1" => Ok(OptionOpenClose::Open),
            "2" => Ok(OptionOpenClose::Close),
            "3" => Ok(OptionOpenClose::Unknown),
            &_ => Err(ParseEnumError),
        }
    }
}

impl Decodable for OptionOpenClose {}

// #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
// #[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShortSaleSlot {
    NoSlot,
    Broker,
    ThirdParty,
}

impl Encodable for ShortSaleSlot {
    fn encode(&self) -> String {
        match self {
            ShortSaleSlot::NoSlot => "0\0",
            ShortSaleSlot::Broker => "1\0",
            ShortSaleSlot::ThirdParty => "2\0",
        }
        .to_string()
    }
}

impl FromStr for ShortSaleSlot {
    type Err = ParseEnumError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(ShortSaleSlot::NoSlot),
            "1" => Ok(ShortSaleSlot::Broker),
            "2" => Ok(ShortSaleSlot::ThirdParty),
            &_ => Err(ParseEnumError),
        }
    }
}

impl Decodable for ShortSaleSlot {}

// #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
// #[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
#[derive(Debug, Clone, Copy, Default)]
pub enum Action {
    #[default]
    Buy,
    Sell,
    SellShort,
    SellLong,
}

impl Encodable for Action {
    fn encode(&self) -> String {
        match self {
            Action::Buy => "BUY\0",
            Action::Sell => "SELL\0",
            Action::SellShort => "SSELL\0",
            Action::SellLong => "SLONG\0",
        }
        .to_string()
    }
}

impl FromStr for Action {
    type Err = ParseEnumError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "BUY" => Ok(Action::Buy),
            "SELL" => Ok(Action::Sell),
            "SSELL" => Ok(Action::SellShort),
            "SLONG" => Ok(Action::SellLong),
            &_ => Err(ParseEnumError),
        }
    }
}

impl Decodable for Action {}

// #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
// #[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
#[derive(Debug, Clone, Copy)]
pub enum OrderOpenClose {
    Open,
    Close,
}

impl Encodable for OrderOpenClose {
    fn encode(&self) -> String {
        match self {
            OrderOpenClose::Open => "O\0",
            OrderOpenClose::Close => "C\0",
        }
        .to_string()
    }
}

impl FromStr for OrderOpenClose {
    type Err = ParseEnumError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "O" => Ok(OrderOpenClose::Open),
            "C" => Ok(OrderOpenClose::Close),
            &_ => Err(ParseEnumError),
        }
    }
}

impl Decodable for OrderOpenClose {}

// #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
// #[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
pub enum OrderType {
    #[default]
    NoOrderType, // only legit for deltaNeutralOrderType
    Limit,
    Market,
    MarketIfTouched,
    MarketOnClose,
    MarketOnOpen,
    PeggedToMarket,
    PeggedToStock,
    PeggedToPrimary,
    BoxTop,
    LimitIfTouched,
    LimitOnClose,
    PassiveRelative,
    PeggedToMidpoint,
    MarketToLimit,
    MarketWithProtection,
    MidPrice,
    Stop,
    StopLimit,
    StopWithProtection,
    TrailingStop,
    TrailingStopLimit,
    RelativeLimit,
    RelativeMarket,
    Volatility,
    PeggedToBenchmark,
}
impl Encodable for OrderType {
    fn encode(&self) -> String {
        match self {
            OrderType::NoOrderType => "None\0",
            OrderType::Limit => "LMT\0",
            OrderType::Market => "MKT\0",
            OrderType::MarketIfTouched => "MIT\0",
            OrderType::MarketOnClose => "MOC\0",
            OrderType::MarketOnOpen => "MOO\0",
            OrderType::PeggedToMarket => "PEG MKT\0",
            OrderType::PeggedToStock => "PEG STK\0",
            OrderType::PeggedToPrimary => "REL\0",
            OrderType::BoxTop => "BOX TOP\0",
            OrderType::LimitIfTouched => "LIT\0",
            OrderType::LimitOnClose => "LOC\0",
            OrderType::PassiveRelative => "PASSV REL\0",
            OrderType::PeggedToMidpoint => "PEG MID\0",
            OrderType::MarketToLimit => "MTL\0",
            OrderType::MarketWithProtection => "MKT PRT\0",
            OrderType::MidPrice => "MIDPRICE\0",
            OrderType::Stop => "STP\0",
            OrderType::StopLimit => "STP LMT\0",
            OrderType::StopWithProtection => "STP PRT\0",
            OrderType::TrailingStop => "TRAIL\0",
            OrderType::TrailingStopLimit => "TRAIL LIMIT\0",
            OrderType::RelativeLimit => "Rel + LMT\0",
            OrderType::RelativeMarket => "Rel + MKT\0",
            OrderType::Volatility => "VOL\0",
            OrderType::PeggedToBenchmark => "PEG BENCH\0",
        }
        .to_string()
    }
}

impl FromStr for OrderType {
    type Err = ParseEnumError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "None" => Ok(OrderType::NoOrderType),
            "Keine" => Ok(OrderType::NoOrderType),
            "LMT" => Ok(OrderType::Limit),
            "MKT" => Ok(OrderType::Market),
            "MIT" => Ok(OrderType::MarketIfTouched),
            "MOC" => Ok(OrderType::MarketOnClose),
            "MOO" => Ok(OrderType::MarketOnOpen),
            "PEG MKT" => Ok(OrderType::PeggedToMarket),
            "PEG STK" => Ok(OrderType::PeggedToStock),
            "REL" => Ok(OrderType::PeggedToPrimary),
            "BOX TOP" => Ok(OrderType::BoxTop),
            "LIT" => Ok(OrderType::LimitIfTouched),
            "LOC" => Ok(OrderType::LimitOnClose),
            "PASSV REL" => Ok(OrderType::PassiveRelative),
            "PEG MID" => Ok(OrderType::PeggedToMidpoint),
            "MTL" => Ok(OrderType::MarketToLimit),
            "MKT PRT" => Ok(OrderType::MarketWithProtection),
            "MIDPRICE" => Ok(OrderType::MidPrice),
            "STP" => Ok(OrderType::Stop),
            "STP LMT" => Ok(OrderType::StopLimit),
            "STP PRT" => Ok(OrderType::StopWithProtection),
            "TRAIL" => Ok(OrderType::TrailingStop),
            "TRAIL LIMIT" => Ok(OrderType::TrailingStopLimit),
            "REL + LMT" => Ok(OrderType::RelativeLimit),
            "REL + MKT" => Ok(OrderType::RelativeMarket),
            "VOL" => Ok(OrderType::Volatility),
            "PEG BENCH" => Ok(OrderType::PeggedToBenchmark),
            &_ => Err(ParseEnumError),
        }
    }
}
impl Decodable for OrderType {}

// #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
// #[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
#[derive(Debug, Clone, Copy)]
pub enum TriggerMethod {
    Default,
    DoubleBidAsk,
    Last,
    DoubleLast,
    BidAsk,
    LastOrBidAsk,
    MidPoint,
}

impl Encodable for TriggerMethod {
    fn encode(&self) -> String {
        match self {
            TriggerMethod::Default => "0\0",
            TriggerMethod::DoubleBidAsk => "1\0",
            TriggerMethod::Last => "2\0",
            TriggerMethod::DoubleLast => "3\0",
            TriggerMethod::BidAsk => "4\0",
            TriggerMethod::LastOrBidAsk => "7\0",
            TriggerMethod::MidPoint => "8\0",
        }
        .to_string()
    }
}

impl FromStr for TriggerMethod {
    type Err = ParseEnumError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(TriggerMethod::Default),
            "1" => Ok(TriggerMethod::DoubleBidAsk),
            "2" => Ok(TriggerMethod::Last),
            "3" => Ok(TriggerMethod::DoubleLast),
            "4" => Ok(TriggerMethod::BidAsk),
            "7" => Ok(TriggerMethod::LastOrBidAsk),
            "8" => Ok(TriggerMethod::MidPoint),
            &_ => Err(ParseEnumError),
        }
    }
}

impl Decodable for TriggerMethod {}

// #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
// #[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
#[derive(Debug, Clone, Copy)]
/// The time in force.
/// Valid values are:
/// DAY - Valid for the day only.
/// GTC - Good until canceled. The order will continue to work within the system
/// and in the marketplace until it executes or is canceled. GTC orders will be
/// automatically be cancelled under the following conditions: If a corporate
/// action on a security results in a stock split (forward or reverse), exchange
/// for shares, or distribution of shares. If you do not log into your IB
/// account for 90 days. At the end of the calendar quarter following the
/// current quarter. For example, an order placed during the third quarter of
/// 2011 will be canceled at the end of the first quarter of 2012. If the last
/// day is a non-trading day, the cancellation will occur at the close of the
/// final trading day of that quarter. For example, if the last day of the
/// quarter is Sunday, the orders will be cancelled on the preceding Friday.
/// Orders that are modified will be assigned a new “Auto Expire” date
/// consistent with the end of the calendar quarter following the current
/// quarter. Orders submitted to IB that remain in force for more than one day
/// will not be reduced for dividends. To allow adjustment to your order price
/// on ex-dividend date, consider using a Good-Til-Date/Time (GTD) or
/// Good-after-Time/Date (GAT) order type, or a combination of the two.
/// IOC - Immediate or Cancel. Any portion that is not filled as soon as it
/// becomes available in the market is canceled. GTD - Good until Date. It will
/// remain working within the system and in the marketplace until it executes or
/// until the close of the market on the date specified OPG - Use OPG to send a
/// market-on-open (MOO) or limit-on-open (LOO) order. FOK - If the entire
/// Fill-or-Kill order does not execute as soon as it becomes available, the
/// entire order is canceled. DTC - Day until Canceled.
pub enum TimeInForce {
    Day,
    GoodTillCancel,
    ImmediateOrCancel,
    GoodUntilDate,
    GoodOnOpen,
    FillOrKill,
    DayUntilCancel,
}

impl Encodable for TimeInForce {
    fn encode(&self) -> String {
        match self {
            TimeInForce::Day => "DAY\0",
            TimeInForce::GoodTillCancel => "GTC\0",
            TimeInForce::ImmediateOrCancel => "IOC\0",
            TimeInForce::GoodUntilDate => "GTD\0",
            TimeInForce::GoodOnOpen => "OPG\0",
            TimeInForce::FillOrKill => "FOK\0",
            TimeInForce::DayUntilCancel => "DTC\0",
        }
        .to_string()
    }
}

impl FromStr for TimeInForce {
    type Err = ParseEnumError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "DAY" => Ok(TimeInForce::Day),
            "GTC" => Ok(TimeInForce::GoodTillCancel),
            "IOC" => Ok(TimeInForce::ImmediateOrCancel),
            "GTD" => Ok(TimeInForce::GoodTillCancel),
            "OPG" => Ok(TimeInForce::GoodOnOpen),
            "FOK" => Ok(TimeInForce::FillOrKill),
            "DTC" => Ok(TimeInForce::DayUntilCancel),
            &_ => Err(ParseEnumError),
        }
    }
}

impl Decodable for TimeInForce {}

// #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
// #[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
#[derive(Debug, Clone, Copy)]
pub enum Rule80A {
    Individual,
    Agency,
    AgentOtherMember,
    IndividualPTIA,
    AgencyPTIA,
    AgentOtherMemberPTIA,
    IndividualPT,
    AgencyPT,
    AgentOtherMemberPT,
    None,
}

impl Encodable for Rule80A {
    fn encode(&self) -> String {
        match *self {
            Rule80A::Individual => "I\0",
            Rule80A::Agency => "A\0",
            Rule80A::AgentOtherMember => "W\0",
            Rule80A::IndividualPTIA => "J\0",
            Rule80A::AgencyPTIA => "U\0",
            Rule80A::AgentOtherMemberPTIA => "M\0",
            Rule80A::IndividualPT => "K\0",
            Rule80A::AgencyPT => "Y\0",
            Rule80A::AgentOtherMemberPT => "N\0",
            Rule80A::None => "0\0",
        }
        .to_string()
    }
}

impl FromStr for Rule80A {
    type Err = ParseEnumError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "I" => Ok(Rule80A::Individual),
            "A" => Ok(Rule80A::Agency),
            "W" => Ok(Rule80A::AgentOtherMember),
            "J" => Ok(Rule80A::IndividualPTIA),
            "U" => Ok(Rule80A::AgencyPTIA),
            "M" => Ok(Rule80A::AgentOtherMemberPTIA),
            "K" => Ok(Rule80A::IndividualPT),
            "Y" => Ok(Rule80A::AgencyPT),
            "N" => Ok(Rule80A::AgentOtherMemberPT),
            "0" => Ok(Rule80A::None),
            &_ => Err(ParseEnumError),
        }
    }
}

impl Decodable for Rule80A {}

// #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
// #[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
#[derive(Debug, Clone, Copy)]
pub enum Origin {
    Customer,
    Firm,
    Unknown,
}

impl Encodable for Origin {
    fn encode(&self) -> String {
        match self {
            Origin::Customer => "0\0",
            Origin::Firm => "1\0",
            Origin::Unknown => "2\0",
        }
        .to_string()
    }
}

impl FromStr for Origin {
    type Err = ParseEnumError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(Origin::Customer),
            "1" => Ok(Origin::Firm),
            "2" => Ok(Origin::Unknown),
            &_ => Err(ParseEnumError),
        }
    }
}

impl Decodable for Origin {}

// #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
// #[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
#[derive(Debug, Clone, Copy)]
pub enum AuctionStrategy {
    NoAuctionStrategy,
    Match,
    Improvement,
    Transparent,
}

impl Encodable for AuctionStrategy {
    fn encode(&self) -> String {
        match self {
            Self::NoAuctionStrategy => "0\0",
            Self::Match => "1\0",
            Self::Improvement => "2\0",
            Self::Transparent => "3\0",
        }
        .to_string()
    }
}

impl FromStr for AuctionStrategy {
    type Err = ParseEnumError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(Self::NoAuctionStrategy),
            "1" => Ok(Self::Match),
            "2" => Ok(Self::Improvement),
            "3" => Ok(Self::Transparent),

            &_ => Err(ParseEnumError),
        }
    }
}

impl Decodable for AuctionStrategy {}

// #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
// #[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
#[derive(Debug, Clone, Copy)]
/// Tells how to handle remaining orders in an OCA group when one order or part
/// of an order executes. Valid values are:
/// 1 - Cancel all remaining orders with block.
/// 2 - Remaining orders are proportionately reduced in size with block.
/// 3 - Remaining orders are proportionately reduced in size with no block.
/// If you use a value "with block" it gives the order overfill protection. This
/// means that only one order in the group will be routed at a time to remove
/// the possibility of an overfill.
pub enum OCAType {
    NoOCAType,
    CancelWithBlock,
    ReduceWithBlock,
    ReduceNonBlock,
}

impl Encodable for OCAType {
    fn encode(&self) -> String {
        match self {
            OCAType::NoOCAType => "0\0",
            OCAType::CancelWithBlock => "1\0",
            OCAType::ReduceWithBlock => "2\0",
            OCAType::ReduceNonBlock => "3\0",
        }
        .to_string()
    }
}

impl FromStr for OCAType {
    type Err = ParseEnumError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(OCAType::NoOCAType),
            "1" => Ok(OCAType::CancelWithBlock),
            "2" => Ok(OCAType::ReduceWithBlock),
            "3" => Ok(OCAType::ReduceNonBlock),
            &_ => Err(ParseEnumError),
        }
    }
}

impl Decodable for OCAType {}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum VolatilityType {
    #[default]
    NoVolType,
    Daily,
    Annual,
}

impl Encodable for VolatilityType {
    fn encode(&self) -> String {
        match self {
            VolatilityType::NoVolType => "0\0",
            VolatilityType::Daily => "1\0",
            VolatilityType::Annual => "2\0",
        }
        .to_string()
    }
}

impl FromStr for VolatilityType {
    type Err = ParseEnumError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(VolatilityType::NoVolType),
            "1" => Ok(VolatilityType::Daily),
            "2" => Ok(VolatilityType::Annual),
            &_ => Err(ParseEnumError),
        }
    }
}

impl Decodable for VolatilityType {}

// #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
// #[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
#[derive(Debug, Clone, Copy, Default)]
pub enum ReferencePriceType {
    #[default]
    NoRefPriceType,
    Average,
    BidOrAsk,
}

impl Encodable for ReferencePriceType {
    fn encode(&self) -> String {
        match self {
            ReferencePriceType::NoRefPriceType => "0\0",
            ReferencePriceType::Average => "1\0",
            ReferencePriceType::BidOrAsk => "2\0",
        }
        .to_string()
    }
}

impl FromStr for ReferencePriceType {
    type Err = ParseEnumError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(ReferencePriceType::NoRefPriceType),
            "1" => Ok(ReferencePriceType::Average),
            "2" => Ok(ReferencePriceType::BidOrAsk),
            &_ => Err(ParseEnumError),
        }
    }
}

impl Decodable for ReferencePriceType {}

// #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
// #[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
#[derive(Debug, Clone, Copy)]
pub enum BasisPointsType {
    Undefined,
}

impl Encodable for BasisPointsType {
    fn encode(&self) -> String {
        match self {
            BasisPointsType::Undefined => "?\0",
        }
        .to_string()
    }
}

impl FromStr for BasisPointsType {
    type Err = ParseEnumError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "?" => Ok(BasisPointsType::Undefined),
            &_ => Err(ParseEnumError),
        }
    }
}

impl Decodable for BasisPointsType {}

// #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
// #[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
#[derive(PartialEq, Debug, Clone, Copy, Eq)]
pub enum HedgeType {
    Undefined,
    Delta,
    Beta,
    Forex,
    Pair,
}

impl Encodable for HedgeType {
    fn encode(&self) -> String {
        match self {
            HedgeType::Undefined => "?\0",
            HedgeType::Delta => "D\0",
            HedgeType::Beta => "B\0",
            HedgeType::Forex => "F\0",
            HedgeType::Pair => "P\0",
        }
        .to_string()
    }
}

impl FromStr for HedgeType {
    type Err = ParseEnumError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "?" => Ok(HedgeType::Undefined),
            "D" => Ok(HedgeType::Delta),
            "B" => Ok(HedgeType::Beta),
            "F" => Ok(HedgeType::Forex),
            "P" => Ok(HedgeType::Pair),
            &_ => Err(ParseEnumError),
        }
    }
}

impl Decodable for HedgeType {}

// #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
// #[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
#[derive(Debug, Clone, Copy)]
pub enum ClearingIntent {
    InteractiveBrokers,
    Away,
    PTA,
}

impl Encodable for ClearingIntent {
    fn encode(&self) -> String {
        match self {
            ClearingIntent::InteractiveBrokers => "IB\0",
            ClearingIntent::Away => "Away\0",
            ClearingIntent::PTA => "PTA\0",
        }
        .to_string()
    }
}

impl FromStr for ClearingIntent {
    type Err = ParseEnumError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "IB" => Ok(ClearingIntent::InteractiveBrokers),
            "Away" => Ok(ClearingIntent::Away),
            "PTA" => Ok(ClearingIntent::PTA),
            &_ => Err(ParseEnumError),
        }
    }
}

impl Decodable for ClearingIntent {}
// #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
// #[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
#[derive(Debug, Clone, Copy)]
pub enum Side {
    Buy,
    Sell,
}

impl Encodable for Side {
    fn encode(&self) -> String {
        match self {
            Side::Buy => "BOT\0",
            Side::Sell => "SLD\0",
        }
        .to_string()
    }
}

impl FromStr for Side {
    type Err = ParseEnumError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "BOT" => Ok(Side::Buy),
            "SLD" => Ok(Side::Sell),
            "BUY" => Ok(Side::Sell),
            "SELL" => Ok(Side::Sell),
            &_ => Err(ParseEnumError),
        }
    }
}

impl Decodable for Side {}
