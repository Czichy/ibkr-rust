use std::str::Split;

use rust_decimal::prelude::*;
use rust_decimal_macros::dec;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{contract::{ComboLeg, Contract, DeltaNeutralContract},
            enums::*,
            ib_frame::{ParseError, ParseIbkrFrame, ParseResult},
            order::Order,
            prelude::{ib_message::{decode, Decodable},
                      SoftDollarTier},
            utils::ib_message::Encodable,
            OrderId,
            ServerVersion,
            TimeStamp};

#[derive(Default, Debug, Clone)]
pub struct OrderState {
    pub order_id:                            Option<OrderId>,
    pub perm_id:                             i32,
    pub status:                              OrderStatus, // Option<String>,
    pub init_margin_before:                  Option<Decimal>,
    pub maint_margin_before:                 Option<Decimal>,
    pub init_margin_change:                  Option<Decimal>,
    pub equity_with_loan_value_before:       Option<Decimal>,
    pub maint_margin_change:                 Option<Decimal>,
    pub equity_with_loan_change:             Option<Decimal>,
    pub init_margin_after:                   Option<Decimal>,
    pub maint_margin_after:                  Option<Decimal>,
    pub equity_with_loan_after:              Option<Decimal>,
    pub commission:                          Option<Decimal>,
    pub min_commission:                      Option<Decimal>,
    pub max_commission:                      Option<Decimal>,
    pub commission_currency:                 Option<String>,
    pub margin_currency:                     Option<String>,
    pub init_margin_before_outside_rth:      Option<Decimal>,
    pub maint_margin_before_outside_rth:     Option<Decimal>,
    pub equity_with_loan_before_outside_rth: Option<Decimal>,
    pub init_margin_change_outside_rth:      Option<Decimal>,
    pub maint_margin_change_outside_rth:     Option<Decimal>,
    pub equity_with_loan_change_outside_rth: Option<Decimal>,
    pub init_margin_after_outside_rth:       Option<Decimal>,
    pub maint_margin_after_outside_rth:      Option<Decimal>,
    pub equity_with_loan_after_outside_rth:  Option<Decimal>,
    pub suggested_size:                      Option<Decimal>,
    pub reject_reason:                       Option<String>,
    pub warning_text:                        Option<String>,
    pub completed_time:                      Option<TimeStamp>,
    pub completed_status:                    Option<String>,
}

#[derive(Default, Debug, Clone)]
pub struct OrderInformation {
    pub order:       Order,
    pub order_state: OrderState,
}

