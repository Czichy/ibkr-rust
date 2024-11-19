use chrono::Utc;

use crate::{cmd::request_market_data::*,
            prelude::{Client, IntoIbkrFrame},
            ticker::MarketDataTracker,
            Result};

impl Client {
    pub fn subscribe_market_data_updates(&mut self) -> MarketDataTracker {
        self.market_data_tracker.clone()
    }

    /// Request tick by tick data
    ///
    /// # Arguments
    /// * req_id	- unique identifier of the request.
    /// * contract	- the contract for which tick-by-tick data is requested.
    /// * tick_type	- TickByTickType data type: "Last", "AllLast", "BidAsk" or
    ///   "MidPoint".
    /// * number_of_ticks	- number of ticks.
    /// * ignore_size	- ignore size flag./
    #[tracing::instrument(skip(self))]
    pub async fn request_tick_by_tick_data(&mut self, request: &TickByTickRequest) -> Result<()> {
        // Write the frame to the socket
        self.writer.write_frame(&request.into_frame()).await?;
        Ok(())
    }

    /// Cancel tick by tick data
    ///
    /// # Arguments
    /// * req_id	- The identifier of the original request.
    #[tracing::instrument(skip(self))]
    pub async fn cancel_tick_by_tick_data(
        &mut self,
        request: &CancelTickByTickRequest,
    ) -> Result<()> {
        // Write the frame to the socket
        self.writer.write_frame(&request.into_frame()).await?;
        Ok(())
    }

    /// Call this function to request market data. The market data
    /// will be returned by the tick_price and tick_size wrapper events.
    ///
    /// # Arguments
    /// * req_id - The request id. Must be a unique value. When the market data
    ///   returns, it will be identified by this tag. This is also used when
    ///   canceling the market data.
    /// * contract - This structure contains a description of the Contract for
    ///   which market data is being requested.
    /// * generic_tick_list - A commma delimited list of generic tick types.
    ///   Tick types can be found in the Generic Tick Types page. Prefixing w/
    ///   'mdoff' indicates that top mkt data shouldn't tick. You can specify
    ///   the news source by postfixing w/ ':<source>. Example: "mdoff, 292: FLY
    ///   + BRF"
    /// * snapshot - Check to return a single snapshot of Market data and have
    ///   the market data subscription cancel. Do not enter any
    ///   generic_tick_list values if you use snapshots.
    /// * regulatory_snapshot - With the US Value Snapshot Bundle for stocks,
    ///   regulatory snapshots are available for 0.01 USD each.
    /// * mkt_data_options - For internal use only. Use default value XYZ.
    #[tracing::instrument(skip(self))]
    pub async fn request_market_data(&mut self, request: &MarketDataRequest) -> Result<()> {
        self.subscriptions_by_time
            .insert(request.req_id, Utc::now());
        // Write the frame to the socket
        self.writer.write_frame(&request.into_frame()).await?;
        Ok(())
    }

    /// After calling this function, market data for the specified id will stop
    /// flowing.
    ///
    /// # Arguments
    /// * req_id - The ID that was specified in the call to req_mkt_data()
    #[tracing::instrument(skip(self))]
    pub async fn cancel_market_data(&mut self, request: &CancelMarketDataRequest) -> Result<()> {
        // when unsubscribing symbols immediately after subscribing IB returns an error
        // (Can't find EId with tickerId:nnn), so we track subscription times to
        // ensure symbols are not unsubscribed before a minimum time span has elapsed
        if let Some(subscription_time) = self.subscriptions_by_time.get(&request.req_id) {
            let time_since_subscription = Utc::now() - *subscription_time;
            if time_since_subscription < self.min_timespan_before_unsubscribe {
                let delay = self.min_timespan_before_unsubscribe - time_since_subscription;
                tokio::time::sleep(std::time::Duration::from_millis(
                    delay.num_milliseconds() as u64
                ))
                .await;
            }
            // convert the command into a frame
            // Write the frame to the socket
            self.writer.write_frame(&request.into_frame()).await?;
        }
        Ok(())
    }

    #[tracing::instrument(skip(self))]
    pub async fn request_realtime_bars(&mut self, request: &RealtimeBarRequest) -> Result<()> {
        self.subscriptions_by_time
            .insert(request.req_id, Utc::now());
        // Write the frame to the socket
        self.writer.write_frame(&request.into_frame()).await?;
        Ok(())
    }

