use std::{fmt::{Display, Formatter},
          str::FromStr};

use bytes::Bytes;
use chrono::{DateTime, Local};

use super::IntoIbkrFrame;
use crate::{contract::Contract,
            enums::{MarketDataType, Outgoing},
            frame::Frame,
            prelude::{ib_message::Decodable, BarSize, Duration, ParseEnumError},
            ticker::{GenericTickType, TickByTickType},
            utils::ib_message::{Encodable, IBMessage},
            RequestId,
            TimeStamp};

/// Call this function to request market data. The market data
/// will be returned by the tick_price and tick_size wrapper events.
///
/// # Arguments
/// * req_id - The request id. Must be a unique value. When the market data
///   returns, it will be identified by this tag. This is also used when
///   canceling the market data.
/// * contract - This structure contains a description of the Contract for which
///   market data is being requested.
/// * generic_tick_list - A commma delimited list of generic tick types. Tick
///   types can be found in the Generic Tick Types page. Prefixing w/ 'mdoff'
///   indicates that top mkt data shouldn't tick. You can specify the news
///   source by postfixing w/ ':<source>. Example: "mdoff, 292: FLY + BRF"
/// * snapshot - Check to return a single snapshot of Market data and have the
///   market data subscription cancel. Do not enter any generic_tick_list values
///   if you use snapshots.
/// * regulatory_snapshot - With the US Value Snapshot Bundle for stocks,
///   regulatory snapshots are available for 0.01 USD each.
/// * mkt_data_options - For internal use only. Use default value XYZ.
#[derive(Debug)]
pub struct MarketDataRequest {
    pub req_id:            RequestId,
    pub contract:          Contract,
    pub generic_tick_list: Vec<GenericTickType>,
    pub snapshot:          bool,
    pub regulatory:        bool,
    pub additional_data:   Vec<TagValue>,
}

