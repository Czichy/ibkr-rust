use std::str::Split;

use fastnum::D256;

use super::{AdjustedOrder,
            FinancialAdvisor,
            Mifid2,
            OrderConditionType,
            OrderState,
            ScaleOrderParameter,
            VolatilityOrderParameter};
use crate::{cmd::UsePriceMgmtAlgo,
            contract::{ComboLegs, Contract, DeltaNeutralContract, SecType},
            enums::*,
            ib_frame::{ParseError, ParseIbkrFrame, ParseResult},
            utils::ib_message::{decode, Encodable},
            OrderId,
            ServerVersion,
            TimeStamp};

#[derive(Default, Debug, Clone)]
pub struct SoftDollarTier {
    pub name:         Option<String>,
    pub val:          Option<String>,
    pub display_name: Option<String>,
}

#[derive(Default, Debug, Clone)]
pub struct Order {
    pub order_id:    Option<OrderId>,
    pub contract:    Contract,
    pub order:       OrderData,
    pub order_state: OrderState,
}

#[derive(Debug, Clone, Default)]
/// Order describes the order.
pub struct OrderData {
    /// The API client's order id.
    pub order_id:  Option<OrderId>,
    /// The API client id which placed the order.
    pub client_id: Option<usize>,
    /// The Host order identifier.
    pub perm_id:   i64,

    /// Identifies the side.
    /// Generally available values are BUY and SELL.
    /// Additionally, SSHORT and SLONG are available in some
    /// institutional-accounts only. For general account types, a SELL order
    /// will be able to enter a short position automatically if the order
    /// quantity is larger than your current long position. SSHORT is only
    /// supported for institutional account configured with Long/Short account
    /// segments or clearing with a separate account. SLONG is available in
    /// specially-configured institutional accounts to indicate that long
    /// position not yet delivered is being sold.
    pub action:     Action,
    /// The number of positions being bought/sold.
    pub total_qty:  D256,
    /// The order's type.
    pub order_type: OrderType,
    /// The LIMIT price.
    /// Used for limit, stop-limit and relative orders. In all other cases
    /// specify zero. For relative orders with no limit price, also specify
    /// zero.
    pub lmt_price:  Option<D256>,
    /// Generic field to contain the stop price for STP LMT orders, trailing
    /// amount, etc.
    pub aux_price:  Option<D256>,

    // extended order fields
    /// The time in force.
    pub tif:                             Option<TimeInForce>,
    pub active_start_time:               Option<String>,
    pub active_stop_time:                Option<String>,
    /// One-Cancels-All group identifier
    pub oca_group:                       Option<String>,
    /// Tells how to handle remaining orders in an OCA group when one order or
    /// part of an order executes.
    pub oca_type:                        Option<OCAType>,
    /// The order reference.
    /// Intended for institutional customers only, although all customers may
    /// use it to identify the API client that sent the order when multiple API
    /// clients are running.
    pub order_ref:                       Option<String>,
    pub transmit:                        bool,
    pub parent_id:                       Option<i32>,
    pub block_order:                     Option<bool>,
    pub sweep_to_fill:                   bool,
    pub display_size:                    Option<i32>,
    pub trigger_method:                  Option<TriggerMethod>,
    pub outside_rth:                     bool,
    pub hidden:                          bool,
    pub good_after_time:                 Option<String>,
    pub good_till_date:                  Option<String>,
    pub override_percentage_constraints: bool,
    pub rule_80_a:                       Option<Rule80A>,
    pub all_or_none:                     bool,
    pub min_qty:                         Option<i32>,
    pub percent_offset:                  Option<D256>,
    pub trail_stop_price:                Option<D256>,
    pub trailing_percent:                Option<D256>,

    // financial advisor fields
    pub financial_advisor: Option<FinancialAdvisor>,

    // institutional (i.e. non-cleared) only
    pub open_close:          Option<OrderOpenClose>,
    pub origin:              Option<Origin>,
    pub short_sale_slot:     Option<ShortSaleSlot>,
    pub designated_location: Option<String>,
    pub exempt_code:         Option<i32>,

    // SMART routing fields
    pub discretionary_amt:     D256,
    pub e_trade_only:          Option<bool>,
    pub firm_quote_only:       Option<bool>,
    pub nbbo_price_cap:        Option<D256>,
    pub opt_out_smart_routing: Option<bool>,

