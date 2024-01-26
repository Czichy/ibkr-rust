use crossbeam::channel::Receiver;
use tokio::{net::ToSocketAddrs, runtime::Runtime};

use crate::{
    account::AccountData,
    contract::{Contract, ContractDetails},
    enums::{GenericTickType, MarketDataType, TickType},
    order::ExecutionFilter,
    prelude::TagValue,
    AccountCode, ClientId, RequestId, Result,
};
use crate::{client::ConnectionStatus, order::OrderTracker};

/// Established connection with a Redis server.
///
/// Backed by a single `TcpStream`, `BlockingClient` provides basic network
/// client functionality (no pooling, retrying, ...). Connections are
/// established using the [`connect`](fn@connect) function.
///
/// Requests are issued using the various methods of `Client`.
pub struct BlockingClient {
    /// The asynchronous `Client`.
    inner: crate::client::Client,

    /// A `current_thread` runtime for executing operations on the asynchronous
    /// client in a blocking manner.
    rt: Runtime,
}

impl BlockingClient {
    /// The client_id of the client
    pub fn client_id(&self) -> ClientId {
        self.inner.client_id
    }

    /// Checks connection status
    pub fn is_connected(&self) -> bool {
        matches!(self.inner.conn_state, ConnectionStatus::CONNECTED)
    }
}

// /// Establish a connection with the Redis server located at `addr`.
// ///
// /// `addr` may be any type that can be asynchronously converted to a
// /// `SocketAddr`. This includes `SocketAddr` and strings. The `ToSocketAddrs`
// /// trait is the Tokio version and not the `std` version.
// ///
// /// # Examples
// ///
// /// ```no_run
// /// use mini_redis::blocking_client;
// ///
// /// fn main() {
// ///     let client = match blocking_client::connect("localhost:6379") {
// ///         Ok(client) => client,
// ///         Err(_) => panic!("failed to establish connection"),
// ///     };
// /// # drop(client);
// /// }
// /// ```
pub fn connect<T: ToSocketAddrs>(addr: T, client_id: ClientId) -> Result<BlockingClient> {
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?;

    let inner = rt.block_on(crate::client::connect(addr, client_id))?;

    Ok(BlockingClient { inner, rt })
}

impl BlockingClient {
    // /// Get the value of key.
    // ///
    // /// If the key does not exist the special value `None` is returned.
    // ///
    // /// # Examples
    // ///
    // /// Demonstrates basic usage.
    // ///
    // /// ```no_run
    // /// use mini_redis::blocking_client;
    // ///
    // /// fn main() {
    // ///     let mut client = blocking_client::connect("localhost:6379").unwrap();
    // ///
    // ///     let val = client.get("foo").unwrap();
    // ///     println!("Got = {:?}", val);
    // /// }
    // /// ```
    pub fn subscribe_account_updates(&mut self) -> Receiver<AccountData> {
        self.inner.subscribe_account_updates()
    }

    pub fn request_account_updates(
        &mut self,
        subscribe: bool,
        account_code: AccountCode,
    ) -> Result<()> {
        self.rt
            .block_on(self.inner.request_account_updates(subscribe, account_code))
    }

    pub fn request_account_summary(&mut self, group_name: String, tags: Vec<String>) -> Result<()> {
        self.rt
            .block_on(self.inner.request_account_summary(group_name, tags))
    }

    pub fn subscribe_contract_details(&mut self) -> Receiver<ContractDetails> {
        self.inner.subscribe_contract_details()
    }

    pub fn req_contract_details(&mut self, req_id: RequestId, contract: Contract) -> Result<()> {
        self.rt
            .block_on(self.inner.request_contract_details(req_id, contract))
    }

    pub fn get_contract_details(
        &mut self,
        req_id: RequestId,
        contract: Contract,
    ) -> Result<Vec<ContractDetails>> {
        self.rt
            .block_on(self.inner.get_contract_details(req_id, contract))
    }

    pub fn subscribe_orders(&mut self) -> OrderTracker {
        self.inner.subscribe_orders()
    }

    pub fn request_completed_orders(&mut self, api_only: bool) -> Result<()> {
        self.rt
            .block_on(self.inner.request_completed_orders(api_only))
    }

    pub fn request_all_open_orders(&mut self) -> Result<()> {
        self.rt.block_on(self.inner.request_all_open_orders())
    }

    pub fn request_auto_open_orders(&mut self, auto_bind: bool) -> Result<()> {
        self.rt
            .block_on(self.inner.request_auto_open_orders(auto_bind))
    }