impl IntoIbkrFrame for MarketDataRequest {
    fn into_frame(&self) -> Frame {
        let version: i32 = 11;
        let mut msg = Outgoing::ReqMktData.encode();
        msg.push_str(&version.encode()); // version
        msg.push_str(&self.req_id.encode());
        msg.push_str(&self.contract.encode_for_ticker());
        msg.push_str("0\0");
        for i in &self.generic_tick_list {
            msg.push_str(&i.encode());
            msg.push(',');
        }
        if let Some(gen_tick) = self.generic_tick_list.last() {
            msg.push_str(&gen_tick.encode());
        }
        // TODO: generic tick list
        msg.push('\0'); // generic tick data
        msg.push_str(&self.snapshot.encode());
        msg.push_str(&self.regulatory.encode());
        // TODO: MarketData Options
        // current doc says this part if for "internal use only" -> won't support it
        msg.push('\0');
        let msg = msg.as_str().to_ib_message().unwrap();
        Frame::Bulk(Bytes::from(msg))
    }
}
/// Cancel the request
///
/// # Arguments
/// * req_id - the id of the original request
#[derive(Debug, Clone, Copy)]
pub struct CancelHeadTimestampRequest {
    pub req_id: RequestId,
}
impl IntoIbkrFrame for CancelHeadTimestampRequest {
    fn into_frame(&self) -> Frame {
        let mut msg = Outgoing::CancelHeadTimestamp.encode();
        msg.push_str(&self.req_id.encode());
        let msg = msg.as_str().to_ib_message().unwrap();
        Frame::Bulk(Bytes::from(msg))
    }
}
/// Used if an internet disconnect has occurred or the results of a query
/// are otherwise delayed and the client is no longer interested in receiving
/// the data.
///
/// # Arguments
/// * req_id - the id of the original request
#[derive(Debug, Clone, Copy)]
pub struct CancelHistoricalDataRequest {
    pub req_id: RequestId,
}
impl IntoIbkrFrame for CancelHistoricalDataRequest {
    fn into_frame(&self) -> Frame {
        let mut msg = Outgoing::CancelHistoricalData.encode();
        msg.push_str(&self.req_id.encode());
        let msg = msg.as_str().to_ib_message().unwrap();
        Frame::Bulk(Bytes::from(msg))
    }
}
/// After calling this function, market depth data for the specified id
/// will stop flowing.
///
/// # Arguments
/// * req_id - The ID that was specified in the call to req_mkt_depth().
//  * is_smart_depth - specifies SMART depth request
#[derive(Debug, Clone, Copy)]
pub struct CancelMarketDepthRequest {
    pub req_id:         RequestId,
    pub is_smart_depth: bool,
}
impl IntoIbkrFrame for CancelMarketDepthRequest {
    fn into_frame(&self) -> Frame {
        let mut msg = Outgoing::CancelMktDepth.encode();
        msg.push_str(&self.req_id.encode());
        let msg = msg.as_str().to_ib_message().unwrap();
        Frame::Bulk(Bytes::from(msg))
    }
}
/// After calling this function, market data for the specified id will stop
/// flowing.
///
/// # Arguments
/// * req_id - The ID that was specified in the call to req_mkt_data()
#[derive(Debug, Clone, Copy)]
pub struct CancelMarketDataRequest {
    pub req_id: RequestId,
}
impl IntoIbkrFrame for CancelMarketDataRequest {
    fn into_frame(&self) -> Frame {
        let mut msg = Outgoing::CancelMktData.encode();
        msg.push_str(&self.req_id.encode());
        let msg = msg.as_str().to_ib_message().unwrap();
        Frame::Bulk(Bytes::from(msg))
    }
}
/// Cancel tick by tick data
///
/// # Arguments
/// * req_id    - The identifier of the original request.
#[derive(Debug, Clone, Copy)]
pub struct CancelTickByTickRequest {
    pub req_id: RequestId,
}
impl IntoIbkrFrame for CancelTickByTickRequest {
    fn into_frame(&self) -> Frame {
        let mut msg = Outgoing::CancelTickByTickData.encode();
        msg.push_str(&self.req_id.encode());
        let msg = msg.as_str().to_ib_message().unwrap();
        Frame::Bulk(Bytes::from(msg))
    }
}
// /// Returns the timestamp of earliest available historical data for a
// contract and data type. ///
// /// # Arguments
// /// * req_id    - an identifier for the request
// /// * contract    - contract object for which head timestamp is being
// requested /// * what_to_show    - type of data for head timestamp - "BID",
// "ASK", "TRADES", etc /// * use_rth    - use regular trading hours only, 1 for
// yes or 0 for no /// * format_date    set to 1 to obtain the bars' time as
// yyyyMMdd HH:mm:ss, set to 2 to obtain it like system time format in seconds
// // / /// Note that formatData parameter affects intraday bars only
// /// 1-day bars always return with date in YYYYMMDD format
#[derive(Debug)]
pub struct HeadTimestampRequest {
    pub req_id:       RequestId,
    pub contract:     Contract,
    pub what_to_show: HistoricalDataType,
    pub use_rth:      UseRegularTradingHoursOnly,
    pub format_date:  IntradayBarDateFormat,
}
impl IntoIbkrFrame for HeadTimestampRequest {
    fn into_frame(&self) -> Frame {
        // let version: i32 = 6;
        let mut msg = Outgoing::ReqHeadTimestamp.encode();
        // msg.push_str(&version.encode()); // version
        msg.push_str(&self.req_id.encode());
        msg.push_str(&self.contract.encode_for_ticker());
        msg.push_str(&self.contract.include_expired.encode());

        msg.push_str(&self.use_rth.encode());
        msg.push_str(&self.what_to_show.encode());
        msg.push_str(&self.format_date.encode());
        let msg = msg.as_str().to_ib_message().unwrap();
        Frame::Bulk(Bytes::from(msg))
    }
}
/// Requests contracts' historical data. When requesting historical data, a
/// finishing time and date is required along with a duration string. The
/// resulting bars will be returned in EWrapper.historicalData()
///
/// # Arguments
/// * req_id - The id of the request. Must be a unique value. When the market
///   data returns, it whatToShowill be identified by this tag. This is also
///   used when canceling the market data.
/// * contract - This object contains a description of the contract for which
///   market data is being requested.
/// * end_date_time - Defines a query end date and time at any point during the
///   past 6 mos. Valid values include any date/time within the past six months
///   in the format: yyyymmdd HH:mm:ss ttt where "ttt" is the optional time
///   zone.
/// * duration_str - Set the query duration up to one week, using a time unit of
///   seconds, days or weeks. Valid values include any integer followed by a
///   space and then S (seconds)); D (days) or W (week). If no unit is
///   specified, seconds is used.
/// * bar_size_setting - See the BarSize enum for valid values. Specifies the
///   size of the bars that will be returned (within IB/TWS listimits). Valid
///   values include:
///     * 1 sec
///     * 5 secs
///     * 15 secs
///     * 30 secs
///     * 1 min
///     * 2 mins
///     * 3 mins
///     * 5 mins
///     * 15 mins
///     * 30 mins
///     * 1 hour
///     * 1 day
/// * what_to_show - See the WhatToShow enum for valid values.  Determines the
///   nature of data beinging extracted. Valid values include:
///
///     * TRADES
///     * MIDPOINT
///     * BID
///     * ASK
///     * BID_ASK
///     * HISTORICAL_VOLATILITY
///     * OPTION_IMPLIED_VOLATILITY
/// * use_rth - Determines whether to return all data available during the
///   requested time span, or only data that falls within regular trading hours.
///   Valid values include:
///
///     * 0 - all data is returned even where the market in question was outside
///       of its regular trading hours.
///     * 1 - only data within the regular trading hours is returned, even if
///       the requested time span falls partially or completely outside of the
///       RTH.
/// * format_date - Determines the date format applied to returned bars. validd
///   values include:
///
///     * 1 - dates applying to bars returned in the format:
///       yyyymmdd{space}{space}hh:mm:dd
///     * 2 - dates are returned as a long integer specifying the number of
///       seconds since 1/1/1970 GMT.
/// *chart_options: - For internal use only. Use default value XYZ.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HistoricalDataRequest {
    pub req_id:           RequestId,
    pub contract:         Contract,
    pub end_date_time:    TimeStamp,
    pub duration:         Duration,
    pub bar_size_setting: BarSize,
    pub what_to_show:     HistoricalDataType,
    pub use_rth:          UseRegularTradingHoursOnly,
    pub format_date:      IntradayBarDateFormat,
    pub keep_up_to_date:  bool,
    pub chart_options:    Vec<TagValue>,
}
impl IntoIbkrFrame for HistoricalDataRequest {
    fn into_frame(&self) -> Frame {
        // let version: i32 = 6;
        let mut msg = Outgoing::ReqHistoricalData.encode();
        // msg.push_str(&version.encode()); // version
        msg.push_str(&self.req_id.encode());
        msg.push_str(&self.contract.encode_for_ticker());
        msg.push_str(&self.contract.include_expired.encode());

        // let local: DateTime<Local> = DateTime::from(self.end_date_time);
        // tracing::debug!("{}", &local.format("%Y%m%d-%H:%M:%S").to_string());
        msg.push_str(
            &self
                .end_date_time
                .format("%Y%m%d-%H:%M:%S")
                .to_string()
                .encode(),
        );
        msg.push_str(&self.bar_size_setting.encode());
        msg.push_str(&self.duration.encode());
        msg.push_str(&self.use_rth.encode());
        msg.push_str(&self.what_to_show.encode());
        msg.push_str(&self.format_date.encode());
        msg.push_str(&self.keep_up_to_date.encode());
        let chart_options_str = self
            .chart_options
            .iter()
            .map(|x| format!("{}={};", x.tag, x.value))
            .collect::<String>();
        msg.push_str(&chart_options_str.encode());
        msg.push('\0');
        let msg = msg.as_str().to_ib_message().unwrap();
        Frame::Bulk(Bytes::from(msg))
    }
}
/// Requests historical Time&Sales data for an instrument.
///
/// # Arguments
/// * req_id - id of the request
/// * contract - Contract object that is subject of query
/// * start_date_time,i.e.    "20170701 12:01:00". Uses TWS timezone specified
///   at login.
/// * end_date_time,i.e.    "20170701 13:01:00". In TWS timezone. Exactly one of
///   start time and end time has to be defined.
/// * number_of_ticks - Number of distinct data points. Max currently 1000 per
///   request.
/// * what_to_show - (Bid_Ask, Midpoint, Trades) Type of data requested.
/// * use_rth - Data from regular trading hours (1), or all available hours (0)
/// * ignore_size - A filter only used when the source price is Bid_Ask
/// * misc_options - should be defined as null, reserved for internal use
#[derive(Debug)]
pub struct HistoricalTicksRequest {
    pub req_id:          RequestId,
    pub contract:        Contract,
    pub date_time:       HistoricalTickDateTime,
    pub number_of_ticks: i32,
    pub what_to_show:    HistoricalDataType,
    pub use_rth:         UseRegularTradingHoursOnly,
    pub ignore_size:     i32,
    pub misc_options:    Vec<TagValue>,
}
impl IntoIbkrFrame for HistoricalTicksRequest {
    fn into_frame(&self) -> Frame {
        // let version: i32 = 6;
        let mut msg = Outgoing::ReqHistoricalTicks.encode();
        // msg.push_str(&version.encode()); // version
        msg.push_str(&self.req_id.encode());
        msg.push_str(&self.contract.encode_for_ticker());
        msg.push_str(&self.contract.include_expired.encode());
        match self.date_time {
            HistoricalTickDateTime::Start(start_date_time) => {
                let local: DateTime<Local> = DateTime::from(start_date_time);
                tracing::debug!("{}", &local.format("%Y%m%d-%H:%M:%S").to_string());
                msg.push_str(
                    &start_date_time
                        .format("%Y%m%d-%H:%M:%S")
                        .to_string()
                        .encode(),
                );
                msg.push('\0'); // end_date_time
            },
            HistoricalTickDateTime::End(end_date_time) => {
                msg.push('\0'); // start_date_time
                let local: DateTime<Local> = DateTime::from(end_date_time);
                tracing::debug!("{}", &local.format("%Y%m%d-%H:%M:%S").to_string());
                msg.push_str(&end_date_time.format("%Y%m%d-%H:%M:%S").to_string().encode());
            },
        };
        msg.push_str(&self.number_of_ticks.encode());
        msg.push_str(&self.what_to_show.encode());
        msg.push_str(&self.use_rth.encode());
        msg.push_str(&self.ignore_size.encode());
        let misc_options_str = self
            .misc_options
            .iter()
            .map(|x| format!("{}={};", x.tag, x.value))
            .collect::<String>();
        msg.push_str(&misc_options_str.encode());
        msg.push('\0');
        let msg = msg.as_str().to_ib_message().unwrap();
        Frame::Bulk(Bytes::from(msg))
    }
}
/// The API can receive frozen market data from Trader
/// Workstation. Frozen market data is the last data recorded in our system.
/// During normal trading hours, the API receives real-time market data. If
/// you use this function, you are telling TWS to automatically switch to
/// frozen market data after the close. Then, before the opening of the next
/// trading day, market data will automatically switch back to real-time
/// market data.
///
/// # Arguments
/// * market_data_type
/// * 1 for real-time streaming market data
/// * 2 for frozen market data
#[derive(Debug, Clone, Copy)]
pub struct MarketDataTypeRequest {
    pub market_data_type: MarketDataType,
}
impl IntoIbkrFrame for MarketDataTypeRequest {
    fn into_frame(&self) -> Frame {
        let version: i32 = 1;
        let mut msg = Outgoing::ReqMarketDataType.encode();
        msg.push_str(&version.encode());
        msg.push_str(&self.market_data_type.encode());
        let msg = msg.as_str().to_ib_message().unwrap();
        Frame::Bulk(Bytes::from(msg))
    }
}
/// Call this function to request market depth for a specific
/// contract. The market depth will be returned by the update_mkt_depth()
/// and update_mkt_depth_l2() events.
///
/// Requests the contract's market depth (order book). Note this request
/// must be direct-routed to an exchange and not smart-routed. The
/// number of simultaneous market depth requests allowed in an account
/// is calculated based on a formula that looks at an accounts equity,
/// commissions, and quote booster packs.
///
/// # Arguments
/// * req_id - The request id. Must be a unique value. When the market depth
///   data returns, it will be identified by this tag. This is also used when
///   canceling the market depth
/// * contract - This structure contains a description of the contract for which
///   market depth data is being requested.
/// * num_rows - Specifies the number of rows of market depth rows to display.
/// * is_smart_depth - specifies SMART depth request  NOTE: ALWAYS SET TO
///   FALSE!!!!! THERE SEEMS TO BE A BUG ON IB's SIDE AND THEY WILL STOP
///   STREAMING DATA IF THIS IS SET TO TRUE
/// * mkt_depth_options - For internal use only. Use default value XYZ.

