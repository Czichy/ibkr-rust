use std::str::{FromStr, Split};

use bitvec::prelude::*;
use crossbeam::channel::{unbounded, Receiver, Sender};
use derive_more::From;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

use crate::{
    bars::{HistoricalBars, HistoricalSchedule, RealtimeBar},
    ib_frame::{ParseError, ParseIbkrFrame, ParseResult},
    prelude::{
        constants::UNSET_INTEGER,
        ib_message::{decode, Decodable, Encodable},
        DepthSide, Incoming, MarketDepthOperation, ParseEnumError,
    },
    MarketDataValueType, RequestId, TimeStamp,
};

#[derive(Debug, Clone)]
pub struct MarketDataTracker {
    pub tick_by_tick: Receiver<Tick>,
    pub bars: Receiver<RealtimeBar>,
    pub market_depth: Receiver<MarketDepthUpdate>,
    pub historical_ticks: Receiver<HistoricalTicks>,
    pub historical_bars: Receiver<HistoricalBars>,
    pub historical_schedule: Receiver<HistoricalSchedule>,
    pub head_timestamp: Receiver<HeadTimestamp>,
}
#[allow(dead_code)]
pub(crate) struct MarketDataTrackerSender {
    pub tick_by_tick_tx: Sender<Tick>,
    pub market_depth_tx: Sender<MarketDepthUpdate>,
    pub bars_tx: Sender<RealtimeBar>,
    pub historical_ticks_tx: Sender<HistoricalTicks>,
    pub historical_bars_tx: Sender<HistoricalBars>,
    pub historical_schedule_tx: Sender<HistoricalSchedule>,
    pub head_timestamp_tx: Sender<HeadTimestamp>,
}
impl MarketDataTracker {
    pub(crate) fn new() -> (MarketDataTrackerSender, Self) {
        let (tick_by_tick_tx, tick_by_tick) = unbounded();
        let (market_depth_tx, market_depth) = unbounded();
        let (bars_tx, bars) = unbounded();
        let (historical_ticks_tx, historical_ticks) = unbounded();
        let (historical_bars_tx, historical_bars) = unbounded();
        let (historical_schedule_tx, historical_schedule) = unbounded();
        let (head_timestamp_tx, head_timestamp) = unbounded();
        (
            MarketDataTrackerSender {
                tick_by_tick_tx,
                bars_tx,
                market_depth_tx,
                historical_ticks_tx,
                historical_bars_tx,
                historical_schedule_tx,
                head_timestamp_tx,
            },
            MarketDataTracker {
                tick_by_tick,
                bars,
                market_depth,
                historical_bars,
                historical_ticks,
                historical_schedule,
                head_timestamp,
            },
        )
    }
}

#[derive(Clone, Debug, From)]
pub enum Tick {
    //
    TickByTickAllLast(TickByTickAllLast),
    //
    TickByTickBidAsk(TickByTickBidAsk),
    //
    TickByTickMidPoint(TickByTickMidPoint),
    //
    Price(TickPrice),
    //
    Size(TickSize),
    //
    String(TickString),
    //
    Generic(TickGeneric),
}

impl ParseIbkrFrame for Tick {
    fn try_parse_frame(msg_id: Incoming, it: &mut Split<&str>) -> ParseResult<Self>
    where
        Self: Sized,
    {
        if !matches!(msg_id, Incoming::TickByTick) {
            return Err(ParseError::UnexpectedMessage);
        }
        // it.next(); // skip version
        let id = decode(it)?.unwrap();
        let tick_type = decode(it)?.unwrap();
        let time = decode(it)?.unwrap();

        match tick_type {
            TickByTickType::NA =>
            // TODO:
            {
                Err(ParseError::UnexpectedMessage)
            },
            // None
            TickByTickType::Last | TickByTickType::AllLast =>
            // Last (1) or AllLast (2)
            {
                let tick = TickByTickAllLast {
                    id,
                    tick_type,
                    time,
                    price: decode(it)?.unwrap(),
                    size: decode(it)?.unwrap(),
                    tick_attrib_last: {
                        let mask: u32 = decode(it)?.unwrap();
                        TickAttribLast {
                            past_limit: mask & 1 != 0,
                            unreported: mask & 2 != 0,
                        }
                    },
                    exchange: decode(it)?.unwrap(),
                    special_conditions: decode(it)?.unwrap(),
                };
                Ok(Self::TickByTickAllLast(tick))
            },
            TickByTickType::BidAsk =>
            // BidAsk
            {
                let tick = TickByTickBidAsk {
                    id,
                    time,
                    bid_price: decode(it)?.unwrap(),
                    ask_price: decode(it)?.unwrap(),
                    bid_size: decode(it)?.unwrap(),
                    ask_size: decode(it)?.unwrap(),
                    tick_attrib_bid_ask: {
                        let mask: u32 = decode(it)?.unwrap();
                        TickAttribBidAsk {
                            bid_past_low: mask & 1 != 0,
                            ask_past_high: mask & 2 != 0,
                        }
                    },
                };
                Ok(Self::TickByTickBidAsk(tick))
            },
            TickByTickType::MidPoint =>
            // MidPoint
            {
                let tick = TickByTickMidPoint {
                    id,
                    time,
                    mid_point: decode(it)?.unwrap(),
                };
                Ok(Self::TickByTickMidPoint(tick)) // self.wrapper
            },
        }
    }
}

