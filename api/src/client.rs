//! Minimal Redis client implementation
//!
//! Provides an async connect and methods for issuing the supported commands.
use std::{collections::{HashMap, VecDeque},
          sync::atomic::AtomicUsize};

use chrono::{DateTime, Utc};
use crossbeam::channel::{unbounded, Receiver, Sender};
use tokio::{net::{TcpStream, ToSocketAddrs},
            sync::{broadcast, mpsc}};
use tracing::{debug, error, info, instrument};

use crate::{cmd::*,
            ib_frame::IBFrame,
            order::{OrderTracker, OrderTrackerSender},
            prelude::*,
            reader::Reader,
            shutdown::Shutdown,
            writer::Writer,
            ServerVersion};
mod account;
mod contract_details;
mod executions;
mod market_data;
mod orders;

#[derive(Debug)]
pub struct ResponseWithId<T> {
    pub req_id:   RequestId,
    pub response: Option<T>,
}
/// Connection status
#[repr(i32)]
#[derive(Copy, Clone, Debug)]
pub enum ConnectionStatus {
    DISCONNECTED,
    CONNECTING,
    CONNECTED,
    READY,
    REDIRECT,
}
#[derive(Debug)]
pub enum Request {
    OrderId {
        sender: mpsc::Sender<Option<OrderId>>,
    },
    RequestWithId {
        req_id: RequestId,
        sender: mpsc::Sender<ResponseWithId<ContractDetails>>,
    },
}

// #[derive(Debug, Clone)]
// pub struct ContractDetailsResponse {
//     #[allow(dead_code)]
//     req_id:  RequestId,
//     details: Option<ContractDetails>,
// }

/// Established connection with a Redis server.
///
/// Backed by a single `TcpStream`, `Client` provides basic network client
/// functionality (no pooling, retrying, ...). Connections are established using
/// the [`connect`](fn@connect) function.
///
/// Requests are issued using the various methods of `Client`.
#[derive(Debug)]
pub struct Client {
    /// The TCP connection decorated with the redis protocol encoder / decoder
    /// implemented using a buffered `TcpStream`.
    ///
    /// When `Listener` receives an inbound connection, the `TcpStream` is
    /// passed to `Connection::new`, which initializes the associated buffers.
    /// `Connection` allows the handler to operate at the "frame" level and keep
    /// the byte level protocol parsing details encapsulated in `Connection`.
    // connection:      Connection,
    writer: Writer,
    pub client_id:                   ClientId,
    server_version:                  ServerVersion,
    pub conn_state:                  ConnectionStatus,
    next_req_id:                     AtomicUsize,
    #[allow(dead_code)]
    subscriptions_by_time:           HashMap<RequestId, DateTime<Utc>>,
    #[allow(dead_code)]
    min_timespan_before_unsubscribe: chrono::Duration,
    /// Broadcasts a shutdown signal to all active connections.
    ///
    /// The initial `shutdown` trigger is provided by the `run` caller. The
    /// server is responsible for gracefully shutting down active connections.
    /// When a connection task is spawned, it is passed a broadcast receiver
    /// handle. When a graceful shutdown is initiated, a `()` value is sent via
    /// the broadcast::Sender. Each active connection receives it, reaches a
    /// safe terminal state, and completes the task.
    notify_shutdown:                 broadcast::Sender<()>,

    /// Used as part of the graceful shutdown process to wait for client
    /// connections to complete processing.
    ///
    /// Tokio channels are closed once all `Sender` handles go out of scope.
    /// When a channel is closed, the receiver receives `None`. This is
    /// leveraged to detect all connection handlers completing. When a
    /// connection handler is initialized, it is assigned a clone of
    /// `shutdown_complete_tx`. When the listener shuts down, it drops the
    /// sender held by this `shutdown_complete_tx` field. Once all handler tasks
    /// complete, all clones of the `Sender` are also dropped. This results in
    /// `shutdown_complete_rx.recv()` completing with `None`. At this point, it
    /// is safe to exit the server process.
    #[allow(dead_code)]
    shutdown_complete_rx: mpsc::Receiver<()>,
    #[allow(dead_code)]
    shutdown_complete_tx: mpsc::Sender<()>,