#[derive(Debug)]
pub struct MarketDepthRequest {
    pub req_id:            RequestId,
    pub contract:          Contract,
    pub num_rows:          i32,
    pub is_smart_depth:    bool,
    pub mkt_depth_options: Vec<TagValue>,
}

impl IntoIbkrFrame for MarketDepthRequest {
    fn into_frame(&self) -> Frame {
        let version: i32 = 5;
        let mut msg = Outgoing::ReqMktDepth.encode();
        msg.push_str(&version.encode()); // version
        msg.push_str(&self.req_id.encode());
        msg.push_str(&self.contract.encode_for_ticker());
        msg.push_str(&self.num_rows.encode());
        msg.push_str(&self.is_smart_depth.encode());
        // send mkt_depth_options parameter
        // current doc says this part if for "internal use only" -> won't support it
        msg.push('\0');
        let msg = msg.as_str().to_ib_message().unwrap();
        Frame::Bulk(Bytes::from(msg))
    }
}
/// Requests venues for which market data is returned to update_mkt_depth_l2
/// (those with market makers)
#[derive(Debug, Clone, Copy)]
pub struct MarketDepthExchangesRequest;
impl IntoIbkrFrame for MarketDepthExchangesRequest {
    fn into_frame(&self) -> Frame {
        let msg = Outgoing::ReqMktDepthExchanges.encode();
        let msg = msg.as_str().to_ib_message().unwrap();
        Frame::Bulk(Bytes::from(msg))
    }
}
/// Request tick by tick data
///
/// # Arguments
/// * req_id    - unique identifier of the request.
/// * contract    - the contract for which tick-by-tick data is requested.
/// * tick_type    - TickByTickType data type: "Last", "AllLast", "BidAsk" or
///   "MidPoint".
/// * number_of_ticks    - number of ticks.
/// * ignore_size    - ignore size flag./
#[derive(Debug)]
pub struct TickByTickRequest {
    pub req_id:          RequestId,
    pub contract:        Contract,
    pub tick_type:       TickByTickType,
    pub number_of_ticks: i32,
    pub ignore_size:     bool,
}
impl IntoIbkrFrame for TickByTickRequest {
    fn into_frame(&self) -> Frame {
        let mut msg = Outgoing::ReqTickByTickData.encode();
        // msg.push_str(&VERSION.encode()); // version
        msg.push_str(&self.req_id.encode());
        msg.push_str(&self.contract.encode_for_ticker());
        msg.push_str(&self.tick_type.encode());
        msg.push_str(&self.number_of_ticks.encode());
        msg.push_str(&self.ignore_size.encode());
        let msg = msg.as_str().to_ib_message().unwrap();
        Frame::Bulk(Bytes::from(msg))
    }
}
// //----------------------------------------------------------------------------------------------
// /// Returns data histogram of specified contract
// ///
// /// # Arguments
// /// * ticker_id - an identifier for the request
// /// * contract - Contract object for which histogram is being requested
// /// * use_rth - use regular trading hours only, 1 for yes or 0 for no
// /// * time_period - period of which data is being requested, e.g. "3 days"
// #[tracing::instrument(level = "debug", skip(self))]
// pub fn req_histogram_data(
//     &mut self,
//     ticker_id: i32,
//     contract: &Contract,
//     use_rth: bool,
//     time_period: &str,
// ) -> Result<(), IBKRApiLibError> {