#[derive(Clone, Debug, From)]
pub enum HistoricalTimeAndSales {
    BidAsk(HistoricalBidAsk),
    //
    Tick(HistoricalTick),
    //
    Last(HistoricalLast),
}

#[derive(Clone, Debug)]
pub struct TickByTickAllLast {
    pub id: RequestId,
    pub tick_type: TickByTickType,
    pub time: TimeStamp,
    pub price: MarketDataValueType,
    pub size: MarketDataValueType,
    pub tick_attrib_last: TickAttribLast,
    pub exchange: String,
    pub special_conditions: String,
}

#[derive(Clone, Debug, Copy)]
pub struct TickByTickBidAsk {
    pub id: RequestId,
    pub time: TimeStamp,
    pub bid_price: MarketDataValueType,
    pub ask_price: MarketDataValueType,
    pub bid_size: MarketDataValueType,
    pub ask_size: MarketDataValueType,
    pub tick_attrib_bid_ask: TickAttribBidAsk,
}

#[derive(Clone, Debug, Copy)]
pub struct TickByTickMidPoint {
    pub id: RequestId,
    pub time: TimeStamp,
    pub mid_point: MarketDataValueType,
}

#[derive(Clone, Debug, Copy)]
pub enum ShortAvailability {
    Available,
    HardToBorrow,
    Unavailable,
}

impl ShortAvailability {
    pub fn from_f64(val: f64) -> Self {
        if val > 2.5 {
            Self::Available
        } else if val > 1.5 {
            Self::HardToBorrow
        } else {
            Self::Unavailable
        }
    }
}
#[derive(Clone, Debug, Default, Copy)]
pub struct TickAttribute {
    pub can_auto_execute: bool,
    pub past_limit: bool,
    pub pre_open: bool,
}

impl TickAttribute {
    pub const fn new(can_auto_execute: bool, past_limit: bool, pre_open: bool) -> Self {
        TickAttribute {
            can_auto_execute,
            past_limit,
            pre_open,
        }
    }
}
#[derive(Clone, Debug, Default, Copy)]
pub struct TickAttribBidAsk {
    pub bid_past_low: bool,
    pub ask_past_high: bool,
}

impl TickAttribBidAsk {
    pub const fn new(bid_past_low: bool, ask_past_high: bool) -> Self {
        TickAttribBidAsk {
            bid_past_low,
            ask_past_high,
        }
    }
}
#[derive(Clone, Debug, Default, Copy)]
pub struct TickAttribLast {
    pub past_limit: bool,
    pub unreported: bool,
}

impl TickAttribLast {
    pub const fn new(past_limit: bool, unreported: bool) -> Self {
        TickAttribLast {
            past_limit,
            unreported,
        }
    }
}

#[derive(Clone, Debug, Copy)]
pub struct TickPrice {
    pub id: RequestId,
    pub kind: TickType,
    pub price: MarketDataValueType,
    pub size: Option<MarketDataValueType>,
    pub attributes: TickAttribute,
}
impl ParseIbkrFrame for TickPrice {
    fn try_parse_frame(msg_id: Incoming, it: &mut Split<&str>) -> ParseResult<Self>
    where
        Self: Sized,
    {
        if !matches!(msg_id, Incoming::TickPrice) {
            return Err(ParseError::UnexpectedMessage);
        }
        it.next(); // skip version
        let id = decode(it)?.unwrap();
        let kind = decode(it)?.unwrap();
        let price = decode(it)?.unwrap();

        let size = decode(it)?;
        let mask: u32 = decode(it)?.unwrap();
        let bits = mask.view_bits::<LocalBits>();
        let attributes = TickAttribute {
            can_auto_execute: bits[0],
            past_limit: bits[1],
            pre_open: bits[2],
        };
        Ok(Self {
            id,
            kind,
            price,
            size,
            attributes,
        })
    }
}