    /// After calling this function, market data for the specified id will stop
    /// flowing.
    ///
    /// # Arguments
    /// * req_id - The ID that was specified in the call to req_mkt_data()
    #[tracing::instrument(skip(self))]
    pub async fn cancel_realtime_bars(&mut self, request: &CancelRealtimeBars) -> Result<()> {
        // when unsubscribing symbols immediately after subscribing IB returns an error
        // (Can't find EId with tickerId:nnn), so we track subscription times to
        // ensure symbols are not unsubscribed before a minimum time span has elapsed
        if let Some(subscription_time) = self.subscriptions_by_time.get(&request.req_id) {
            let time_since_subscription = Utc::now() - *subscription_time;
            if time_since_subscription < self.min_timespan_before_unsubscribe {
                let delay = self.min_timespan_before_unsubscribe - time_since_subscription;
                tokio::time::sleep(std::time::Duration::from_millis(
                    delay.num_milliseconds() as u64
                ))
                .await;
            }
            // convert the command into a frame
            // Write the frame to the socket
            self.writer.write_frame(&request.into_frame()).await?;
        }
        Ok(())
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
    #[tracing::instrument(skip(self))]
    pub async fn request_market_data_type(
        &mut self,
        request: &MarketDataTypeRequest,
    ) -> Result<()> {
        // Write the frame to the socket
        self.writer.write_frame(&request.into_frame()).await?;
        Ok(())
    }

    /// Requests venues for which market data is returned to update_mkt_depth_l2
    /// (those with market makers)
    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn request_market_depth_exchanges(&mut self) -> Result<()> {
        // Write the frame to the socket
        self.writer
            .write_frame(&MarketDepthExchangesRequest.into_frame())
            .await?;
        Ok(())
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
    ///   data returns, it will be identified by this tag. This is also used
    ///   when canceling the market depth
    /// * contract - This structure contains a description of the contract for
    ///   which market depth data is being requested.
    /// * num_rows - Specifies the number of rows of market depth rows to
    ///   display.
    /// * is_smart_depth - specifies SMART depth request  NOTE: ALWAYS SET TO
    ///   FALSE!!!!! THERE SEEMS TO BE A BUG ON IB's SIDE AND THEY WILL STOP
    ///   STREAMING DATA IF THIS IS SET TO TRUE
    /// * mkt_depth_options - For internal use only. Use default value XYZ.
    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn request_market_depth(&mut self, request: &MarketDepthRequest) -> Result<()> {
        // Write the frame to the socket
        self.writer.write_frame(&request.into_frame()).await?;
        Ok(())
    }

    /// After calling this function, market depth data for the specified id
    /// will stop flowing.
    ///
    /// # Arguments
    /// * req_id - The ID that was specified in the call to req_mkt_depth().
    //  * is_smart_depth - specifies SMART depth request
    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn cancel_market_depth(&mut self, request: &CancelMarketDepthRequest) -> Result<()> {
        // Write the frame to the socket
        self.writer.write_frame(&request.into_frame()).await?;
        Ok(())
    }

    //#########################################################################
    //################## Historical Data
    //################## #######################################################
    /// Requests contracts' historical data. When requesting historical data, a
    /// finishing time and date is required along with a duration string. The
    /// resulting bars will be returned in EWrapper.historicalData()
    ///
    /// # Arguments
    /// * req_id - The id of the request. Must be a unique value. When the
    ///   market data returns, it whatToShowill be identified by this tag. This
    ///   is also used when canceling the market data.
    /// * contract - This object contains a description of the contract for
    ///   which market data is being requested.
    /// * end_date_time - Defines a query end date and time at any point during
    ///   the past 6 mos. Valid values include any date/time within the past six
    ///   months in the format: yyyymmdd HH:mm:ss ttt where "ttt" is the
    ///   optional time zone.
    /// * duration_str - Set the query duration up to one week, using a time
    ///   unit of seconds, days or weeks. Valid values include any integer
    ///   followed by a space and then S (seconds)); D (days) or W (week). If no
    ///   unit is specified, seconds is used.
    /// * bar_size_setting - See the BarSize enum for valid values. Specifies
    ///   the size of the bars that will be returned (within IB/TWS listimits).
    ///   Valid values include:
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
    /// * what_to_show - See the WhatToShow enum for valid values.  Determines
    ///   the nature of data beinging extracted. Valid values include:
    ///
    ///     * TRADES
    ///     * MIDPOINT
    ///     * BID
    ///     * ASK
    ///     * BID_ASK
    ///     * HISTORICAL_VOLATILITY
    ///     * OPTION_IMPLIED_VOLATILITY
    /// * use_rth - Determines whether to return all data available during the
    ///   requested time span, or only data that falls within regular trading
    ///   hours. Valid values include:
    ///
    ///     * 0 - all data is returned even where the market in question was
    ///       outside of its regular trading hours.
    ///     * 1 - only data within the regular trading hours is returned, even
    ///       if the requested time span falls partially or completely outside
    ///       of the RTH.
    /// * format_date - Determines the date format applied to returned bars.
    ///   validd values include:
    ///
    ///     * 1 - dates applying to bars returned in the format:
    ///       yyyymmdd{space}{space}hh:mm:dd
    ///     * 2 - dates are returned as a long integer specifying the number of
    ///       seconds since 1/1/1970 GMT.
    /// *chart_options: - For internal use only. Use default value XYZ.
    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn request_historical_data(&mut self, request: &HistoricalDataRequest) -> Result<()> {
        // Write the frame to the socket
        self.writer.write_frame(&request.into_frame()).await?;
        Ok(())
    }

    //----------------------------------------------------------------------------------------------
    /// Used if an internet disconnect has occurred or the results of a query
    /// are otherwise delayed and the client is no longer interested in
    /// receiving the data.
    ///
    /// # Arguments
    /// * req_id - the id of the original request
    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn cancel_historical_data(
        &mut self,
        request: &CancelHistoricalDataRequest,
    ) -> Result<()> {
        // Write the frame to the socket
        self.writer.write_frame(&request.into_frame()).await?;
        Ok(())
    }

    // //----------------------------------------------------------------------------------------------
    // /// Returns the timestamp of earliest available historical data for a
    // contract and data type. ///
    // /// # Arguments
    // /// * req_id	- an identifier for the request
    // /// * contract	- contract object for which head timestamp is being requested
    // /// * what_to_show	- type of data for head timestamp - "BID", "ASK", "TRADES",
    // etc /// * use_rth	- use regular trading hours only, 1 for yes or 0 for no
    // /// * format_date	set to 1 to obtain the bars' time as yyyyMMdd HH:mm:ss, set
    // to 2 to obtain it like system time format in seconds ///
    // /// Note that formatData parameter affects intraday bars only
    // /// 1-day bars always return with date in YYYYMMDD format
    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn request_head_time_stamp(&mut self, request: &HeadTimestampRequest) -> Result<()> {
        // Write the frame to the socket
        self.writer.write_frame(&request.into_frame()).await?;
        Ok(())
    }

    // //----------------------------------------------------------------------------------------------
    // /// Cancel the request
    // ///
    // /// # Arguments
    // /// * req_id - the id of the original request
    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn cancel_head_time_stamp(
        &mut self,
        request: &CancelHeadTimestampRequest,
    ) -> Result<()> {
        // Write the frame to the socket
        self.writer.write_frame(&request.into_frame()).await?;
        Ok(())
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
    // convert the command into a frame
    // ;

    // // Write the frame to the socket
    // self.writer.write_frame(&request.into_frame()).await?;
    // Ok(())
    // }

    // //----------------------------------------------------------------------------------------------
    // /// Cancel the request
    // ///
    // /// # Arguments
    // /// * req_id - the id of the original request
    // pub fn cancel_histogram_data(&mut self, ticker_id: i32) -> Result<(),
    // IBKRApiLibError> { // convert the command into a frame
    // ;

    // // Write the frame to the socket
    // self.writer.write_frame(&request.into_frame()).await?;
    // Ok(())
    // }

    //----------------------------------------------------------------------------------------------
    /// Requests historical Time&Sales data for an instrument.
    ///
    /// # Arguments
    /// * req_id - id of the request
    /// * contract - Contract object that is subject of query
    /// * start_date_time,i.e.	"20170701 12:01:00". Uses TWS timezone specified
    ///   at login.
    /// * end_date_time,i.e.	"20170701 13:01:00". In TWS timezone. Exactly one of
    ///   start time and end time has to be defined.
    /// * number_of_ticks - Number of distinct data points. Max currently 1000
    ///   per request.
    /// * what_to_show - (Bid_Ask, Midpoint, Trades) Type of data requested.
    /// * use_rth - Data from regular trading hours (1), or all available hours
    ///   (0)
    /// * ignore_size - A filter only used when the source price is Bid_Ask
    /// * misc_options - should be defined as null, reserved for internal use
    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn request_historical_ticks(
        &mut self,
        request: &HistoricalTicksRequest,
    ) -> Result<()> {
        // Write the frame to the socket
        self.writer.write_frame(&request.into_frame()).await?;
        Ok(())
    }
}
