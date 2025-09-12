use std::str::{FromStr, Split};

use fastnum::D256;

use crate::{enums::*,
            ib_frame::{ParseError, ParseIbkrFrame, ParseResult},
            utils::ib_message::{decode, Decodable, Encodable},
            OrderId,
            ServerVersion,
            TimeStamp};

#[derive(Default, Debug, Clone)]
/// Provides an active order's current state.
pub struct OrderState {
    pub order_id:                            Option<OrderId>,
    pub perm_id:                             i64,
    /// The order's current status
    pub status:                              OrderStatus, // Option<String>,
    /// The account's current initial margin.
    pub init_margin_before:                  Option<D256>,
    /// The account's current maintenance margin
    pub maint_margin_before:                 Option<D256>,
    /// The account's current equity with loan
    pub init_margin_change:                  Option<D256>,
    /// The change of the account's initial margin.
    pub equity_with_loan_value_before:       Option<D256>,
    /// The change of the account's maintenance margin
    pub maint_margin_change:                 Option<D256>,
    /// The change of the account's equity with loan
    pub equity_with_loan_change:             Option<D256>,
    /// The order's impact on the account's initial margin.
    pub init_margin_after:                   Option<D256>,
    /// The order's impact on the account's maintenance margin
    pub maint_margin_after:                  Option<D256>,
    /// Shows the impact the order would have on the account's equity with loan
    pub equity_with_loan_after:              Option<D256>,
    /// The order's generated commission.
    pub commission_and_fees:                 Option<D256>,
    // The execution's minimum commission.
    pub min_commission_and_fees:             Option<D256>,
    /// The executions maximum commission.
    pub max_commission_and_fees:             Option<D256>,
    /// The generated commission currency
    pub commission_and_fees_currency:        Option<String>,
    pub margin_currency:                     Option<String>,
    /// The account’s expected initial margin outside of regular trading hours.
    pub init_margin_before_outside_rth:      Option<D256>,
    /// The account’s expected maintenance margin outside of regular trading
    /// hours.
    pub maint_margin_before_outside_rth:     Option<D256>,
    /// The account’s expected equity with loan outside of regular trading
    /// hours.
    pub equity_with_loan_before_outside_rth: Option<D256>,
    /// The expected change of the account’s initial margin outside of regular
    /// trading hours.
    pub init_margin_change_outside_rth:      Option<D256>,
    /// The expected change of the account’s maintenance margin outside of
    /// regular trading hours.
    pub maint_margin_change_outside_rth:     Option<D256>,
    /// The expected change of the account’s equity with loan outside of regular
    /// trading hours.
    pub equity_with_loan_change_outside_rth: Option<D256>,
    /// The order’s expected impact on the account’s initial margin outside of
    /// regular trading hours.
    pub init_margin_after_outside_rth:       Option<D256>,
    /// The order’s expected impact on the account’s maintenance margin outside
    /// of regular trading hours.
    pub maint_margin_after_outside_rth:      Option<D256>,
    /// Shows the expected impact the order would have on the account’s equity
    /// with loan outside of regular trading hours.
    pub equity_with_loan_after_outside_rth:  Option<D256>,
    pub suggested_size:                      Option<D256>,
    pub reject_reason:                       Option<String>,
    /// Order allocations
    pub order_allocations:                   Option<Vec<OrderAllocation>>,
    /// If the order is warranted, a descriptive message will be provided.
    pub warning_text:                        Option<String>,
    pub completed_time:                      Option<TimeStamp>,
    pub completed_status:                    Option<String>,
}

#[derive(Debug, Clone)]
/// The OrderAllocation class to denote an advisor’s allocations while trading
/// subaccounts.
pub struct OrderAllocation {
    /// References the Account ID, i.e. U1234567, being allocated to.
    pub account:              String,
    /// References the current position of the account being allocated to.
    pub position:             Option<D256>,
    /// States the full position increase intended by the current trade.
    pub position_desired:     Option<D256>,
    /// References the increase to position from the current trade. Unless the
    /// order is partially filled, this should reflect the PositionDesired
    /// value.
    pub position_after:       Option<D256>,
    /// Reference the quantity to increase by based on allocation.
    pub position_alloc_qty:   Option<D256>,
    /// References the maximum allowed quantity increase.
    pub position_allowed_qty: Option<D256>,
    /// Denotes whether the order is a monetary allocation (true) or whole share
    /// allocation (false).
    pub is_monetary:          bool,
}