#[derive(Clone, Debug, Copy)]
pub struct TickSize {
    pub id: RequestId,
    pub kind: TickType,
    pub size: MarketDataValueType,
}
impl ParseIbkrFrame for TickSize {
    fn try_parse_frame(msg_id: Incoming, it: &mut Split<&str>) -> ParseResult<Self>
    where
        Self: Sized,
    {
        if !matches!(msg_id, Incoming::TickSize) {
            return Err(ParseError::UnexpectedMessage);
        }
        it.next(); // skip version
        Ok(Self {
            id: decode(it)?.unwrap(),
            kind: decode(it)?.unwrap(),
            size: decode(it)?.unwrap(),
        })
    }
}
//
#[derive(Clone, Debug)]
pub struct TickString {
    pub id: RequestId,
    pub kind: TickType,
    pub val: Option<String>,
}
impl ParseIbkrFrame for TickString {
    fn try_parse_frame(msg_id: Incoming, it: &mut Split<&str>) -> ParseResult<Self>
    where
        Self: Sized,
    {
        if !matches!(msg_id, Incoming::TickString) {
            return Err(ParseError::UnexpectedMessage);
        }
        it.next(); // skip version
        Ok(Self {
            id: decode(it)?.unwrap(),
            kind: decode(it)?.unwrap(),
            val: decode(it)?,
        })
    }
}
//
#[derive(Clone, Debug, Copy)]
pub struct TickGeneric {
    pub id: RequestId,
    pub kind: TickType,
    pub val: MarketDataValueType,
}
impl ParseIbkrFrame for TickGeneric {
    fn try_parse_frame(msg_id: Incoming, it: &mut Split<&str>) -> ParseResult<Self>
    where
        Self: Sized,
    {
        if !matches!(msg_id, Incoming::TickGeneric) {
            return Err(ParseError::UnexpectedMessage);
        }
        it.next(); // skip version
        Ok(Self {
            id: decode(it)?.unwrap(),
            kind: decode(it)?.unwrap(),
            val: decode(it)?.unwrap(),
        })
    }
}

#[derive(Clone, Debug, Copy)]
pub struct HistoricalBidAsk {
    pub time: TimeStamp,
    pub attributes: TickAttribute,
    pub price_bid: MarketDataValueType,
    pub price_ask: MarketDataValueType,
    pub size_bid: MarketDataValueType,
    pub size_ask: MarketDataValueType,
}
impl ParseIbkrFrame for HistoricalBidAsk {
    fn try_parse_frame(msg_id: Incoming, it: &mut Split<&str>) -> ParseResult<Self>
    where
        Self: Sized,
    {
        if !matches!(msg_id, Incoming::HistoricalTicksLast) {
            return Err(ParseError::UnexpectedMessage);
        }
        let time = decode(it)?.unwrap();
        let mask: u32 = decode(it)?.unwrap();
        let bits = mask.view_bits::<LocalBits>();
        let attributes = TickAttribute {
            can_auto_execute: bits[0],
            past_limit: bits[1],
            pre_open: bits[2],
        };
        Ok(Self {
            time,
            attributes,
            price_bid: decode(it)?.unwrap(),
            price_ask: decode(it)?.unwrap(),
            size_bid: decode(it)?.unwrap(),
            size_ask: decode(it)?.unwrap(),
        })
    }
}

