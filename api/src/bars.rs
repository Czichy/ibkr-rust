use std::{convert::TryFrom,
          fmt::{Display, Formatter},
          str::{FromStr, Split}};

use chrono::{NaiveDate, Utc};
use chrono_tz::Tz;

use crate::{ib_frame::{ParseError, ParseIbkrFrame, ParseResult},
            prelude::{dateparser::Parse,
                      ib_message::{decode, Decodable, Encodable},
                      Incoming,
                      ParseEnumError},
            MarketDataValueType,
            RequestId,
            ServerVersion,
            TimeStamp};

#[derive(Debug, Clone, Copy)]
pub struct Bar {
    pub t_stamp: TimeStamp,
    pub open:    MarketDataValueType,
    pub high:    MarketDataValueType,
    pub low:     MarketDataValueType,
    pub close:   MarketDataValueType,
    pub wap:     MarketDataValueType,
    pub volume:  MarketDataValueType,
    pub count:   isize,
}

#[derive(Debug, Clone)]
pub struct BarSeries {
    pub start_dt: TimeStamp,
    pub end_dt:   TimeStamp,
    pub n_bars:   usize,
    pub bars:     Vec<Bar>,
}

#[derive(Debug, Clone)]
pub struct HistoricalSchedule {
    pub id:              RequestId,
    pub start_date_time: TimeStamp,
    pub end_date_time:   TimeStamp,
    pub time_zone:       Tz,
    pub sessions:        Vec<HistoricalSession>,
}
impl ParseIbkrFrame for HistoricalSchedule {
    fn try_parse_frame(
        msg_id: Incoming,
        server_version: Option<ServerVersion>,
        it: &mut Split<&str>,
    ) -> ParseResult<Self>
    where
        Self: Sized,
    {
        if !matches!(msg_id, Incoming::HistoricalSchedule) {
            return Err(ParseError::UnexpectedMessage);
        }
        let id = decode(it)?.unwrap();
        let start: String = decode(it)?.unwrap();
        let end: String = decode(it)?.unwrap();
        let tz: String = decode(it)?.unwrap();
        let time_zone =
            Tz::from_str(tz.trim()).map_err(|_| ParseError::UnexpectedVariant(tz.clone()))?;
        let n_sessions = decode(it)?.unwrap();

        let parse = Parse::new(&Utc, Utc::now().time());

        let start_date_time = parse
            .parse(&format!("{start} {tz}"))
            .map_err(|_| ParseError::UnexpectedVariant(tz.clone()))?;
        let end_date_time = parse
            .parse(&format!("{end} {tz}"))
            .map_err(|_| ParseError::UnexpectedVariant(tz.clone()))?;
        let mut sessions = Vec::with_capacity(n_sessions);
        for _i in 0..n_sessions {
            let start_session: String = decode(it)?.unwrap();
            let end_session: String = decode(it)?.unwrap();
            tracing::log::error!("start: {start_session:#?}");
            let start_date_time_session = parse
                .parse(&format!("{start_session} {tz}"))
                .map_err(|_| ParseError::UnexpectedVariant(tz.clone()))?;
            let end_date_time_session = parse
                .parse(&format!("{end_session} {tz}"))
                .map_err(|_| ParseError::UnexpectedVariant(tz.clone()))?;
            let session = HistoricalSession {
                start_date_time: start_date_time_session,
                end_date_time:   end_date_time_session,
                ref_date:        decode(it)?.unwrap(),
            };
            sessions.push(session);
        }
        Ok(Self {
            id,
            start_date_time,
            end_date_time,
            time_zone,
            sessions,
        })
    }
}