    // BOX exchange order fields
    pub auction_strategy: Option<AuctionStrategy>,
    pub starting_price:   Option<D256>,
    pub stock_ref_price:  Option<D256>,
    pub delta:            Option<D256>,

    // Pegged to stock and VOL order fields
    pub stock_range_lower: Option<D256>,
    pub stock_range_upper: Option<D256>,

    pub randomize_size:  bool,
    pub randomize_price: bool,

    // Volatility order fields
    pub volatility_order_parameter: VolatilityOrderParameter,

    // Combo order fields
    pub basis_points:      Option<D256>,
    pub basis_points_type: Option<BasisPointsType>,

    // Scale order fields
    pub scale_order_parameter: ScaleOrderParameter,

    // Hedge order fields
    pub hedge_type:  Option<HedgeType>,
    pub hedge_param: Option<String>, // 'beta=X' value for beta hedge, 'ratio=Y' for pair hedge

    // Clearing info
    pub account:          Option<String>,
    pub settling_firm:    Option<String>,
    pub clearing_account: Option<String>,
    pub clearing_intent:  Option<ClearingIntent>,

    // Algo order fields
    pub algo_strategy:              Option<String>,
    pub algo_params:                Option<Vec<(String, String)>>,
    pub smart_combo_routing_params: Option<Vec<(String, String)>>,
    pub algo_id:                    Option<String>,

    // What-if
    pub what_if: Option<bool>,

    // Not held
    pub not_held:  bool,
    pub solicited: bool,

    // Models
    pub model_code: Option<String>,

    // Order combo legs
    pub order_combo_legs:   Option<Vec<Option<D256>>>,
    pub order_misc_options: Option<Vec<(String, String)>>,

    // VER PEG2BENCH fields
    pub reference_contract_id:            i32,
    pub pegged_change_amount:             Option<D256>,
    pub is_pegged_change_amount_decrease: bool,
    pub reference_change_amount:          f64,
    pub reference_exchange_id:            Option<String>,

    pub adjusted_order: Option<AdjustedOrder>,

    pub conditions:              Option<Vec<OrderConditionType>>,
    pub conditions_cancel_order: bool,
    pub conditions_ignore_rth:   bool,

    // ext operator
    pub ext_operator: Option<String>,

    pub soft_dollar_tier: Option<SoftDollarTier>,

    pub cash_qty: Option<D256>,

    pub mifid_2: Option<Mifid2>,

    pub dont_use_auto_price_for_hedge:   bool,
    pub is_oms_container:                bool,
    pub discretionary_up_to_limit_price: bool,
    pub auto_cancel_date:                Option<String>,
    pub filled_quantity:                 Option<D256>,
    pub ref_futures_con_id:              Option<i32>,
    pub auto_cancel_parent:              bool,
    pub shareholder:                     Option<String>,
    pub imbalance_only:                  bool,
    pub route_marketable_to_bbo:         bool,
    pub parent_perm_id:                  Option<usize>,
    /// Specifies wether to use Price Management Algo. CTCI users only.
    pub use_price_mgmt_algo:             Option<UsePriceMgmtAlgo>,
    /// Specifies the duration of the order. Format: yyyymmdd hh:mm:ss TZ.
    /// For GTD orders.
    pub duration:                        Option<TimeStamp>,
    /// Value must be positive, and it is number of seconds that SMART order
    /// would be parked for at IBKRATS before being routed to exchange.
    pub post_to_ats:                     Option<i32>,
    /// Accepts a list with parameters obtained from advancedOrderRejectJson.
    pub advanced_error_override:         Option<String>,

    /// Used by brokers and advisors when manually entering, modifying or
    /// cancelling orders at the direction of a client.  used when
    /// allocating orders to specific groups or accounts. Excluding "All"
    /// group.</i>
    pub manual_order_time: Option<String>,

    /// Defines the minimum trade quantity to fill. <i>For IBKRATS orders.</i>
    pub min_trade_qty: Option<i32>,

    /// Defines the minimum size to compete. <i>For IBKRATS orders.</i>
    pub min_compete_size: Option<i32>,

    /// Dpecifies the offset Off The Midpoint that will be applied to the order.
    /// <i>For IBKRATS orders.</i>
    pub compete_against_best_offset: Option<D256>,