    subscribe_handler_tx:       mpsc::Sender<Request>,
    pub order_tracker:          OrderTracker,
    pub account_tracker:        Receiver<AccountData>,
    pub market_data_tracker:    MarketDataTracker,
    pub contract_events:        Receiver<ResponseWithId<ContractDetails>>,
    pub account_update_tracker: Receiver<DateTime<Utc>>,
    pub message_tracker:        Receiver<TwsApiMessage>,
}

/// Establish a connection with the Redis server located at `addr`.
///
/// `addr` may be any type that can be asynchronously converted to a
/// `SocketAddr`. This includes `SocketAddr` and strings. The `ToSocketAddrs`
/// trait is the Tokio version and not the `std` version.
///
/// # Examples
///
/// ```no_run
/// use mini_redis::client;
///
/// #[tokio::main]
/// async fn main() {
///     let client = match client::connect("localhost:6379").await {
///         Ok(client) => client,
///         Err(_) => panic!("failed to establish connection"),
///     };
/// # drop(client);
/// }
/// ```
pub async fn connect<T: ToSocketAddrs + Send>(addr: T, client_id: ClientId) -> Result<Client> {
    // The `addr` argument is passed directly to `TcpStream::connect`. This
    // performs any asynchronous DNS lookup and attempts to establish the TCP
    // connection. An error at either step returns an error, which is then
    // bubbled up to the caller of `mini_redis` connect.
    let socket = TcpStream::connect(addr).await?;

    let (recv, trans) = socket.into_split();
    // Initialize the connection state. This allocates read/write buffers to
    // perform redis protocol frame parsing.
    let mut writer = Writer::new(trans);
    let mut reader = Reader::new(recv);
    // initiate handshake
    writer.write_raw(b"API\0").await?;
    let frame = Api::Init {
        min_client_version: constants::MIN_CLIENT_VER,
        max_client_version: constants::MAX_CLIENT_VER,
    };
    writer.write_frame(&frame.into_frame()).await?;

    // Read the response
    let response = reader.read_frame(None).await?;
    debug!("{:?}", response);
    let (server_version, _connection_time) = match response {
        Some(IBFrame::ServerVersion {
            server_version,
            connection_time,
        }) => (server_version, connection_time),
        _ => return Err("could not get server version".into()),
    };

    tracing::error!("Server Version: {}", &server_version);
    // start API
    let frame = Api::Start {
        client_id,
        optional_capabilities: None,
    };
    writer.write_frame(&frame.into_frame()).await?;

    let conn_state = ConnectionStatus::CONNECTED;

    // When the provided `shutdown` future completes, we must send a shutdown
    // message to all active connections. We use a broadcast channel for this
    // purpose. The call below ignores the receiver of the broadcast pair, and when
    // a receiver is needed, the subscribe() method on the sender is used to create
    // one.
    let (notify_shutdown, _) = broadcast::channel(1);
    let (shutdown_complete_tx, shutdown_complete_rx) = mpsc::channel(1);
    let (subscribe_handler_tx, subscribe_handler_rx) = mpsc::channel(100);
    let (account_tx, account) = unbounded();
    let (contract_tx, contract) = unbounded();
    let (message_tx, message) = unbounded();
    let (account_update_tx, account_update) = unbounded();
    let (market_data_tracker_tx, market_data_tracker) = MarketDataTracker::new();
    let (order_tracker_tx, order_tracker) = OrderTracker::new();
    // init handler
    let client = Client {
        writer,
        // connection,
        client_id,
        server_version,
        conn_state,
        next_req_id: AtomicUsize::new(0),
        subscriptions_by_time: HashMap::new(),
        min_timespan_before_unsubscribe: chrono::Duration::milliseconds(500),
        notify_shutdown,
        shutdown_complete_tx,
        shutdown_complete_rx,
        subscribe_handler_tx,
        order_tracker,
        account_tracker: account,
        account_update_tracker: account_update,
        market_data_tracker,
        contract_events: contract,
        message_tracker: message,
    };
    let test = client.notify_shutdown.subscribe();
    tokio::spawn(async move {
        // Process the connection. If an error is encountered, log it.
        if let Err(err) = run(
            reader,
            server_version,
            subscribe_handler_rx,
            test,
            order_tracker_tx,
            account_tx,
            account_update_tx,
            market_data_tracker_tx,
            contract_tx,
            message_tx,
        )
        .await
        {
            tracing::error!(cause = ?err, "connection error");
        }
    });
    Ok(client)
}