#[derive(Debug, Clone, Copy)]
pub struct HistoricalSession {
    pub start_date_time: TimeStamp,
    pub end_date_time:   TimeStamp,
    pub ref_date:        NaiveDate,
}
impl ParseIbkrFrame for HistoricalSession {
    fn try_parse_frame(
        msg_id: Incoming,
        server_version: Option<ServerVersion>,
        it: &mut Split<&str>,
    ) -> ParseResult<Self>
    where
        Self: Sized,
    {
        if !matches!(msg_id, Incoming::HistoricalSchedule) {
            return Err(ParseError::UnexpectedMessage);
        }
        Ok(Self {
            start_date_time: decode(it)?.unwrap(),
            end_date_time:   decode(it)?.unwrap(),
            ref_date:        decode(it)?.unwrap(),
        })
    }
}
#[derive(Debug, Clone, Copy)]
pub struct RealtimeBar {
    pub id:   RequestId,
    pub data: Bar,
}
impl ParseIbkrFrame for RealtimeBar {
    fn try_parse_frame(
        msg_id: Incoming,
        server_version: Option<ServerVersion>,
        it: &mut Split<&str>,
    ) -> ParseResult<Self>
    where
        Self: Sized,
    {
        if !matches!(msg_id, Incoming::RealTimeBars) {
            return Err(ParseError::UnexpectedMessage);
        }
        it.next(); // skip version
        let id = decode(it)?.unwrap();
        Ok(Self {
            id,
            data: Bar {
                t_stamp: decode(it)?.unwrap(),
                open:    decode(it)?.unwrap(),
                high:    decode(it)?.unwrap(),
                low:     decode(it)?.unwrap(),
                close:   decode(it)?.unwrap(),
                volume:  decode(it)?.unwrap(),
                wap:     decode(it)?.unwrap(),
                count:   decode(it)?.unwrap(),
            },
        })
    }
}

#[derive(Debug, Clone)]
pub struct HistoricalBars {
    pub id:   RequestId,
    pub data: BarSeries,
}
impl ParseIbkrFrame for HistoricalBars {
    fn try_parse_frame(
        msg_id: Incoming,
        server_version: Option<ServerVersion>,
        it: &mut Split<&str>,
    ) -> ParseResult<Self>
    where
        Self: Sized,
    {
        match msg_id {
            Incoming::HistoricalDataUpdate | Incoming::HistoricalData => {
                let id = decode(it)?.unwrap();
                let start_dt = decode(it)?.unwrap();
                let end_dt = decode(it)?.unwrap();
                let n_bars = decode(it)?.unwrap();
                let data = {
                    let mut bar_data = Vec::with_capacity(n_bars);
                    for _i in 0..n_bars {
                        bar_data.push(Bar {
                            t_stamp: decode(it)?.unwrap(),
                            open:    decode(it)?.unwrap(),
                            high:    decode(it)?.unwrap(),
                            low:     decode(it)?.unwrap(),
                            close:   decode(it)?.unwrap(),
                            volume:  decode(it)?.unwrap(),
                            wap:     decode(it)?.unwrap(),
                            count:   decode(it)?.unwrap(),
                        });
                    }
                    bar_data
                };
                Ok(Self {
                    id,
                    data: BarSeries {
                        start_dt,
                        end_dt,
                        n_bars,
                        bars: data,
                    },
                })
            },
            _ => Err(ParseError::UnexpectedMessage),
        }
    }
}
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum BarSize {
    _1Secs,
    _5Secs,
    _10Secs,
    _15Secs,
    _30Secs,
    _1Min,
    _2Mins,
    _3Mins,
    _5Mins,
    _10Mins,
    _15Mins,
    _20Mins,
    _30Mins,
    _1Hour,
    _4Hours,
    _1Day,
    _1Week,
    _1Month,
}

impl Encodable for BarSize {
    fn encode(&self) -> String {
        match self {
            BarSize::_1Secs => "1 secs\0",
            BarSize::_5Secs => "5 secs\0",
            BarSize::_10Secs => "10 secs\0",
            BarSize::_15Secs => "15 secs\0",
            BarSize::_30Secs => "30 secs\0",
            BarSize::_1Min => "1 min\0",
            BarSize::_2Mins => "2 mins\0",
            BarSize::_3Mins => "3 mins\0",
            BarSize::_5Mins => "5 mins\0",
            BarSize::_10Mins => "10 mins\0",
            BarSize::_15Mins => "15 mins\0",
            BarSize::_20Mins => "20 mins\0",
            BarSize::_30Mins => "30 mins\0",
            BarSize::_1Hour => "1 hour\0",
            BarSize::_4Hours => "4 hours\0",
            BarSize::_1Day => "1 day\0",
            BarSize::_1Week => "1 week\0",
            BarSize::_1Month => "1 month\0",
        }
        .to_string()
    }
}
impl Default for BarSize {
    fn default() -> Self { Self::_1Min }
}