// /// Cancel the request
// ///
// /// # Arguments
// /// * req_id - the id of the original request
// pub fn cancel_histogram_data(&mut self, ticker_id: i32) -> Result<(),
// IBKRApiLibError> {

#[derive(Debug, Clone, Copy)]
pub enum HistoricalTickDateTime {
    Start(TimeStamp),
    End(TimeStamp),
}

/// Call the req_real_time_bars() function to start receiving real time bar
/// results through the realtimeBar() EWrapper function.
///
/// # Arguments
/// * req_id - The Id for the request. Must be a unique value. When the data is
///   received, it will be identified by this Id. This is also used when
///   canceling the request.
/// * contract - This object contains a description of the contract for which
///   real time bars are being requested
/// * bar_size - Currently only 5 second bars are supported, if any other value
///   is used, an exception will be thrown.
/// * what_to_show - Determines the nature of the data extracted. Valid values
///   include:
///                  * TRADES
///                  * BID
///                  * ASK
///                  * MIDPOINT
/// * use_rth:bool - Regular Trading Hours only. Valid values include:
///                  * 0 = all data available during the time span requested is
///                    returned, including time intervals when the market in
///                    question was outside of regular trading hours.
///                  * 1 = only data within the regular trading hours for the
///                    product requested is returned, even if the time time span
///                    falls partially or completely outside.
/// * real_time_bars_options: - For internal use only. Use pub fnault value XYZ