/// Run the handler.
///
/// Accepts connections from the supplied listener. For each inbound
/// connection, a task is spawned to handle that connection. The server
/// runs until the `shutdown` future completes, at which point the
/// server shuts down gracefully.
///
/// `tokio::signal::ctrl_c()` can be used as the `shutdown` argument. This
/// will listen for a SIGINT signal.
#[allow(clippy::too_many_arguments)]
async fn run(
    //&mut self,
    reader: Reader,
    server_version: ServerVersion,
    // socket: tokio::net::tcp::OwnedReadHalf,
    subscribe_handler_rx: mpsc::Receiver<Request>,
    notify_shutdown: broadcast::Receiver<()>,
    order_tracker_tx: OrderTrackerSender,
    account_tracker_tx: Sender<AccountData>,
    account_update_tracker_tx: Sender<AccountLastUpdate>,
    market_data_tracker_tx: MarketDataTrackerSender,
    contract_details_events_tx: Sender<ResponseWithId<ContractDetails>>,
    message_events_tx: Sender<TwsApiMessage>,
) -> Result<()> {
    // When the provided `shutdown` future completes, we must send a shutdown
    // message to all active connections. We use a broadcast channel for this
    // purpose. The call below ignores the receiver of the broadcast pair, and when
    // a receiver is needed, the subscribe() method on the sender is used to create
    // one.
    // let (notify_shutdown, _) = broadcast::channel(1);
    // let (shutdown_complete_tx, shutdown_complete_rx) = mpsc::channel(1);
    // Create the necessary per-connection handler state.
    let mut handler = Handler {
        // Initialize the connection state. This allocates read/write
        // buffers to perform redis protocol frame parsing.
        reader, //: Reader::new(socket),

        // The connection state needs a handle to the max connections
        // semaphore. When the handler is done processing the
        // connection, a permit is added back to the semaphore.
        // limit_connections: self.limit_connections.clone(),

        // Receive shutdown notifications.
        shutdown: Shutdown::new(notify_shutdown),

        server_version,
        // Notifies the receiver half once all clones are
        // dropped.
        //_shutdown_complete: shutdown_complete_tx.clone(),
        subscribe_handler_rx,
        order_id_reqs: VecDeque::new(),
        requests: HashMap::new(),
        order_tracker_tx,
        account_tracker_tx,
        account_update_tracker_tx,
        market_data_tracker_tx,
        contract_details_events_tx,
        message_events_tx,
    };

    // Spawn a new task to process the connections. Tokio tasks are like
    // asynchronous green threads and are executed concurrently.
    tokio::spawn(async move {
        // Process the connection. If an error is encountered, log it.
        if let Err(err) = handler.run().await {
            tracing::error!(cause = ?err, "connection error");
        }
    });
    Ok(())
}

impl Client {
    /// Checks connection status
    pub const fn is_connected(&self) -> bool {
        matches!(self.conn_state, ConnectionStatus::CONNECTED)
    }

    /// Get the server version (important for checking feature flags for
    /// different versions)
    pub const fn server_version(&self) -> ServerVersion { self.server_version }

    /// Sets server logging level
    #[instrument(skip(self))]
    pub async fn set_server_log_level(&mut self, log_level: ServerLogLevel) -> Result<()> {
        ////The pub default detail level is ERROR. For more details, see API
        ////        Logging.
        debug!("set_server_log_level -- log_level: {:?}", log_level);
        let frame = Api::SetServerLoglevel { log_level };

        debug!(request = ?frame);

        // Write the frame to the socket
        self.writer.write_frame(&frame.into_frame()).await?;
        Ok(())
    }