    pub fn request_ids(&mut self) -> Result<()> {
        self.rt.block_on(self.inner.request_ids())
    }

    pub fn request_current_time(&mut self) -> Result<()> {
        self.rt.block_on(self.inner.request_current_time())
    }

    /// When this function is called, the execution reports that meet the
    /// filter criteria are downloaded to the client via the execDetails()
    /// function. To view executions beyond the past 24 hours, open the
    /// Trade Log in TWS and, while the Trade Log is displayed, request
    /// the executions again from the API.
    ///
    /// # Arguments
    /// * req_id - The ID of the data request. Ensures that responses are
    ///   matched to requests if several requests are in process.
    /// * exec_filter - This object contains attributes that describe the filter
    ///   criteria used to determine which execution reports are returned.
    ///
    /// NOTE: Time format must be 'yyyymmdd-hh:mm:ss' Eg: '20030702-14:55'
    pub fn request_executions(
        &mut self,
        req_id: RequestId,
        exec_filter: Option<ExecutionFilter>,
    ) -> Result<()> {
        self.rt
            .block_on(self.inner.request_executions(req_id, exec_filter))
    }

    /// Request tick by tick data
    ///
    /// # Arguments
    /// * req_id - unique identifier of the request.
    /// * contract - the contract for which tick-by-tick data is requested.
    /// * tick_type - TickByTickType data type: "Last", "AllLast", "BidAsk" or
    ///   "MidPoint".
    /// * number_of_ticks - number of ticks.
    /// * ignore_size - ignore size flag./
    pub fn request_tick_by_tick_data(
        &mut self,
        req_id: RequestId,
        contract: Contract,
        tick_type: TickType,
        number_of_ticks: i32,
        ignore_size: bool,
    ) -> Result<()> {
        self.rt.block_on(self.inner.request_tick_by_tick_data(
            req_id,
            contract,
            tick_type,
            number_of_ticks,
            ignore_size,
        ))
    }

    /// Cancel tick by tick data
    ///
    /// # Arguments
    /// * req_id - The identifier of the original request.
    pub fn cancel_tick_by_tick_data(&mut self, req_id: RequestId) -> Result<()> {
        self.rt
            .block_on(self.inner.cancel_tick_by_tick_data(req_id))
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
    pub fn request_market_data(
        &mut self,
        req_id: RequestId,
        contract: Contract,
        snapshot: bool,
        regulatory: bool,
        additional_data: Vec<GenericTickType>,
    ) -> Result<()> {
        self.rt.block_on(self.inner.request_market_data(
            req_id,
            contract,
            snapshot,
            regulatory,
            additional_data,
        ))
    }

    /// After calling this function, market data for the specified id will stop
    /// flowing.
    ///
    /// # Arguments
    /// * req_id - The ID that was specified in the call to req_mkt_data()
    pub fn cancel_market_data(&mut self, req_id: RequestId) -> Result<()> {
        self.rt.block_on(self.inner.cancel_market_data(req_id))
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
    pub fn request_market_data_type(&mut self, market_data_type: MarketDataType) -> Result<()> {
        self.rt
            .block_on(self.inner.request_market_data_type(market_data_type))
    }

    /// Requests venues for which market data is returned to update_mkt_depth_l2
    /// (those with market makers)
    pub fn request_market_depth_exchanges(&mut self) -> Result<()> {
        self.rt
            .block_on(self.inner.request_market_depth_exchanges())
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
    pub fn req_market_depth(
        &mut self,
        req_id: RequestId,
        contract: Contract,
        num_rows: i32,
        is_smart_depth: bool,
        mkt_depth_options: Vec<TagValue>,
    ) -> Result<()> {
        self.rt.block_on(self.inner.request_market_depth(
            req_id,
            contract,
            num_rows,
            is_smart_depth,
            mkt_depth_options,
        ))
    }

    /// After calling this function, market depth data for the specified id
    /// will stop flowing.
    ///
    /// # Arguments
    /// * req_id - The ID that was specified in the call to req_mkt_depth().
    //
    //  * is_smart_depth - specifies SMART depth request
    pub fn cancel_market_depth(&mut self, req_id: RequestId, is_smart_depth: bool) -> Result<()> {
        self.rt
            .block_on(self.inner.cancel_market_depth(req_id, is_smart_depth))
    }

    pub fn disconnect(&mut self) -> Result<()> {
        self.inner.disconnect()
    }
}