// Requests real time bars
// Currently, only 5 seconds bars are provided. This request is subject to the same pacing as any
// historical data request: no more than 60 API queries in more than 600 seconds. Real time bars
// subscriptions are also included in the calculation of the number of Level 1 market data
// subscriptions allowed in an account.
#[derive(Debug)]
pub struct RealtimeBarRequest {
    pub req_id:                 RequestId,
    pub contract:               Contract,
    pub bar_size:               BarSize,
    pub what_to_show:           HistoricalDataType,
    pub use_rth:                UseRegularTradingHoursOnly,
    pub real_time_bars_options: Vec<TagValue>,
}
impl IntoIbkrFrame for RealtimeBarRequest {
    fn into_frame(&self) -> Frame {
        let version: i32 = 3;

        let mut msg = Outgoing::ReqRealTimeBars.encode();
        msg.push_str(&version.encode());
        msg.push_str(&self.req_id.encode());
        msg.push_str(&self.contract.encode_for_ticker());

        // * bar_size - Currently only 5 second bars are supported, if any other value
        //   is used, an exception will be thrown.
        // msg.push_str(&self.bar_size.encode());
        msg.push_str("5\0");
        msg.push_str(&self.what_to_show.encode());
        msg.push_str(&self.use_rth.encode());

        let options_str = self
            .real_time_bars_options
            .iter()
            .map(|x| format!("{}={};", x.tag, x.value))
            .collect::<String>();
        msg.push_str(&options_str.encode());
        msg.push('\0');
        let msg = msg.as_str().to_ib_message().unwrap();
        Frame::Bulk(Bytes::from(msg))
    }
}