impl ParseIbkrFrame for OrderState {
    #[allow(clippy::cognitive_complexity)]
    fn try_parse_frame(
        msg_id: Incoming,
        server_version: Option<ServerVersion>,
        it: &mut Split<&str>,
    ) -> ParseResult<Self>
    where
        Self: Sized,
    {
        if !matches!(msg_id, Incoming::OpenOrder) {
            return Err(ParseError::UnexpectedMessage);
        }
        let server_version = server_version.ok_or(ParseError::MissingServerVersion)?;
        let mut order_state = Self {
            status: decode(it)?.unwrap(),

            init_margin_before: decode(it)?,
            maint_margin_before: decode(it)?,
            equity_with_loan_value_before: decode(it)?,
            init_margin_change: decode(it)?,
            maint_margin_change: decode(it)?,
            equity_with_loan_change: decode(it)?,

            init_margin_after: decode(it)?,
            maint_margin_after: decode(it)?,
            equity_with_loan_after: decode(it)?,
            commission_and_fees: decode(it)?,
            min_commission_and_fees: decode(it)?,
            max_commission_and_fees: decode(it)?,
            commission_and_fees_currency: decode(it)?,

            margin_currency: decode(it)?,
            init_margin_before_outside_rth: decode(it)?,
            maint_margin_before_outside_rth: decode(it)?,
            equity_with_loan_before_outside_rth: decode(it)?,
            init_margin_change_outside_rth: decode(it)?,
            maint_margin_change_outside_rth: decode(it)?,
            equity_with_loan_change_outside_rth: decode(it)?,
            init_margin_after_outside_rth: decode(it)?,
            maint_margin_after_outside_rth: decode(it)?,
            equity_with_loan_after_outside_rth: decode(it)?,
            suggested_size: decode(it)?,
            reject_reason: decode(it)?,

            ..Default::default()
        };

        let order_allocations_count: Option<usize> = decode(it)?;
        if let Some(n) = order_allocations_count {
            if n > 0 {
                let mut allocations = Vec::with_capacity(n);
                for _i in 0..n {
                    let order_allocation = OrderAllocation {
                        account:              decode(it)?.unwrap(),
                        position:             decode(it)?,
                        position_desired:     decode(it)?,
                        position_after:       decode(it)?,
                        position_alloc_qty:   decode(it)?,
                        position_allowed_qty: decode(it)?,
                        is_monetary:          decode(it)?.unwrap(),
                    };
                    allocations.push(order_allocation);
                }
                order_state.order_allocations = Some(allocations);
            }
        }

        order_state.warning_text = decode(it)?;

        Ok(order_state)
    }
}

#[derive(Debug, Clone)]
pub struct OrderStatusUpdate {
    pub order_id:        OrderId,
    pub status:          OrderStatus,
    pub filled:          D256,
    pub remaining:       D256,
    pub avg_fill_price:  D256,
    pub perm_id:         i64,
    pub parent_id:       OrderId,
    pub last_fill_price: D256,
    pub client_id:       usize,
    pub why_held:        Option<String>,
}
impl ParseIbkrFrame for OrderStatusUpdate {
    fn try_parse_frame(
        msg_id: Incoming,
        server_version: Option<ServerVersion>,
        it: &mut Split<&str>,
    ) -> ParseResult<Self>
    where
        Self: Sized,
    {
        if !matches!(msg_id, Incoming::OrderStatus) {
            return Err(ParseError::UnexpectedMessage);
        }
        tracing::debug!("decode OrderStatus");
        Ok(Self {
            order_id:        decode(it)?.unwrap(),
            status:          decode(it)?.unwrap(),
            filled:          decode(it)?.unwrap(),
            remaining:       decode(it)?.unwrap(),
            avg_fill_price:  decode(it)?.unwrap(),
            perm_id:         decode(it)?.unwrap(),
            parent_id:       decode(it)?.unwrap(),
            last_fill_price: decode(it)?.unwrap(),
            client_id:       decode(it)?.unwrap(),
            why_held:        decode(it)?,
        })
    }
}
#[derive(Debug, Clone, Copy, Default)]
pub enum OrderStatus {
    /// indicates that you have transmitted the order, but have not yet received
    /// confirmation that it has been accepted by the order destination.
    /// This order status is not sent by TWS and should be explicitly set by the
    /// API developer when an order is submitted.
    PendingSubmit,