    /// This offset is applied when the spread is an even number of cents wide.
    /// This offset must be in whole-penny increments or zero. <i>For IBKRATS
    /// orders.</i>
    pub mid_offset_at_whole: Option<D256>,

    /// This offset is applied when the spread is an odd number of cents wide.
    /// This offset must be in half-penny increments. <i>For IBKRATS orders.</i>
    pub mid_offset_at_half: Option<D256>,

    /// Customer account
    pub customer_account: Option<String>,

    /// Professional customer
    pub professional_customer: bool,

    /// Bond accrued interest
    pub bond_accrued_interest: Option<String>,

    /// Include Overnight
    pub include_overnight: bool,

    /// Manual Order Indicator
    pub manual_order_indicator: Option<i32>,

    /// Submitter
    pub submitter: Option<String>,
}

impl OrderData {
    #[allow(clippy::missing_const_for_fn)]
    fn new() -> Self {
        OrderData {
            transmit: true,
            open_close: Some(OrderOpenClose::Open),
            origin: Some(Origin::Customer),
            exempt_code: Some(-1),
            auction_strategy: Some(AuctionStrategy::NoAuctionStrategy),
            ..Default::default()
        }
    }

    // pub fn market(contract: Contract, action: Action, qty: D256) -> Self {
    //     let mut order = OrderData::new();
    //     order.action = action;
    //     order.contract = contract;
    //     order.total_qty = qty;
    //     order
    // }

    // pub fn market_on_close(contract: Contract, action: Action, qty: D256) ->
    // Self {     let mut order = OrderData::new();
    //     order.action = action;
    //     order.contract = contract;
    //     order.total_qty = qty;
    //     order.order_type = OrderType::MarketOnClose;
    //     order
    // }

    // pub fn relative_market(contract: Contract, action: Action, qty: D256) ->
    // Self {     let mut order = OrderData::new();
    //     order.action = action;
    //     order.contract = contract;
    //     order.total_qty = qty;
    //     order.order_type = OrderType::RelativeMarket;
    //     order
    // }

    // pub fn limit(
    //     contract: Contract,
    //     action: Action,
    //     qty: D256,
    //     lmt: D256,
    //     tif: TimeInForce,
    // ) -> Self {
    //     let mut order = OrderData::new();
    //     order.action = action;
    //     order.contract = contract;
    //     order.total_qty = qty;
    //     order.order_type = OrderType::Limit;
    //     order.lmt_price = Some(lmt);
    //     order.tif = Some(tif);
    //     order
    // }
}

