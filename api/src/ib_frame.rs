use std::{convert::TryInto, io::Cursor, marker::Sized, str::Split, string::FromUtf8Error};

use bytes::Buf;
use chrono::{DateTime, TimeZone, Timelike, Utc};
use derive_more::From;

use crate::{account::{AccountData, AccountLastUpdate, Position},
            bars::{HistoricalBars, RealtimeBar},
            contract,
            contract::Contract,
            enums::*,
            order::{CommissionReport, Execution, OrderInformation, OrderStatusUpdate},
            prelude::HistoricalSchedule,
            ticker::{HeadTimestamp,
                     HistoricalTicks,
                     Tick,
                     TickGeneric,
                     TickPrice,
                     TickSize,
                     TickString},
            utils::ib_message::{decode, IbDecodeError},
            AccountCode,
            OrderId,
            RequestId,
            TimeStamp};

pub type ParseResult<T, E = ParseError> = Result<T, E>;

pub trait FromIbkrFrame {
    /// Converts the command into an equivalent `Frame`.
    ///
    /// This is called by the client when encoding a command
    /// to send to the server.

    #[allow(clippy::wrong_self_convention)]
    fn try_into_frame(msg_id: Incoming, it: Split<&str>) -> ParseResult<IBFrame>
    where
        Self: Sized;
}

pub trait ParseIbkrFrame {
    /// Converts the command into an equivalent `Frame`.
    ///
    /// This is called by the client when encoding a command
    /// to send to the server.

    #[allow(clippy::wrong_self_convention)]
    fn try_parse_frame(msg_id: Incoming, it: &mut Split<&str>) -> ParseResult<Self>
    where
        Self: Sized;
}