    /// PendingCancel - indicates that you have sent a request to cancel the
    /// order but have not yet received cancel confirmation from the order
    /// destination. At this point, your order is not confirmed canceled.
    /// You may still receive an execution while your cancellation request
    /// is pending. This order status is not sent by TWS and should be
    /// explicitly set by the API developer when an order is canceled.
    PendingCancel,

    /// indicates that a simulated order type has been accepted by the IB system
    /// and that this order has yet to be elected. The order is held in the
    /// IB system (and the status remains DARK BLUE) until the election
    /// criteria are met. At that time the order is transmitted to the order
    /// destination as specified (and the order status color will change).
    PreSubmitted,

    /// indicates that your order has been accepted at the order destination and
    /// is working.
    Submitted,

    /// indicates that the balance of your order has been confirmed canceled by
    /// the IB system. This could occur unexpectedly when IB or the
    /// destination has rejected your order.
    Cancelled,

    /// The order has been completely filled.
    Filled,

    /// The Order is inactive
    Inactive,

    /// The order is Partially Filled
    PartiallyFilled,

    /// Api Pending
    ApiPending,

    /// Api Cancelled
    ApiCancelled,

    /// Indicates that there is an error with this order
    /// This order status is not sent by TWS and should be explicitly set by the
    /// API developer when an error has occured.
    Error,

    /// No Order Status
    #[default]
    None,
}

impl Encodable for OrderStatus {
    fn encode(&self) -> String {
        match self {
            OrderStatus::PendingSubmit => "PendingSubmit\0",

            OrderStatus::PendingCancel => "PendingCancel\0",

            OrderStatus::PreSubmitted => "PreSubmitted\0",

            OrderStatus::Submitted => "Submitted\0",

            OrderStatus::Cancelled => "Cancelled\0",

            OrderStatus::Filled => "Filled\0",

            OrderStatus::Inactive => "Inactive\0",

            OrderStatus::PartiallyFilled => "PartiallyFilled\0",

            OrderStatus::ApiPending => "ApiPending\0",

            OrderStatus::ApiCancelled => "ApiCancelled\0",

            OrderStatus::Error => "Error\0",

            OrderStatus::None => "\0",
        }
        .to_string()
    }
}

impl FromStr for OrderStatus {
    type Err = ParseEnumError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "PendingSubmit" => Ok(OrderStatus::PendingSubmit),

            "PendingCancel" => Ok(OrderStatus::PendingCancel),

            "PreSubmitted" => Ok(OrderStatus::PreSubmitted),

            "Submitted" => Ok(OrderStatus::Submitted),

            "Cancelled" => Ok(OrderStatus::Cancelled),

            "Filled" => Ok(OrderStatus::Filled),

            "Inactive" => Ok(OrderStatus::Inactive),

            "PartiallyFilled" => Ok(OrderStatus::PartiallyFilled),

            "ApiPending" => Ok(OrderStatus::ApiPending),

            "ApiCancelled" => Ok(OrderStatus::ApiCancelled),

            "Error" => Ok(OrderStatus::Error),

            "" => Ok(OrderStatus::None),
            &_ => Err(ParseEnumError),
        }
    }
}

impl Decodable for OrderStatus {}

// #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
// #[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
#[derive(Debug, Clone, Copy)]
pub enum OrderConditionType {
    Price         = 1,
    Time          = 3,
    Margin        = 4,
    Execution     = 5,
    Volume        = 6,
    PercentChange = 7,
}

impl Encodable for OrderConditionType {
    fn encode(&self) -> String {
        match self {
            Self::Price => "1\0",
            Self::Time => "3\0",
            Self::Margin => "4\0",
            Self::Execution => "5\0",
            Self::Volume => "6\0",
            Self::PercentChange => "7\0",
        }
        .to_string()
    }
}

impl FromStr for OrderConditionType {
    type Err = ParseEnumError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" => Ok(Self::Price),
            "3" => Ok(Self::Time),
            "4" => Ok(Self::Margin),
            "5" => Ok(Self::Execution),
            "6" => Ok(Self::Volume),
            "7" => Ok(Self::PercentChange),
            &_ => Err(ParseEnumError),
        }
    }
}
impl Decodable for OrderConditionType {}