#[derive(Debug, Clone)]
pub struct OrderAllocation {
    pub account:              String,
    pub position:             Option<Decimal>,
    pub position_desired:     Option<Decimal>,
    pub position_after:       Option<Decimal>,
    pub position_alloc_qty:   Option<Decimal>,
    pub position_allowed_qty: Option<Decimal>,
    pub is_monetary:          bool,
}
impl ParseIbkrFrame for OrderInformation {
    #[allow(clippy::cognitive_complexity)]
    fn try_parse_frame(
        msg_id: Incoming,
        server_version: Option<ServerVersion>,
        it: &mut Split<&str>,
    ) -> ParseResult<Self>
    where
        Self: Sized,
    {
        if !matches!(msg_id, Incoming::OpenOrder | Incoming::CompletedOrder) {
            return Err(ParseError::UnexpectedMessage);
        }
        let server_version = server_version.ok_or(ParseError::MissingServerVersion)?;
        let completed = matches!(msg_id, Incoming::CompletedOrder);
        tracing::debug!("decode Orders: {:#?}", msg_id);
        let order_id = if !completed { decode(it)? } else { None };
        tracing::debug!("orderid: {:#?}", order_id);
        // decode contract
        let contract = Contract::try_parse_frame(msg_id, Some(server_version), it)?;
        tracing::debug!("contract: {:#?}", &contract);
        let mut order = Order {
            contract,
            order_id, //
            action: decode(it)?.unwrap(),
            total_qty: decode(it)?.unwrap(),
            order_type: decode(it)?.unwrap(),
            lmt_price: decode(it)?,
            aux_price: decode(it)?,
            tif: decode(it)?,
            oca_group: decode(it)?,
            account: decode(it)?,
            open_close: decode(it)?,
            origin: decode(it)?,
            order_ref: decode(it)?,
            client_id: if !completed { decode(it)? } else { None },
            perm_id: decode(it)?.unwrap(),
            outside_rth: decode(it)?.unwrap(),
            hidden: decode(it)?.unwrap(),
            discretionary_amt: decode(it)?.unwrap(),
            good_after_time: decode(it)?,
            fa_group: {
                // skip deprecated sharesAllocation field
                if !completed {
                    it.next();
                }
                decode(it)?
            },
            fa_method: decode(it)?,
            fa_percentage: decode(it)?,
            fa_profile: decode(it)?,
            model_code: decode(it)?,
            good_till_date: decode(it)?,
            // TODO: latest Version 183 comment out
            rule_80_a: if server_version < 183 {
                decode(it)?
            } else {
                Default::default()
            },
            percent_offset: decode(it)?,
            settling_firm: { decode(it)? },

            short_sale_slot: decode(it)?,
            designated_location: decode(it)?,
            exempt_code: decode(it)?,

            auction_strategy: if !completed { decode(it)? } else { None },
            starting_price: decode(it)?,
            stock_ref_price: decode(it)?,
            delta: decode(it)?,
            stock_range_lower: decode(it)?,
            stock_range_upper: decode(it)?,
            display_size: decode(it)?,
            block_order: if !completed { decode(it)? } else { None },
            sweep_to_fill: decode(it)?.unwrap(),
            all_or_none: decode(it)?.unwrap(),
            // all_or_none: decode(it)?.unwrap_or_default(),
            min_qty: decode(it)?,
            oca_type: decode(it)?,
            e_trade_only: if !completed { decode(it)? } else { None },
            firm_quote_only: if !completed { decode(it)? } else { None },
            nbbo_price_cap: if !completed { decode(it)? } else { None },
            parent_id: if !completed { decode(it)? } else { None },
            trigger_method: decode(it)?,
            volatility: decode(it)?,
            volatility_type: decode(it)?,
            delta_neutral_order_type: decode(it)?,
            delta_neutral_aux_price: decode(it)?,
            ..Default::default()
        };
        if order.delta_neutral_order_type.is_some() {
            order.delta_neutral_con_id = decode(it)?.unwrap();
            order.delta_neutral_settling_firm = if !completed { decode(it)? } else { None };
            order.delta_neutral_clearing_account = if !completed { decode(it)? } else { None };
            order.delta_neutral_clearing_intent = if !completed { decode(it)? } else { None };
            order.delta_neutral_open_close = if !completed { decode(it)? } else { None };
            order.delta_neutral_short_sale = decode(it)?.unwrap();
            order.delta_neutral_short_sale_slot = decode(it)?.unwrap();
            order.delta_neutral_designated_location = decode(it)?;
        }
        order.continuous_update = decode(it)?.unwrap();
        order.reference_price_type = decode(it)?;
        order.trail_stop_price = decode(it)?;
        order.trailing_percent = decode(it)?;
        order.basis_points = if !completed { decode(it)? } else { None };
        order.basis_points_type = if !completed { decode(it)? } else { None };
        order.contract.combo_legs_description = decode(it)?;
        let combo_legs_count: Option<usize> = decode(it)?;
        if let Some(n) = combo_legs_count {
            let mut legs = Vec::with_capacity(n);
            for _i in 0..n {
                legs.push(ComboLeg {
                    con_id:              decode(it)?.unwrap(),
                    ratio:               decode(it)?.unwrap(),
                    action:              decode(it)?.unwrap(),
                    exchange:            decode(it)?.unwrap(),
                    open_close:          decode(it)?,
                    shortsale_slot:      decode(it)?,
                    designated_location: decode(it)?,
                    exempt_code:         decode(it)?,
                })
            }
            order.contract.combo_legs = Some(legs);
        }
        let order_combo_legs_count: Option<usize> = decode(it)?;
        if let Some(n) = order_combo_legs_count {
            let mut order_legs: Vec<Option<Decimal>> = Vec::with_capacity(n);
            for _i in 0..n {
                order_legs.push(decode(it)?);
            }
            order.order_combo_legs = Some(order_legs);
        }
        let smart_combo_routing_params_count: Option<usize> = decode(it)?;
        if let Some(n) = smart_combo_routing_params_count {
            let mut _combo_params: Vec<(String, String)> = Vec::with_capacity(n);
            for _i in 0..n {
                _combo_params.push((decode(it)?.unwrap(), decode(it)?.unwrap()));
            }
        }
        order.scale_init_level_size = decode(it)?;
        order.scale_subs_level_size = decode(it)?;
        order.scale_price_increment = decode(it)?;
        if let Some(incr) = order.scale_price_increment {
            if incr > dec!(0.0) {
                order.scale_price_adjust_value = decode(it)?;
                order.scale_price_adjust_interval = decode(it)?;
                order.scale_profit_offset = decode(it)?;
                order.scale_auto_reset = decode(it)?.unwrap();
                order.scale_init_position = decode(it)?;
                order.scale_init_fill_qty = decode(it)?;
                order.scale_random_percent = decode(it)?.unwrap();
            }
        }
        order.hedge_type = decode(it)?;
        if let Some(ht) = &order.hedge_type {
            if *ht != HedgeType::Undefined {
                order.hedge_param = decode(it)?;
            }
        }
        order.opt_out_smart_routing = if !completed { decode(it)? } else { None };
        order.clearing_account = decode(it)?;
        order.clearing_intent = decode(it)?;
        order.not_held = decode(it)?.unwrap();
        let has_delta_neutral_contract: Option<bool> = decode(it)?;
        if let Some(has_dnc) = has_delta_neutral_contract {
            if has_dnc {
                order.contract.delta_neutral_contract = Some(DeltaNeutralContract {
                    con_id: decode(it)?.unwrap(),
                    delta:  decode(it)?.unwrap(),
                    price:  decode(it)?.unwrap(),
                });
            }
        }
        order.algo_strategy = decode(it)?;
        if order.algo_strategy.is_some() {
            let params_count: Option<usize> = decode(it)?;
            if let Some(n) = params_count {
                let mut params: Vec<(String, String)> = Vec::with_capacity(n);
                for _i in 0..n {
                    params.push((decode(it)?.unwrap(), decode(it)?.unwrap()));
                }
                order.algo_params = Some(params);
            }
        }
        order.solicited = decode(it)?.unwrap();
        order.what_if = if !completed { decode(it)? } else { None };
        let mut order_state = OrderState {
            status: decode(it)?.unwrap(),
            init_margin_before: if !completed { decode(it)? } else { None },
            maint_margin_before: if !completed { decode(it)? } else { None },
            equity_with_loan_value_before: if !completed { decode(it)? } else { None },
            init_margin_change: if !completed { decode(it)? } else { None },
            maint_margin_change: if !completed { decode(it)? } else { None },
            equity_with_loan_change: if !completed { decode(it)? } else { None },
            init_margin_after: if !completed { decode(it)? } else { None },
            maint_margin_after: if !completed { decode(it)? } else { None },
            equity_with_loan_after: if !completed { decode(it)? } else { None },
            commission: if !completed { decode(it)? } else { None },
            min_commission: if !completed { decode(it)? } else { None },
            max_commission: if !completed { decode(it)? } else { None },
            commission_currency: if !completed { decode(it)? } else { None },
            margin_currency: if !completed { decode(it)? } else { None },
            init_margin_before_outside_rth: if !completed { decode(it)? } else { None },
            maint_margin_before_outside_rth: if !completed { decode(it)? } else { None },
            equity_with_loan_before_outside_rth: if !completed { decode(it)? } else { None },
            init_margin_change_outside_rth: if !completed { decode(it)? } else { None },
            maint_margin_change_outside_rth: if !completed { decode(it)? } else { None },
            equity_with_loan_change_outside_rth: if !completed { decode(it)? } else { None },
            init_margin_after_outside_rth: if !completed { decode(it)? } else { None },
            maint_margin_after_outside_rth: if !completed { decode(it)? } else { None },
            equity_with_loan_after_outside_rth: if !completed { decode(it)? } else { None },
            suggested_size: if !completed { decode(it)? } else { None },
            reject_reason: if !completed { decode(it)? } else { None },

            ..Default::default()
        };
        let order_allocations_count: Option<usize> = decode(it)?;
        if let Some(n) = order_allocations_count {
            if n > 0 {
                let mut _allocations = Vec::with_capacity(n);
                for _i in 0..n {
                    let order_allocation = OrderAllocation {
                        account:              if !completed {
                            decode(it)?.unwrap()
                        } else {
                            Default::default()
                        },
                        position:             if !completed { decode(it)? } else { None },
                        position_desired:     if !completed { decode(it)? } else { None },
                        position_after:       if !completed { decode(it)? } else { None },
                        position_alloc_qty:   if !completed { decode(it)? } else { None },
                        position_allowed_qty: if !completed { decode(it)? } else { None },
                        is_monetary:          if !completed {
                            decode(it)?.unwrap()
                        } else {
                            Default::default()
                        },
                    };
                    _allocations.push(order_allocation);
                }
            }
        }

        order_state.warning_text = if !completed { decode(it)? } else { None };

        order.randomize_size = decode(it)?.unwrap();
        order.randomize_price = decode(it)?.unwrap();
        if order.order_type == OrderType::PeggedToBenchmark {
            order.reference_contract_id = decode(it)?.unwrap();
            order.is_pegged_change_amount_decrease = decode(it)?.unwrap();
            order.pegged_change_amount = decode(it)?;
            order.reference_change_amount = decode(it)?.unwrap();
            order.reference_exchange_id = decode(it)?;
        }
        let conditions_count: Option<usize> = decode(it)?;
        if let Some(n) = conditions_count {
            if n > 0 {
                let mut conditions = Vec::with_capacity(n);
                for _i in 0..n {
                    conditions.push(decode(it)?.unwrap());
                }
                order.conditions = Some(conditions);
                order.conditions_ignore_rth = decode(it)?.unwrap();
                order.conditions_cancel_order = decode(it)?.unwrap();
            }
        }
        if !completed {
            order.adjusted_order_type = decode(it)?;
            order.trigger_price = decode(it)?;
        }
        order.trail_stop_price = decode(it)?;
        order.lmt_price_offset = decode(it)?;
        if !completed {
            order.adjusted_stop_price = decode(it)?;
            order.adjusted_stop_limit_price = decode(it)?;
            order.adjusted_trailing_amount = decode(it)?;
            order.adjustable_trailing_unit = decode(it)?.unwrap();
            let name: Option<String> = decode(it)?;
            let val: Option<String> = decode(it)?;
            let display_name: Option<String> = decode(it)?;
            if name.is_some() || val.is_some() || display_name.is_some() {
                order.soft_dollar_tier = Some(SoftDollarTier {
                    name,
                    val,
                    display_name,
                })
            }
        }
        order.cash_qty = decode(it)?;
        order.dont_use_auto_price_for_hedge = decode(it)?.unwrap();
        order.is_oms_container = decode(it)?.unwrap();

        if !completed {
            order.discretionary_up_to_limit_price = decode(it)?.unwrap();
            order.use_price_mgmt_algo = decode(it)?;
        }

        if completed {
            order.auto_cancel_date = decode(it)?;
            order.filled_quantity = decode(it)?;
            order.ref_futures_con_id = decode(it)?;
            order.auto_cancel_parent = decode(it)?.unwrap();
            order.shareholder = decode(it)?;
            order.imbalance_only = decode(it)?.unwrap();
            order.route_marketable_to_bbo = decode(it)?.unwrap();
            order.parent_perm_id = decode(it)?;
            order_state.completed_time = decode(it)?;
            order_state.completed_status = decode(it)?;
        }

        Ok(OrderInformation { order, order_state })
    }
}

#[derive(Debug, Clone)]
pub struct OrderStatusUpdate {
    pub order_id:        OrderId,
    pub status:          OrderStatus,
    pub filled:          Decimal,
    pub remaining:       Decimal,
    pub avg_fill_price:  Decimal,
    pub perm_id:         i32,
    pub parent_id:       OrderId,
    pub last_fill_price: Decimal,
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
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
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

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
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