#[derive(Clone, Debug)]
pub struct HistoricalTicks {
    pub id: RequestId,
    pub ticks: Vec<HistoricalTimeAndSales>,
    pub done: bool,
}
impl ParseIbkrFrame for HistoricalTicks {
    fn try_parse_frame(msg_id: Incoming, it: &mut Split<&str>) -> ParseResult<Self>
    where
        Self: Sized,
    {
        match msg_id {
            Incoming::HistoricalTicks => {
                let id = decode(it)?.unwrap();
                let tick_count = decode(it)?.unwrap();
                let ticks = {
                    let mut ticks = Vec::with_capacity(tick_count);
                    for _i in 0..tick_count {
                        ticks.push(HistoricalTimeAndSales::Tick(
                            HistoricalTick::try_parse_frame(msg_id, it)?,
                        ));
                    }
                    ticks
                };
                Ok(Self {
                    id,
                    ticks,
                    done: decode(it)?.unwrap(),
                })
            },
            Incoming::HistoricalTicksBidAsk => {
                let id = decode(it)?.unwrap();
                let tick_count = decode(it)?.unwrap();
                let ticks = {
                    let mut ticks = Vec::with_capacity(tick_count);
                    for _i in 0..tick_count {
                        ticks.push(HistoricalTimeAndSales::BidAsk(
                            HistoricalBidAsk::try_parse_frame(msg_id, it)?,
                        ));
                    }
                    ticks
                };
                tracing::error!("{:#?}", ticks);
                Ok(Self {
                    id,
                    ticks,
                    done: decode(it)?.unwrap(),
                })
            },
            Incoming::HistoricalTicksLast => {
                let id = decode(it)?.unwrap();
                let tick_count = decode(it)?.unwrap();
                let ticks = {
                    let mut ticks = Vec::with_capacity(tick_count);
                    for _i in 0..tick_count {
                        let tick = HistoricalTimeAndSales::Last(HistoricalLast::try_parse_frame(
                            msg_id, it,
                        )?);
                        ticks.push(tick);
                    }
                    ticks
                };
                Ok(Self {
                    id,
                    ticks,
                    done: decode(it)?.unwrap(),
                })
            },

            _ => Err(ParseError::UnexpectedMessage),
        }
    }
}

#[derive(Clone, Debug, Copy)]
pub struct HistoricalTick {
    pub time: TimeStamp,
    pub price: MarketDataValueType,
    pub size: MarketDataValueType,
}
impl ParseIbkrFrame for HistoricalTick {
    fn try_parse_frame(msg_id: Incoming, it: &mut Split<&str>) -> ParseResult<Self>
    where
        Self: Sized,
    {
        if !matches!(msg_id, Incoming::HistoricalTicks) {
            return Err(ParseError::UnexpectedMessage);
        }
        Ok(Self {
            time: decode(it)?.unwrap(),
            price: decode(it)?.unwrap(),
            size: decode(it)?.unwrap(),
        })
    }
}

#[derive(Clone, Debug)]
pub struct HistoricalLast {
    pub time: TimeStamp,
    pub attributes: TickAttribute,
    pub price: MarketDataValueType,
    pub size: MarketDataValueType,
    pub exchange: String,
    pub special_conditions: Option<String>,
}
impl ParseIbkrFrame for HistoricalLast {
    fn try_parse_frame(msg_id: Incoming, it: &mut Split<&str>) -> ParseResult<Self>
    where
        Self: Sized,
    {
        if !matches!(msg_id, Incoming::HistoricalTicksLast) {
            return Err(ParseError::UnexpectedMessage);
        }
        let time = decode(it)?.unwrap();
        let mask: u32 = decode(it)?.unwrap();
        let bits = mask.view_bits::<LocalBits>();
        let attributes = TickAttribute {
            can_auto_execute: bits[0],
            past_limit: bits[1],
            pre_open: bits[2],
        };
        Ok(Self {
            time, //: decode(it)?.unwrap(),
            attributes,
            price: decode(it)?.unwrap(),
            size: decode(it)?.unwrap(),
            exchange: decode(it)?.unwrap(),
            special_conditions: decode(it)?,
        })
    }
}