#[derive(Debug, thiserror::Error)]
#[allow(dead_code)]
pub enum ParseError {
    #[error("decode error: {}", _0)]
    Decoding(#[from] IbDecodeError),

    #[error("Not enough data is available to parse a message")]
    Incomplete,

    #[error("Invalid message encoding")]
    Other(crate::prelude::Error),

    #[error("Unexpected variant: {}", _0)]
    UnexpectedVariant(String),

    #[error("Unexpected message type")]
    UnexpectedMessage,

    #[error("protocol error; invalid frame format.")]
    Protocol(#[from] FromUtf8Error),
}

// TODO: split into separate structs to implement parse
#[derive(Debug, strum_macros::Display, From)]
pub enum IBFrame {
    AccountCode(AccountCode),

    AccountSummary(AccountData),

    #[from(ignore)]
    AccountUpdateEnd(AccountCode),

    #[from(ignore)]
    AccountUpdateTime(AccountLastUpdate),

    #[from(ignore)]
    AccountValue(AccountData),

    CommissionReport(CommissionReport),

    CompletedOrder(OrderInformation),

    ContractDetails {
        req_id:           RequestId,
        contract_details: contract::ContractDetails,
    },

    ContractDetailsEnd(RequestId),

    CurrentTime(TimeStamp),

    Error {
        req_id:  i32,
        status:  i32,
        message: Option<String>,
    },

    Execution(Execution),

    HeadTimestamp(HeadTimestamp),

    HistoricalBars(HistoricalBars),

    HistoricalSchedule(HistoricalSchedule),

    HistoricalTicks(HistoricalTicks),

    #[from(ignore)]
    NotImplemented,

    #[from(ignore)]
    OpenOrder(OrderInformation),

    #[from(ignore)]
    OpenOrderEnd,

    OrderId(OrderId),

    OrderStatus(OrderStatusUpdate),

    PortfolioValue(Position),

    RealtimeBar(RealtimeBar),

    ServerVersion {
        server_version:  i32,
        connection_time: String,
    },

    Tick(Tick),
}

impl IBFrame {
    /// Checks if an entire message can be decoded from `src`
    pub fn check(src: &mut Cursor<&[u8]>) -> ParseResult<()> {
        if !src.has_remaining() {
            return Err(ParseError::Incomplete);
        }
        if src.get_ref().len() < 4 {
            return Err(ParseError::Incomplete);
        }
        let start = src.position() as usize;
        let headbuf: [u8; 4] = src.get_ref()[0..4].try_into().expect("Incomplete");
        let msg_size = u32::from_be_bytes(headbuf) as usize;
        tracing::debug!("message: {:?} with size {:?}", headbuf, msg_size);
        let end = src.get_ref().len() - 1;
        tracing::debug!("start: {:?} end: {:?} msg_size: {:?}", start, end, msg_size);
        if end < (start + msg_size + 3) {
            tracing::warn!(
                "incomplete message! expectiong len: {}, msg_size: {}",
                end,
                (start + msg_size + 3)
            );
            return Err(ParseError::Incomplete);
        }
        src.set_position((start + msg_size + 4) as u64);
        Ok(())
    }

    #[allow(clippy::cognitive_complexity)]
    // TODO: Simplify
    pub fn parse(src: &mut Cursor<&[u8]>) -> ParseResult<IBFrame> {
        let msg = read(src)?;
        let utf8msg = String::from_utf8_lossy(msg);
        tracing::debug!("trying to parse message: {:?}", utf8msg);
        #[allow(clippy::single_char_pattern)]
        let mut it = utf8msg.split("\0");
        let msg_id: Incoming = it
            .next()
            .unwrap()
            .parse()
            .expect("Could not parse message type.");
        tracing::debug!("incoming message: {:?}", msg_id);
        match msg_id {
            Incoming::ManagedAccts => {
                // skip version
                it.next();
                let code = decode::<String>(&mut it)?.unwrap();
                Ok(code.into())
            },
            Incoming::AccountSummary => {
                Ok(IBFrame::AccountSummary(AccountData::try_parse_frame(
                    msg_id, &mut it,
                )?))
            },

            Incoming::AcctValue => {
                Ok(IBFrame::AccountValue(AccountData::try_parse_frame(
                    msg_id, &mut it,
                )?))
            },

            Incoming::AcctDownloadEnd => {
                it.next(); // skip version
                let code = decode::<String>(&mut it)?.unwrap();
                Ok(IBFrame::AccountUpdateEnd(code))
            },

            Incoming::AcctUpdateTime => {
                it.next(); // skip version
                let time = decode::<String>(&mut it)?.unwrap();
                let mut time = time.split(':');
                if let (Some(hour), Some(minute)) = (
                    time.next().and_then(|h| h.parse().ok()),
                    time.next().and_then(|m| m.parse().ok()),
                ) {
                    let dt = chrono::Local::now()
                        .with_hour(hour)
                        .unwrap()
                        .with_minute(minute)
                        .unwrap()
                        .with_second(0)
                        .unwrap()
                        .with_nanosecond(0)
                        .unwrap();
                    Ok(IBFrame::AccountUpdateTime(DateTime::from(dt)))
                } else {
                    Err(ParseError::Incomplete)
                }
            },
            Incoming::PortfolioValue => {
                let _version: i32 = decode(&mut it)?.unwrap();
                Ok(IBFrame::PortfolioValue(Position {
                    contract:       Contract::try_parse_frame(msg_id, &mut it)?,
                    position:       decode(&mut it)?,
                    market_price:   decode(&mut it)?,
                    market_value:   decode(&mut it)?,
                    average_cost:   decode(&mut it)?,
                    unrealized_pnl: decode(&mut it)?,
                    realized_pnl:   decode(&mut it)?,
                }))
            },
            Incoming::CurrentTime => {
                it.next(); // skip version
                let unix_time: i64 = decode(&mut it)?.unwrap();
                Ok(IBFrame::CurrentTime(
                    Utc.timestamp_opt(unix_time, 0).unwrap(),
                ))
            },
            Incoming::ContractData => {
                tracing::debug!("decode ContractData");
                // let msg_version: usize = decode(&mut it)?.unwrap();
                // tracing::error!("message version: {}", msg_version);
                let req_id: usize = //if msg_version >= 3 {
                    decode(&mut it)?.unwrap();
                // } else {
                //     0
                // };
                let mut contract = contract::Contract::try_parse_frame(msg_id, &mut it)?;
                tracing::debug!("decoded contract: {:#?}", contract);
                let mut details = contract::ContractDetails {
                    market_name: decode(&mut it)?,
                    ..Default::default()
                };
                contract.trading_class = decode(&mut it)?;
                contract.con_id = decode(&mut it)?;
                details.min_tick = decode(&mut it)?;
                // details.md_size_multiplier = decode(&mut it)?;
                contract.multiplier = decode(&mut it)?;
                details.order_types = decode(&mut it)?;
                details.valid_exchanges = decode(&mut it)?;
                details.price_magnifier = decode(&mut it)?;
                details.under_con_id = decode(&mut it)?;
                details.long_name = decode(&mut it)?;
                contract.primary_exchange = decode(&mut it)?;
                details.contract_month = decode(&mut it)?;
                details.industry = decode(&mut it)?;
                details.category = decode(&mut it)?;
                details.subcategory = decode(&mut it)?;
                details.timezone_id = decode(&mut it)?;
                details.trading_hours = decode(&mut it)?;
                details.liquid_hours = decode(&mut it)?;
                details.ev_rule = decode(&mut it)?;
                details.ev_multiplier = decode(&mut it)?;
                let sec_id_list_count: Option<usize> = decode(&mut it)?;
                details.sec_id_list = match sec_id_list_count {
                    Some(count) => {
                        let mut sec_ids: Vec<(String, String)> = Vec::with_capacity(count);
                        for _i in 0..count {
                            sec_ids.push((decode(&mut it)?.unwrap(), decode(&mut it)?.unwrap()));
                        }
                        Some(sec_ids)
                    },
                    None => None,
                };
                details.agg_group = decode(&mut it)?;
                details.under_symbol = decode(&mut it)?;
                details.under_sec_type = decode(&mut it)?;
                details.market_rule_ids = decode(&mut it)?;
                details.real_expiration_date = decode(&mut it)?;
                details.contract = contract;
                details.stock_type = decode(&mut it)?;
                Ok(IBFrame::ContractDetails {
                    req_id,
                    contract_details: details,
                })
            },
            Incoming::ContractDataEnd => {
                it.next(); // skip version
                Ok(IBFrame::ContractDetailsEnd(decode(&mut it)?.unwrap()))
            },
            Incoming::NextValidId => {
                it.next(); // skip version
                Ok(IBFrame::OrderId(decode(&mut it)?.unwrap()))
            },
            Incoming::OpenOrderEnd => {
                tracing::debug!("open order end!");
                it.next(); // skip version
                Ok(IBFrame::OpenOrderEnd)
            },
            Incoming::OpenOrder | Incoming::CompletedOrder => {
                let order_information = OrderInformation::try_parse_frame(msg_id, &mut it)?;
                match msg_id {
                    Incoming::OpenOrder => Ok(IBFrame::OpenOrder(order_information)),
                    Incoming::CompletedOrder => Ok(IBFrame::CompletedOrder(order_information)),
                    _ => Err(ParseError::UnexpectedMessage),
                }
            },

            Incoming::CommissionReport => {
                Ok(IBFrame::CommissionReport(
                    CommissionReport::try_parse_frame(msg_id, &mut it)?,
                ))
            },

            Incoming::ExecutionData => {
                Ok(IBFrame::Execution(Execution::try_parse_frame(
                    msg_id, &mut it,
                )?))
            },

            Incoming::OrderStatus => {
                Ok(IBFrame::OrderStatus(OrderStatusUpdate::try_parse_frame(
                    msg_id, &mut it,
                )?))
            },

            Incoming::RealTimeBars => {
                Ok(IBFrame::RealtimeBar(RealtimeBar::try_parse_frame(
                    msg_id, &mut it,
                )?))
            },

            Incoming::TickPrice => {
                Ok(IBFrame::Tick(
                    TickPrice::try_parse_frame(msg_id, &mut it)?.into(),
                ))
            },

            Incoming::TickSize => {
                Ok(IBFrame::Tick(
                    TickSize::try_parse_frame(msg_id, &mut it)?.into(),
                ))
            },

            Incoming::TickString => {
                Ok(IBFrame::Tick(
                    TickString::try_parse_frame(msg_id, &mut it)?.into(),
                ))
            },

            Incoming::TickGeneric => {
                Ok(IBFrame::Tick(
                    TickGeneric::try_parse_frame(msg_id, &mut it)?.into(),
                ))
            },

            Incoming::TickByTick => Ok(IBFrame::Tick(Tick::try_parse_frame(msg_id, &mut it)?)),

            Incoming::HistoricalData => {
                Ok(IBFrame::HistoricalBars(HistoricalBars::try_parse_frame(
                    msg_id, &mut it,
                )?))
            },

            Incoming::HistoricalDataUpdate => {
                Ok(IBFrame::HistoricalBars(HistoricalBars::try_parse_frame(
                    msg_id, &mut it,
                )?))
            },

            Incoming::HistoricalSchedule => {
                Ok(HistoricalSchedule::try_parse_frame(msg_id, &mut it)?.into())
            },

            Incoming::HeadTimestamp => {
                Ok(IBFrame::HeadTimestamp(HeadTimestamp {
                    id:        decode(&mut it)?.unwrap(),
                    timestamp: decode(&mut it)?.unwrap(),
                }))
            },

            Incoming::HistoricalTicks
            | Incoming::HistoricalTicksBidAsk
            | Incoming::HistoricalTicksLast => {
                Ok(IBFrame::HistoricalTicks(HistoricalTicks::try_parse_frame(
                    msg_id, &mut it,
                )?))
            },

            Incoming::ErrMsg => {
                it.next(); // skip version
                Ok(IBFrame::Error {
                    req_id:  decode(&mut it)?.unwrap(),
                    status:  decode(&mut it)?.unwrap(),
                    message: decode(&mut it)?,
                })
            },
            _ => Ok(IBFrame::NotImplemented),
        }
    }

    // TODO resolve unwrap() !!!
    // TODO use DateTime instead of String
    pub fn parse_server_version(msg: &mut Cursor<&[u8]>) -> ParseResult<IBFrame> {
        let msg = read(msg)?;
        let utf8msg = String::from_utf8_lossy(msg);
        let mut it = utf8msg.split('\0');
        let server_version = it.next().unwrap().parse().unwrap();
        let connection_time = it.next().unwrap().parse().unwrap();
        Ok(IBFrame::ServerVersion {
            server_version,
            connection_time,
        })
    }
}

fn read<'a>(src: &mut Cursor<&'a [u8]>) -> ParseResult<&'a [u8]> {
    // Scan the bytes directly
    let start = src.position() as usize;
    // Scan to the second to last byte
    let end = src.get_ref().len() - 1;
    if (end - start) < 3 {
        return Err(ParseError::Incomplete);
    }