    pub fn subscribe_message_updates(&mut self) -> Receiver<TwsApiMessage> {
        self.message_tracker.clone()
    }

    //----------------------------------------------------------------------------------------------
    /// Gets the connection time
    // pub fn tws_connection_time(&mut self) -> String {
    //    //"""Returns the time the API client made a connection to TWS."""

    //    self.conn_time.clone()
    //}

    //----------------------------------------------------------------------------------------------
    // Request the current time according to TWS or IB Gateway
    pub async fn request_current_time(&mut self) -> Result<()> {
        let frame = Api::RequestCurrentTime;

        debug!(request = ?frame);

        // Write the frame to the socket
        self.writer.write_frame(&frame.into_frame()).await?;
        Ok(())
    }

    //----------------------------------------------------------------------------------------------
    /// Disconnect from TWS
    pub fn disconnect(&self) -> Result<()> {
        if !self.is_connected() {
            info!("Already disconnected...");
            return Ok(());
        }
        error!("Disconnect requested.  Shutting down stream...");
        let _ = self.notify_shutdown.send(())?;
        Ok(())
    }

    fn get_next_req_id(&mut self) -> usize {
        let req_id = self.next_req_id.get_mut();
        let id = *req_id;
        *req_id += 1;
        id
    }
}
/// Per-connection handler. Reads requests from `connection` and applies the
/// commands to `db`.
//#[derive(Debug)]
struct Handler {
    /// Shared database handle.
    ///
    /// When a command is received from `connection`, it is applied with `db`.
    /// The implementation of the command is in the `cmd` module. Each command
    /// will need to interact with `db` in order to complete the work.
    // db: Db,
    server_version: ServerVersion,
    /// The TCP connection decorated with the redis protocol encoder / decoder
    /// implemented using a buffered `TcpStream`.
    ///
    /// When `Listener` receives an inbound connection, the `TcpStream` is
    /// passed to `Connection::new`, which initializes the associated buffers.
    /// `Connection` allows the handler to operate at the "frame" level and keep
    /// the byte level protocol parsing details encapsulated in `Connection`.
    reader:         Reader,

    /// Max connection semaphore.
    ///
    /// When the handler is dropped, a permit is returned to this semaphore. If
    /// the listener is waiting for connections to close, it will be notified of
    /// the newly available permit and resume accepting connections.
    // limit_connections: Arc<Semaphore>,

    /// Listen for shutdown notifications.
    ///
    /// A wrapper around the `broadcast::Receiver` paired with the sender in
    /// `Listener`. The connection handler processes requests from the
    /// connection until the peer disconnects **or** a shutdown notification is
    /// received from `shutdown`. In the latter case, any in-flight work being
    /// processed for the peer is continued until it reaches a safe state, at
    /// which point the connection is terminated.
    shutdown:      Shutdown,
    /// track order ids request and send the result to the corresponsing
    /// receivers
    order_id_reqs: VecDeque<mpsc::Sender<Option<OrderId>>>,

    /// Not used directly. Instead, when `Handler` is dropped...?
    //_shutdown_complete: mpsc::Sender<()>,
    subscribe_handler_rx: mpsc::Receiver<Request>,
    /// track order details request and send the result to the corresponsing
    /// receivers
    requests:             HashMap<usize, mpsc::Sender<ResponseWithId<ContractDetails>>>,

    // track market data request, send the incomming frames to the corresponding receivers
    // ticker_reqs: HashMap<usize, mpsc::Sender<Option<contract::ContractDetails>>>,
    // register sender for open orders
    account_tracker_tx:        Sender<AccountData>,
    account_update_tracker_tx: Sender<AccountLastUpdate>,

    // track tick messages, send incoming frames to the receiver
    market_data_tracker_tx: MarketDataTrackerSender,

    // account_tracker_tx: AccountSender,
    /// send the trade related result to the corLastUpdate
    /// receivers
    order_tracker_tx: OrderTrackerSender,

    contract_details_events_tx: Sender<ResponseWithId<ContractDetails>>,