//----------------------------------------------------------------------------------------------
/// Call this to stop receiving real time bars.
///
/// # Arguments
/// * req_id - The Id that was specified in the call to req_real_time_bars().
#[derive(Debug, Clone, Copy)]
pub struct CancelRealtimeBars {
    pub req_id: RequestId,
}
impl IntoIbkrFrame for CancelRealtimeBars {
    fn into_frame(&self) -> Frame {
        let version: i32 = 1;
        let mut msg = Outgoing::CancelRealTimeBars.encode();
        msg.push_str(&version.encode());
        msg.push_str(&self.req_id.encode());
        let msg = msg.as_str().to_ib_message().unwrap();
        Frame::Bulk(Bytes::from(msg))
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// format_date - Determines the date format applied to returned bars. validd
/// values include:
///
/// 1 - dates applying to bars returned in the format:
/// yyyymmdd{space}{space}hh:mm:dd 2 - dates are returned as a long integer
/// specifying the number of seconds since 1/1/1970 GMT.
pub enum IntradayBarDateFormat {
    YYYYMMDD,
    UnixEpochSeconds,
}

impl Encodable for IntradayBarDateFormat {
    fn encode(&self) -> String {
        match self {
            Self::YYYYMMDD => "1\0",
            Self::UnixEpochSeconds => "2\0",
        }
        .to_string()
    }
}

impl FromStr for IntradayBarDateFormat {
    type Err = ParseEnumError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" => Ok(Self::YYYYMMDD),
            "2" => Ok(Self::UnixEpochSeconds),
            &_ => Err(ParseEnumError),
        }
    }
}

impl Decodable for IntradayBarDateFormat {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// use regular trading hours only, 1 for yes or 0 for no
pub enum UseRegularTradingHoursOnly {
    DontUse,
    Use,
}

impl Encodable for UseRegularTradingHoursOnly {
    fn encode(&self) -> String {
        match self {
            Self::DontUse => "0\0",
            Self::Use => "1\0",
        }
        .to_string()
    }
}

impl FromStr for UseRegularTradingHoursOnly {
    type Err = ParseEnumError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(Self::DontUse),
            "1" => Ok(Self::Use),
            &_ => Err(ParseEnumError),
        }
    }
}

impl Decodable for UseRegularTradingHoursOnly {}

#[derive(Debug, Clone, Copy)]
pub enum UsePriceMgmtAlgo {
    DontUse,
    Use,
}

impl Encodable for UsePriceMgmtAlgo {
    fn encode(&self) -> String {
        match self {
            UsePriceMgmtAlgo::DontUse => "0\0",
            UsePriceMgmtAlgo::Use => "1\0",
        }
        .to_string()
    }
}