    let headbuf: [u8; 4] = src.get_ref()[start..start + 4]
        .try_into()
        .expect("Incomplete");
    let msg_size = u32::from_be_bytes(headbuf) as usize;
    src.set_position((start + 4 + msg_size) as u64);
    Ok(&src.get_ref()[start + 4..start + 4 + msg_size])
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    #[test]
    #[ignore]
    fn parse_open_order() {
        let string_to_parse =
            "\u{0}\u{0}\u{2}\u{7f}5\u{0}-38\u{0}10291\u{0}NKE\u{0}STK\u{0}\u{0}0\u{0}?\u{0}\\
             u{0}SMART\u{0}USD\u{0}NKE\u{0}NKE\u{0}SELL\u{0}5\u{0}STP\u{0}0.0\u{0}142.5\u{0}GTC\\
             u{0}613177344\u{0}U6469752\u{0}\u{0}0\u{0}\u{0}0\u{0}613177345\u{0}0\u{0}0\u{0}0\\
             u{0}\u{0}613177345.1/U6469752/100\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}0\u{0}\u{0}\\
             u{0}0\u{0}\u{0}-1\u{0}0\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}2147483647\u{0}0\u{0}0\u{0}0\\
             u{0}\u{0}3\u{0}0\u{0}0\u{0}\u{0}0\u{0}0\u{0}\u{0}0\u{0}None\u{0}\u{0}0\u{0}\u{0}\\
             u{0}\u{0}?\u{0}0\u{0}0\u{0}\u{0}0\u{0}0\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}0\u{0}0\u{0}0\\
             u{0}2147483647\u{0}2147483647\u{0}\u{0}\u{0}0\u{0}\u{0}IB\u{0}0\u{0}0\u{0}\u{0}0\\
             \
             u{0}0\u{0}PreSubmitted\u{0}1.7976931348623157E308\u{0}1.7976931348623157E308\u{0}1.\
             7976931348623157E308\u{0}1.7976931348623157E308\u{0}1.7976931348623157E308\u{0}1.\
             7976931348623157E308\u{0}1.7976931348623157E308\u{0}1.7976931348623157E308\u{0}1.\
             7976931348623157E308\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}0\u{0}0\u{0}0\u{0}None\u{0}1.\
             7976931348623157E308\u{0}142.5\u{0}1.7976931348623157E308\u{0}1.7976931348623157E308\\
             \
             u{0}1.7976931348623157E308\u{0}1.7976931348623157E308\u{0}0\u{0}\u{0}\u{0}\u{0}0\\
             u{0}1\u{0}0\u{0}0\u{0}0\u{0}\u{0}\u{0}0\u{0}"
                .as_bytes();
        tracing::warn!("{:?}", String::from_utf8_lossy(&[0, 0, 2, 120]));
        let mut buff = Cursor::new(string_to_parse);
        let open_position = IBFrame::parse(&mut buff);
        tracing::warn!("parse open position: {:#?}", open_position);
    }
    #[test]
    #[ignore]
    fn parse_open_order_simu() {
        let string_to_parse =
            "\u{0}\u{0}\u{2}x5\u{0}0\u{0}10291\u{0}NKE\u{0}STK\u{0}\u{0}0\u{0}?\u{0}\u{0}SMART\\
             u{0}USD\u{0}NKE\u{0}NKE\u{0}SELL\u{0}10\u{0}STP\u{0}0.0\u{0}152.5\u{0}DAY\u{0}\\
             u{0}DU3293378\u{0}\u{0}0\u{0}\u{0}0\u{0}1716317258\u{0}0\u{0}0\u{0}0\u{0}\\
             u{0}1716317258.0/DU3293378/100\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}0\\
             u{0}\u{0}-1\u{0}0\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}2147483647\u{0}0\u{0}0\u{0}0\u{0}\\
             u{0}3\u{0}0\u{0}0\u{0}\u{0}0\u{0}0\u{0}\u{0}0\u{0}None\u{0}\u{0}0\u{0}\u{0}\u{0}\\
             u{0}?\u{0}0\u{0}0\u{0}\u{0}0\u{0}0\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}0\u{0}0\u{0}0\\
             u{0}2147483647\u{0}2147483647\u{0}\u{0}\u{0}0\u{0}\u{0}IB\u{0}0\u{0}0\u{0}\u{0}0\\
             \
             u{0}0\u{0}PreSubmitted\u{0}1.7976931348623157E308\u{0}1.7976931348623157E308\u{0}1.\
             7976931348623157E308\u{0}1.7976931348623157E308\u{0}1.7976931348623157E308\u{0}1.\
             7976931348623157E308\u{0}1.7976931348623157E308\u{0}1.7976931348623157E308\u{0}1.\
             7976931348623157E308\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}0\u{0}0\u{0}0\u{0}None\u{0}1.\
             7976931348623157E308\u{0}152.5\u{0}1.7976931348623157E308\u{0}1.7976931348623157E308\\
             \
             u{0}1.7976931348623157E308\u{0}1.7976931348623157E308\u{0}0\u{0}\u{0}\u{0}\u{0}0\\
             u{0}1\u{0}0\u{0}0\u{0}0\u{0}\u{0}\u{0}0\u{0}"
                .as_bytes();
        tracing::warn!("{:?}", String::from_utf8_lossy(&[0, 0, 2, 127]));
        let mut buff = Cursor::new(string_to_parse);
        let open_position = IBFrame::parse(&mut buff);
        tracing::warn!("parse open position: {:#?}", open_position);
    }
}