impl FromStr for BarSize {
    type Err = ParseEnumError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let res = match s {
            "1 secs" => BarSize::_1Secs,
            "5 secs" => BarSize::_5Secs,
            "10 secs" => BarSize::_10Secs,
            "15 secs" => BarSize::_15Secs,
            "30 secs" => BarSize::_30Secs,
            "1 min" => BarSize::_1Min,
            "2 mins" => BarSize::_2Mins,
            "3 mins" => BarSize::_3Mins,
            "5 mins" => BarSize::_5Mins,
            "10 mins" => BarSize::_10Mins,
            "15 mins" => BarSize::_15Mins,
            "20 mins" => BarSize::_20Mins,
            "30 mins" => BarSize::_30Mins,
            "1 hour" => BarSize::_1Hour,
            "4 hours" => BarSize::_4Hours,
            "1 day" => BarSize::_1Day,
            "1 week" => BarSize::_1Week,
            "1 month" => BarSize::_1Month,
            &_ => return Err(ParseEnumError),
        };
        Ok(res)
    }
}

impl Display for BarSize {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result { write!(f, "{self:?}") }
}
impl Decodable for BarSize {}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Duration {
    Seconds(u32),
    Day(u32),
    Week(u32),
    Month(u32),
    Year(u32),
}

impl Encodable for Duration {
    fn encode(&self) -> String {
        match self {
            Duration::Seconds(value) => format!("{} {}\0", value, 'S'),
            Duration::Day(value) => format!("{} {}\0", value, 'D'),
            Duration::Week(value) => format!("{} {}\0", value, 'W'),
            Duration::Month(value) => format!("{} {}\0", value, 'M'),
            Duration::Year(value) => format!("{} {}\0", value, 'Y'),
        }
    }
}
impl Default for Duration {
    fn default() -> Self { Self::Day(1) }
}

impl TryFrom<std::time::Duration> for Duration {
    type Error = ParseEnumError;

    fn try_from(value: std::time::Duration) -> Result<Self, Self::Error> {
        match value.as_secs() {
            d if d > 365 * 24 * 60 * 60 => {
                u32::try_from(d / (365 * 24 * 60 * 60))
                    .map_or(Err(ParseEnumError), |years| Ok(Duration::Year(years)))
            },
            d if d > 24 * 60 * 60 && d <= 365 * 24 * 60 * 60 => {
                u32::try_from(d).map_or(Err(ParseEnumError), |days| {
                    Ok(Duration::Day(days / (24 * 60 * 60)))
                })
            },
            d => {
                u32::try_from(d).map_or(Err(ParseEnumError), |seconds| {
                    if seconds > 0 {
                        Ok(Duration::Seconds(seconds))
                    } else {
                        Err(ParseEnumError)
                    }
                })
            },
        }
    }
}

impl FromStr for Duration {
    type Err = ParseEnumError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s.split_whitespace();
        // if s.len() != 2 {
        //     Err(ParseEnumError)
        // } else {
        // let res = if let Some(value) =
        s.next()
            .and_then(|v| v.parse::<u32>().ok())
            .and_then(|value| {
                s.next().and_then(|d| d.chars().next()).map(|d| {
                    match d {
                        'S' => Ok(Duration::Seconds(value)),
                        'D' => Ok(Duration::Day(value)),
                        'W' => Ok(Duration::Week(value)),
                        'M' => Ok(Duration::Month(value)),
                        'Y' => Ok(Duration::Year(value)),
                        _ => Err(ParseEnumError),
                    }
                })
            })
            .unwrap_or(Err(ParseEnumError))
    }
}

impl Display for Duration {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result { write!(f, "{self:?}") }
}
impl Decodable for Duration {}