    message_events_tx: Sender<TwsApiMessage>,
}

impl Handler {
    /// Process a single connection.
    ///
    /// Request frames are read from the socket and processed. Responses are
    /// written back to the socket.
    ///
    /// Currently, pipelining is not implemented. Pipelining is the ability to
    /// process more than one request concurrently per connection without
    /// interleaving frames. See for more details:
    /// https://redis.io/topics/pipelining
    ///
    /// When the shutdown signal is received, the connection is processed until
    /// it reaches a safe state, at which point it is terminated.
    #[instrument(level = "debug", skip(self))]
    async fn run(&mut self) -> Result<()> {
        // As long as the shutdown signal has not been received, try to read a
        // new request frame.
        while !self.shutdown.is_shutdown() {
            // While reading a request frame, also listen for the shutdown
            // signal.
            let maybe_frame = tokio::select! {
                res = self.reader.read_frame(Some(self.server_version)) => res?,
                Some(request) = self.subscribe_handler_rx.recv() => {
                match request {
                    Request::OrderId {sender } => {
                        self.order_id_reqs.push_back(sender);
                        },
                    Request::RequestWithId { req_id, sender } => {
                        debug!("requestwithid:\t{}",req_id);
                        self.requests.insert(req_id, sender);
                        },
                    }
                    None
                }
                ,
                _ = self.shutdown.recv() => {
                    tracing::error!("Shutting down!");
                    // If a shutdown signal is received, return from `run`.
                    // This will result in the task terminating.
                    return Ok(());
                }
            };

            // If `None` is returned from `read_frame()` then the peer closed
            // the socket. There is no further work to do and the task can be
            // terminated.
            if let Some(frame) = maybe_frame {
                match frame {
                    IBFrame::AccountSummary(data) => {
                        self.account_tracker_tx.send(data)?;
                    },

                    IBFrame::AccountValue(data) => {
                        self.account_tracker_tx.send(data)?;
                    },

                    IBFrame::PortfolioValue(position) => debug!("got position:\t{:?}", position),

                    IBFrame::AccountUpdateTime(time) => {
                        self.account_update_tracker_tx.send(time)?;
                    },

                    IBFrame::AccountUpdateEnd(_) => {},

                    IBFrame::CurrentTime(dtime) => {
                        self.message_events_tx
                            .send(TwsApiMessage::ServerTime(dtime))?;
                    },

                    IBFrame::OrderId(id) => {
                        while let Some(sender) = self.order_id_reqs.pop_front() {
                            sender.send(Some(id)).await?;
                            // self.order_id_reqs.remove_entry(&_id);
                        }
                        self.order_tracker_tx.order_id_tx.send(id)?;
                    },

                    IBFrame::ContractDetails {
                        req_id: id,
                        contract_details: details,
                    } => {
                        debug!("got contract details with req_id:\t{}", id);
                        if let Some(sender) = self.requests.get(&id) {
                            debug!("got sender for req_id:\t{}", id);
                            sender
                                .send(ResponseWithId {
                                    req_id:   id,
                                    response: Some(details.clone()),
                                })
                                .await?;
                        }
                        self.contract_details_events_tx.send(ResponseWithId {
                            req_id:   id,
                            response: Some(details.clone()),
                        })?;
                    },

                    IBFrame::ContractDetailsEnd(req_id) => {
                        match self.requests.remove_entry(&req_id) {
                            Some((_, sender)) => {
                                sender
                                    .send(ResponseWithId {
                                        req_id,
                                        response: None,
                                    })
                                    .await?;
                            },
                            None => {
                                debug!("No pending contract details request for req_id {}", req_id);
                            },
                        };
                    },
                    IBFrame::OpenOrder(order_information) => {
                        let _order_id = order_information.order.order_id;
                        let state = OrderState {
                            order_id: order_information.order.order_id,
                            perm_id: order_information.order.perm_id,
                            ..order_information.order_state
                        };
                        self.order_tracker_tx.order_state_tx.send(state)?;
                        self.order_tracker_tx
                            .order_tx
                            .send(order_information.order)?;
                    },
                    IBFrame::CompletedOrder(order_information) => {
                        let _order_id = order_information.order.order_id;
                        let state = OrderState {
                            order_id: order_information.order.order_id,
                            perm_id: order_information.order.perm_id,
                            ..order_information.order_state
                        };
                        debug!(
                            "completed order:\norder:{:?}\nstate:{:#?}",
                            &order_information.order, &state
                        );
                        self.order_tracker_tx.order_state_tx.send(state)?;
                        self.order_tracker_tx
                            .order_tx
                            .send(order_information.order)?;
                    },
                    IBFrame::Execution(execution) => {
                        self.order_tracker_tx.executions_tx.send(execution)?;
                    },
                    IBFrame::CommissionReport(commission) => {
                        self.order_tracker_tx
                            .commission_reports_tx
                            .send(commission)?;
                    },
                    IBFrame::OrderStatus(order_status) => {
                        self.order_tracker_tx.order_status_tx.send(order_status)?;
                    },
                    IBFrame::Tick(tick) => {
                        debug!("got tick: {:#?}", tick);
                        self.market_data_tracker_tx.tick_by_tick_tx.send(tick)?;
                    },
                    IBFrame::HistoricalBars(bars) => {
                        self.market_data_tracker_tx.historical_bars_tx.send(bars)?;
                    },
                    IBFrame::HistoricalSchedule(schedule) => {
                        self.market_data_tracker_tx
                            .historical_schedule_tx
                            .send(schedule)?;
                    },
                    IBFrame::RealtimeBar(bar) => {
                        self.market_data_tracker_tx.bars_tx.send(bar)?;
                    },
                    IBFrame::HistoricalTicks(tick) => {
                        self.market_data_tracker_tx.historical_ticks_tx.send(tick)?;
                    },
                    IBFrame::HeadTimestamp(timestamp) => {
                        self.market_data_tracker_tx
                            .head_timestamp_tx
                            .send(timestamp)?;
                    },
                    IBFrame::Error {
                        req_id,
                        status,
                        message,
                    } => {
                        error!("id:{}\tcode:{}\t{:#?}", req_id, status, message);
                        let req_id = if req_id < 0 {
                            None
                        } else {
                            Some(req_id as usize)
                        };
                        self.message_events_tx.send(TwsApiMessage::TwsError {
                            req_id,
                            status,
                            message,
                        })?;
                    },
                    // TODO: Implement missing IBFrames
                    IBFrame::AccountCode(_) => (),
                    IBFrame::OpenOrderEnd => (),
                    IBFrame::ServerVersion {
                        server_version: _,
                        connection_time: _,
                    } => (),
                    IBFrame::NotImplemented => (),
                    // _ => (),
                }
            };

            // Convert the redis frame into a command struct. This returns an
            // error if the frame is not a valid redis command or it is an
            // unsupported command.
            // let cmd = Command::from_frame(frame)?;

            // Logs the `cmd` object. The syntax here is a shorthand provided by
            // the `tracing` crate. It can be thought of as similar to:
            //
            // ```
            // debug!(cmd = format!("{:?}", cmd));
            // ```
            //
            // `tracing` provides structured logging, so information is "logged"
            // as key-value pairs.
            // debug!(?cmd);

            // Perform the work needed to apply the command. This may mutate the
            // database state as a result.
            //
            // The connection is passed into the apply function which allows the
            // command to write response frames directly to the connection. In
            // the case of pub/sub, multiple frames may be send back to the
            // peer.
            // cmd.apply(&self.db, &mut self.connection, &mut self.shutdown)
            //     .await?;
        }

        Ok(())
    }
}

impl Drop for Handler {
    fn drop(&mut self) {
        // Add a permit back to the semaphore.
        //
        // Doing so unblocks the listener if the max number of
        // connections has been reached.
        //
        // This is done in a `Drop` implementation in order to guarantee that
        // the permit is added even if the task handling the connection panics.
        // If `add_permit` was called at the end of the `run` function and some
        // bug causes a panic. The permit would never be returned to the
        // semaphore.
        // self.limit_connections.add_permits(1);
    }
}