#[derive(Clone, Debug, Copy)]
pub struct HeadTimestamp {
    pub id: RequestId,
    pub timestamp: TimeStamp,
}
#[derive(Clone, Debug)]
pub struct MarketDepthUpdate {
    pub id: RequestId,
    pub position: usize,
    pub operation: MarketDepthOperation,
    pub side: DepthSide,
    pub price: MarketDataValueType,
    pub size: MarketDataValueType,
    pub is_smart_depth: String,
}
#[repr(i32)]
#[derive(FromPrimitive, Debug, Clone, enum_ordinalize::Ordinalize, Copy)]
pub enum TickType {
    BidSize = 0,
    Bid = 1,
    Ask = 2,
    AskSize = 3,
    Last = 4,
    LastSize = 5,
    High = 6,
    Low = 7,
    Volume = 8,
    Close = 9,
    BidOptionComputation = 10,
    AskOptionComputation = 11,
    LastOptionComputation = 12,
    ModelOption = 13,
    Open = 14,
    Low13Week = 15,
    High13Week = 16,
    Low26Week = 17,
    High26Week = 18,
    Low52Week = 19,
    High52Week = 20,
    AvgVolume = 21,
    OpenInterest = 22,
    OptionHistoricalVol = 23,
    OptionImpliedVol = 24,
    OptionBidExch = 25,
    OptionAskExch = 26,
    OptionCallOpenInterest = 27,
    OptionPutOpenInterest = 28,
    OptionCallVolume = 29,
    OptionPutVolume = 30,
    IndexFuturePremium = 31,
    BidExch = 32,
    AskExch = 33,
    AuctionVolume = 34,
    AuctionPrice = 35,
    AuctionImbalance = 36,
    MarkPrice = 37,
    BidEfpComputation = 38,
    AskEfpComputation = 39,
    LastEfpComputation = 40,
    OpenEfpComputation = 41,
    HighEfpComputation = 42,
    LowEfpComputation = 43,
    CloseEfpComputation = 44,
    LastTimestamp = 45,
    Shortable = 46,
    FundamentalRatios = 47,
    RtVolume = 48,
    Halted = 49,
    BidYield = 50,
    AskYield = 51,
    LastYield = 52,
    CustOptionComputation = 53,
    TradeCount = 54,
    TradeRate = 55,
    VolumeRate = 56,
    LastRthTrade = 57,
    RtHistoricalVol = 58,
    IbDividends = 59,
    BondFactorMultiplier = 60,
    RegulatoryImbalance = 61,
    NewsTick = 62,
    ShortTermVolume3Min = 63,
    ShortTermVolume5Min = 64,
    ShortTermVolume10Min = 65,
    DelayedBid = 66,
    DelayedAsk = 67,
    DelayedLast = 68,
    DelayedBidSize = 69,
    DelayedAskSize = 70,
    DelayedLastSize = 71,
    DelayedHigh = 72,
    DelayedLow = 73,
    DelayedVolume = 74,
    DelayedClose = 75,
    DelayedOpen = 76,
    RtTrdVolume = 77,
    CreditmanMarkPrice = 78,
    CreditmanSlowMarkPrice = 79,
    DelayedBidOption = 80,
    DelayedAskOption = 81,
    DelayedLastOption = 82,
    DelayedModelOption = 83,
    LastExch = 84,
    LastRegTime = 85,
    FuturesOpenInterest = 86,
    AvgOptVolume = 87,
    DelayedLastTimestamp = 88,
    ShortableShares = 89,
    NotSet = UNSET_INTEGER,
}

impl FromStr for TickType {
    type Err = ParseEnumError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ord = match s.parse::<i32>() {
            Ok(n) => n,
            Err(_) => return Err(ParseEnumError),
        };
        FromPrimitive::from_i32(ord).map_or(Err(ParseEnumError), Ok)
    }
}
impl Encodable for TickType {
    fn encode(&self) -> String {
        let ord = self.ordinal();
        ord.to_string() + "\0"
    }
}

impl Decodable for TickType {}

#[derive(Debug, Clone, Copy)]
pub enum GenericTickType {
    ShortableData,
    HistoricData,
    OptionHistoricalVol,
    OptionImpliedVol,
    OptionOpenInterest,
    AuctionData,
    OptionVolume,
}

impl Encodable for GenericTickType {
    fn encode(&self) -> String {
        match self {
            GenericTickType::ShortableData => "236",
            GenericTickType::HistoricData => "165",
            GenericTickType::OptionHistoricalVol => "10",
            GenericTickType::OptionImpliedVol => "106",
            GenericTickType::OptionOpenInterest => "101",
            GenericTickType::AuctionData => "225",
            GenericTickType::OptionVolume => "100",
        }
        .to_string()
    }
}
/// Tick by tick types
#[derive(Clone, Debug, Copy)]
pub enum TickByTickType {
    NA = 0,
    Last = 1,
    AllLast = 2,
    BidAsk = 3,
    MidPoint = 4,
}
impl FromStr for TickByTickType {
    type Err = ParseEnumError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let res = match s {
            "0" => TickByTickType::NA,
            "1" => TickByTickType::Last,
            "2" => TickByTickType::AllLast,
            "3" => TickByTickType::BidAsk,
            "4" => TickByTickType::MidPoint,
            &_ => return Err(ParseEnumError),
        };
        Ok(res)
    }
}
impl Encodable for TickByTickType {
    fn encode(&self) -> String {
        match self {
            TickByTickType::Last => "Last\0",
            TickByTickType::BidAsk => "BidAsk\0",
            TickByTickType::AllLast => "AllLast\0",
            TickByTickType::MidPoint => "MidPoint\0",
            TickByTickType::NA => "\0",
        }
        .to_string()
    }
}

impl Decodable for TickByTickType {}