impl FromStr for UsePriceMgmtAlgo {
    type Err = ParseEnumError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(UsePriceMgmtAlgo::DontUse),
            "1" => Ok(UsePriceMgmtAlgo::Use),
            &_ => Err(ParseEnumError),
        }
    }
}
impl Decodable for UsePriceMgmtAlgo {}
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct TagValue {
    pub tag:   String,
    pub value: String,
}

impl TagValue {
    pub const fn new(tag: String, value: String) -> Self { TagValue { tag, value } }
}
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum HistoricalDataType {
    Trades,
    Midpoint,
    Bid,
    Ask,
    BidAsk,
    AdjustedLast,
    HistoricalVolatility,
    OptionImpliedVolatility,
    RebateRate,
    FeeRate,
    YieldBid,
    YieldAsk,
    YieldBidAsk,
    YieldLast,
    Schedule,
}

impl Encodable for HistoricalDataType {
    fn encode(&self) -> String {
        match self {
            HistoricalDataType::Trades => "TRADES\0",
            HistoricalDataType::Midpoint => "MIDPOINT\0",
            HistoricalDataType::Bid => "BID\0",
            HistoricalDataType::Ask => "ASK\0",
            HistoricalDataType::BidAsk => "BID_ASK\0",
            HistoricalDataType::AdjustedLast => "ADJUSTED_LAST\0",
            HistoricalDataType::HistoricalVolatility => "HISTORICAL_VOLATILITY\0",
            HistoricalDataType::OptionImpliedVolatility => "OPTION_IMPLIED_VOLATILITY\0",
            HistoricalDataType::RebateRate => "REBATE_RATE\0",
            HistoricalDataType::FeeRate => "FEE_RATE\0",
            HistoricalDataType::YieldBid => "YIELD_BID\0",
            HistoricalDataType::YieldAsk => "YIELD_ASK\0",
            HistoricalDataType::YieldBidAsk => "YIELD_BID_ASK\0",
            HistoricalDataType::YieldLast => "YIELD_LAST\0",
            HistoricalDataType::Schedule => "SCHEDULE\0",
        }
        .to_string()
    }
}
impl Default for HistoricalDataType {
    fn default() -> Self { Self::Trades }
}

impl FromStr for HistoricalDataType {
    type Err = ParseEnumError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let res = match s {
            "TRADES" => HistoricalDataType::Trades,
            "MIDPOINT" => HistoricalDataType::Midpoint,
            "BID" => HistoricalDataType::Bid,
            "ASK" => HistoricalDataType::Ask,
            "BID_ASK" => HistoricalDataType::BidAsk,
            "ADJUSTED_LAST" => HistoricalDataType::AdjustedLast,
            "HISTORICAL_VOLATILITY" => HistoricalDataType::HistoricalVolatility,
            "OPTION_IMPLIED_VOLATILITY" => HistoricalDataType::OptionImpliedVolatility,
            "REBATE_RATE" => HistoricalDataType::RebateRate,
            "FEE_RATE" => HistoricalDataType::FeeRate,
            "YIELD_BID" => HistoricalDataType::YieldBid,
            "YIELD_ASK" => HistoricalDataType::YieldAsk,
            "YIELD_BID_ASK" => HistoricalDataType::YieldBidAsk,
            "YIELD_LAST" => HistoricalDataType::YieldLast,
            "SCHEDULE" => HistoricalDataType::Schedule,
            &_ => return Err(ParseEnumError),
        };
        Ok(res)
    }
}

impl Display for HistoricalDataType {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result { write!(f, "{self:?}") }
}

impl Decodable for HistoricalDataType {}
#[derive(Debug, Clone, Copy)]
pub enum MarketDepthOperation {
    Insert = 0,
    Update = 1,
    Remove = 2,
}

impl FromStr for MarketDepthOperation {
    type Err = ParseEnumError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let res = match s {
            "0" => MarketDepthOperation::Insert,
            "1" => MarketDepthOperation::Update,
            "2" => MarketDepthOperation::Remove,
            &_ => return Err(ParseEnumError),
        };
        Ok(res)
    }
}

impl Decodable for MarketDepthOperation {}

#[derive(Debug, Clone, Copy)]
pub enum DepthSide {
    Bid,
    Ask,
}

impl FromStr for DepthSide {
    type Err = ParseEnumError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(DepthSide::Bid),
            "1" => Ok(DepthSide::Ask),
            &_ => Err(ParseEnumError),
        }
    }
}

impl Decodable for DepthSide {}