impl Encodable for Order {
    fn encode(&self) -> String {
        let mut code = String::new();
        code.push_str(&self.contract.encode_for_order());
        code.push_str(&self.order.action.encode());
        code.push_str(&self.order.total_qty.encode());
        code.push_str(&self.order.order_type.encode());
        code.push_str(&self.order.lmt_price.encode());
        code.push_str(&self.order.aux_price.encode());
        code.push_str(&self.order.tif.encode());
        code.push_str(&self.order.oca_group.encode());
        code.push_str(&self.order.account.encode());
        code.push_str(&self.order.open_close.encode());
        code.push_str(&self.order.origin.encode());
        code.push_str(&self.order.order_ref.encode());
        code.push_str(&self.order.transmit.encode());
        code.push_str(&self.order.parent_id.encode());
        code.push_str(&self.order.block_order.encode());
        code.push_str(&self.order.sweep_to_fill.encode());
        code.push_str(&self.order.display_size.encode());
        code.push_str(&self.order.trigger_method.encode());
        code.push_str(&self.order.outside_rth.encode());
        code.push_str(&self.order.hidden.encode());
        let sec = &self.contract.sec_type;
        if *sec == SecType::Combo {
            match &self.contract.combo_legs {
                Some(legs) => {
                    code.push_str(&legs.encode());
                },
                None => code.push_str("0\0"),
            }
            match &self.order.order_combo_legs {
                Some(legs) => {
                    code.push_str(&legs.len().encode());
                    for leg in legs {
                        code.push_str(&leg.encode());
                    }
                },
                None => code.push_str("0\0"),
            }
            match &self.order.smart_combo_routing_params {
                Some(tag_val_list) => {
                    code.push_str(&tag_val_list.len().encode());
                    for tv in tag_val_list {
                        code.push_str(&tv.0.encode());
                        code.push_str(&tv.1.encode());
                    }
                },
                None => code.push_str("0\0"),
            }
        }

        code.push('\0'); // deprecated shares allocation field
        code.push_str(&self.order.discretionary_amt.encode());
        code.push_str(&self.order.good_after_time.encode());
        code.push_str(&self.order.good_till_date.encode());
        code.push_str(&self.order.financial_advisor.encode());
        code.push_str(&self.order.model_code.encode());
        code.push_str(&self.order.short_sale_slot.encode());
        code.push_str(&self.order.designated_location.encode());
        code.push_str(&self.order.exempt_code.encode());
        code.push_str(&self.order.oca_type.encode());
        code.push_str(&self.order.rule_80_a.encode());
        code.push_str(&self.order.settling_firm.encode());
        code.push_str(&self.order.all_or_none.encode());
        code.push_str(&self.order.min_qty.encode());
        code.push_str(&self.order.percent_offset.encode());
        code.push_str(&self.order.e_trade_only.encode());
        code.push_str(&self.order.firm_quote_only.encode());
        code.push_str(&self.order.nbbo_price_cap.encode());
        code.push_str(&self.order.auction_strategy.encode());
        code.push_str(&self.order.starting_price.encode());
        code.push_str(&self.order.stock_ref_price.encode());
        code.push_str(&self.order.delta.encode());
        code.push_str(&self.order.stock_range_lower.encode());
        code.push_str(&self.order.stock_range_upper.encode());
        code.push_str(&self.order.override_percentage_constraints.encode());

        code.push_str(&self.order.volatility_order_parameter.encode());

        code.push_str(&self.order.trail_stop_price.encode());
        code.push_str(&self.order.trailing_percent.encode());
        code.push_str(&self.order.scale_order_parameter.encode());
        code.push_str(&self.order.active_start_time.encode());
        code.push_str(&self.order.active_stop_time.encode());
        code.push_str(&self.order.hedge_type.encode());
        if self.order.hedge_type.is_some() {
            code.push_str(&self.order.hedge_param.encode());
        }
        code.push_str(&self.order.opt_out_smart_routing.encode());
        code.push_str(&self.order.clearing_account.encode());
        code.push_str(&self.order.clearing_intent.encode());
        code.push_str(&self.order.not_held.encode());
        match &self.contract.delta_neutral_contract {
            Some(dn) => {
                code.push_str("1\0");
                code.push_str(&dn.con_id.encode());
                code.push_str(&dn.delta.encode());
                code.push_str(&dn.price.encode());
            },
            None => code.push_str("0\0"),
        };
        code.push_str(&self.order.algo_strategy.encode());
        if self.order.algo_strategy.is_some() {
            match &self.order.algo_params {
                Some(params) => {
                    code.push_str(&params.len().encode());
                    for param in params {
                        code.push_str(&param.0.encode());
                        code.push_str(&param.1.encode());
                    }
                },
                None => code.push_str("0\0"),
            }
        }
        code.push_str(&self.order.algo_id.encode());
        code.push_str(&self.order.what_if.encode());
        code.push_str(&self.order.order_misc_options.encode());
        code.push_str(&self.order.solicited.encode());
        code.push_str(&self.order.randomize_size.encode());
        code.push_str(&self.order.randomize_price.encode());

        if self.order.order_type == OrderType::PeggedToBenchmark {
            code.push_str(&self.order.reference_contract_id.encode());
            code.push_str(&self.order.is_pegged_change_amount_decrease.encode());
            code.push_str(&self.order.pegged_change_amount.encode());
            code.push_str(&self.order.reference_change_amount.encode());
            code.push_str(&self.order.reference_exchange_id.encode());
        }

        match &self.order.conditions {
            Some(conds) => {
                code.push_str(&conds.len().encode());
                for cond in conds {
                    // C++ API has some facility for external notification here
                    code.push_str(&cond.encode());
                }
                code.push_str(&self.order.conditions_ignore_rth.encode());
                code.push_str(&self.order.conditions_cancel_order.encode());
            },
            None => code.push_str("0\0"),
        }

        code.push_str(&self.order.adjusted_order.encode());
        match &self.order.soft_dollar_tier {
            Some(tier) => {
                code.push_str(&tier.name.encode());
                code.push_str(&tier.val.encode());
            },
            None => code.push_str("\0\0"),
        }
        code.push_str(&self.order.cash_qty.encode());

        code.push_str(&self.order.mifid_2.encode());

        code.push_str(&self.order.dont_use_auto_price_for_hedge.encode());
        code.push_str(&self.order.is_oms_container.encode());
        code.push_str(&self.order.discretionary_up_to_limit_price.encode());
        code.push_str(&self.order.use_price_mgmt_algo.encode());
        code
    }
}

impl ParseIbkrFrame for Order {
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
        let mut contract = Contract::try_parse_frame(msg_id, Some(server_version), it)?;
        tracing::debug!("contract: {:#?}", &contract);
        let mut order = OrderData {
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

            // ##############################################################
            financial_advisor: {
                let fa = FinancialAdvisor::try_parse_frame(msg_id, Some(server_version), it)?;
                if fa.fa_group.is_some()
                    || fa.fa_profile.is_some()
                    || fa.fa_method.is_some()
                    || fa.fa_percentage.is_some()
                {
                    Some(fa)
                } else {
                    None
                }
            },
            // #############################################################
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
            ..Default::default()
        };
        // ####################################################################
        order.volatility_order_parameter =
            VolatilityOrderParameter::try_parse_frame(msg_id, Some(server_version), it)?;

        // ####################################################################
        // read trail params
        order.trail_stop_price = decode(it)?;
        order.trailing_percent = decode(it)?;
        order.basis_points = if !completed { decode(it)? } else { None };
        order.basis_points_type = if !completed { decode(it)? } else { None };

        // ####################################################################
        let combo_legs = ComboLegs::try_parse_frame(msg_id, Some(server_version), it)?;
        if combo_legs.description.is_some() && !combo_legs.legs.is_empty() {
            contract.combo_legs = Some(combo_legs);
        }
        // ####################################################################
        let order_combo_legs_count: Option<usize> = decode(it)?;
        if let Some(n) = order_combo_legs_count {
            let mut order_legs: Vec<Option<D256>> = Vec::with_capacity(n);
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

        // ####################################################################
        order.scale_order_parameter =
            ScaleOrderParameter::try_parse_frame(msg_id, Some(server_version), it)?;

        // ####################################################################

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
                contract.delta_neutral_contract = Some(DeltaNeutralContract {
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
        // ####################################################################
        let mut order_state = match msg_id {
            Incoming::OpenOrder => OrderState::try_parse_frame(msg_id, Some(server_version), it)?,
            Incoming::CompletedOrder => {
                OrderState {
                    status: decode(it)?.unwrap(),
                    ..Default::default()
                }
            },
            _ => OrderState::default(),
        };

        // ####################################################################
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
        // ####################################################################
        let adj_order = AdjustedOrder::try_parse_frame(msg_id, Some(server_version), it)?;

        if adj_order.adjusted_order_type.is_some()
            || adj_order.trail_stop_price.is_some()
            || adj_order.trigger_price.is_some()
            || adj_order.adjusted_stop_price.is_some()
            || adj_order.adjusted_stop_limit_price.is_some()
            || adj_order.adjusted_trailing_amount.is_some()
            || adj_order.lmt_price_offset.is_some()
        {
            order.adjusted_order = Some(adj_order);
        }

        // ####################################################################

        if !completed {
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
            order.duration = decode(it)?;
            order.post_to_ats = decode(it)?;

            order.auto_cancel_parent = decode(it)?.unwrap();
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
        // PegBestPegMidOrderAttributes
        order.min_trade_qty = decode(it)?;
        order.min_compete_size = decode(it)?;
        order.compete_against_best_offset = decode(it)?;
        order.mid_offset_at_whole = decode(it)?;
        order.mid_offset_at_half = decode(it)?;

        order.customer_account = decode(it)?;
        order.professional_customer = decode(it)?.unwrap();
        if !completed {
            order.bond_accrued_interest = decode(it)?;
            order.include_overnight = decode(it)?.unwrap();
            order.ext_operator = decode(it)?;
            order.manual_order_indicator = decode(it)?;
        }

        order.submitter = decode(it)?;
        if !completed {
            order.imbalance_only = decode(it)?.unwrap();
        }

        Ok(Order {
            order,
            order_state,
            order_id,
            contract,
        })
    }
}

#[cfg(test)]
mod tests;
