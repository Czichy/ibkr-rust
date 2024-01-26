use std::str::Split;

use crossbeam::channel::{unbounded, Receiver, Sender};
use rust_decimal::prelude::*;
use rust_decimal_macros::dec;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{contract::{ComboLeg, Contract, DeltaNeutralContract, SecType},
            enums::*,
            ib_frame::{ParseError, ParseIbkrFrame, ParseResult},
            prelude::{ib_message::{decode, Decodable},
                      UsePriceMgmtAlgo},
            utils::ib_message::Encodable,
            AccountCode,
            ClientId,
            OrderId,
            TimeStamp};

#[derive(Debug, Clone)]
pub struct OrderTracker {
    pub order:              Receiver<Order>,
    pub order_id:           Receiver<OrderId>,
    pub error:              Option<(i32, String)>,
    pub order_state:        Receiver<OrderState>,
    pub order_status:       Receiver<OrderStatusUpdate>,
    pub executions:         Receiver<Execution>,
    pub commission_reports: Receiver<CommissionReport>,
}
pub(crate) struct OrderTrackerSender {
    pub executions_tx:         Sender<Execution>,
    pub order_tx:              Sender<Order>,
    pub order_id_tx:           Sender<OrderId>,
    pub order_status_tx:       Sender<OrderStatusUpdate>,
    pub order_state_tx:        Sender<OrderState>,
    pub commission_reports_tx: Sender<CommissionReport>,
}
impl OrderTracker {
    pub(crate) fn new() -> (OrderTrackerSender, Self) {
        let (executions_tx, executions) = unbounded();
        let (commission_reports_tx, commission_reports) = unbounded();
        let (order_status_tx, order_status) = unbounded();
        let (order_state_tx, order_state) = unbounded();
        let (order_tx, order) = unbounded();
        let (order_id_tx, order_id) = unbounded();
        (
            OrderTrackerSender {
                order_tx,
                order_id_tx,
                executions_tx,
                commission_reports_tx,
                order_status_tx,
                order_state_tx,
            },
            OrderTracker {
                order,
                order_id,
                error: None,
                executions,
                commission_reports,
                order_status,
                order_state,
            },
        )
    }
}

#[derive(Default, Debug, Clone)]
pub struct SoftDollarTier {
    pub name:         Option<String>,
    pub val:          Option<String>,
    pub display_name: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct Order {
    // contract
    pub contract: Contract,

    // order identification
    pub order_id:  Option<OrderId>,
    pub client_id: Option<usize>,
    pub perm_id:   i32,

    // main order fields
    pub action:     Action,
    pub total_qty:  Decimal,
    pub order_type: OrderType,
    pub lmt_price:  Option<Decimal>,
    pub aux_price:  Option<Decimal>,

    // extended order fields
    pub tif:                             Option<TimeInForce>,
    pub active_start_time:               Option<String>,
    pub active_stop_time:                Option<String>,
    pub oca_group:                       Option<String>,
    pub oca_type:                        Option<OCAType>,
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
    pub percent_offset:                  Option<Decimal>,
    pub trail_stop_price:                Option<Decimal>,
    pub trailing_percent:                Option<Decimal>,

    // financial advisor fields
    pub fa_group:      Option<String>,
    pub fa_profile:    Option<String>,
    pub fa_method:     Option<String>,
    pub fa_percentage: Option<String>,

    // institutional (i.e. non-cleared) only
    pub open_close:          Option<OrderOpenClose>,
    pub origin:              Option<Origin>,
    pub short_sale_slot:     Option<ShortSaleSlot>,
    pub designated_location: Option<String>,
    pub exempt_code:         i32,

    // SMART routing fields
    pub discretionary_amt:     Decimal,
    pub e_trade_only:          Option<bool>,
    pub firm_quote_only:       Option<bool>,
    pub nbbo_price_cap:        Option<Decimal>,
    pub opt_out_smart_routing: Option<bool>,

    // BOX exchange order fields
    pub auction_strategy: Option<AuctionStrategy>,
    pub starting_price:   Option<Decimal>,
    pub stock_ref_price:  Option<Decimal>,
    pub delta:            Option<Decimal>,

    // Pegged to stock and VOL order fields
    pub stock_range_lower: Option<Decimal>,
    pub stock_range_upper: Option<Decimal>,

    pub randomize_size:  bool,
    pub randomize_price: bool,

    // Volatility order fields
    pub volatility:                        Option<Decimal>,
    pub volatility_type:                   Option<VolatilityType>,
    pub delta_neutral_order_type:          Option<OrderType>,
    pub delta_neutral_aux_price:           Option<Decimal>,
    pub delta_neutral_con_id:              usize,
    pub delta_neutral_settling_firm:       Option<String>,
    pub delta_neutral_clearing_account:    Option<String>,
    pub delta_neutral_clearing_intent:     Option<String>,
    pub delta_neutral_open_close:          Option<String>,
    pub delta_neutral_short_sale:          bool,
    pub delta_neutral_short_sale_slot:     bool,
    pub delta_neutral_designated_location: Option<String>,
    pub continuous_update:                 bool,
    pub reference_price_type:              Option<ReferencePriceType>,

    // Combo order fields
    pub basis_points:      Option<Decimal>,
    pub basis_points_type: Option<BasisPointsType>,

    // Scale order fields
    pub scale_init_level_size:       Option<i32>,
    pub scale_subs_level_size:       Option<i32>,
    pub scale_price_increment:       Option<Decimal>,
    pub scale_price_adjust_value:    Option<Decimal>,
    pub scale_price_adjust_interval: Option<i32>,
    pub scale_profit_offset:         Option<Decimal>,
    pub scale_auto_reset:            bool,
    pub scale_init_position:         Option<i32>,
    pub scale_init_fill_qty:         Option<i32>,
    pub scale_random_percent:        bool,
    pub scale_table:                 Option<String>,

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
    pub order_combo_legs:   Option<Vec<Option<Decimal>>>,
    pub order_misc_options: Option<Vec<(String, String)>>,

    // VER PEG2BENCH fields
    pub reference_contract_id:            i32,
    pub pegged_change_amount:             Option<Decimal>,
    pub is_pegged_change_amount_decrease: bool,
    pub reference_change_amount:          f64,
    pub reference_exchange_id:            Option<String>,
    pub adjusted_order_type:              Option<String>,
    pub trigger_price:                    Option<Decimal>,
    pub adjusted_stop_price:              Option<Decimal>,
    pub adjusted_stop_limit_price:        Option<Decimal>,
    pub adjusted_trailing_amount:         Option<Decimal>,
    pub adjustable_trailing_unit:         i32,
    pub lmt_price_offset:                 Option<Decimal>,

    pub conditions:              Option<Vec<OrderConditionType>>,
    pub conditions_cancel_order: bool,
    pub conditions_ignore_rth:   bool,

    // ext operator
    pub ext_operator: Option<String>,

    pub soft_dollar_tier: Option<SoftDollarTier>,

    pub cash_qty: Option<Decimal>,

    pub mifid_2_decision_maker:   Option<String>,
    pub mifid_2_decision_algo:    Option<String>,
    pub mifid_2_execution_trader: Option<String>,
    pub mifid_2_execution_algo:   Option<String>,

    pub dont_use_auto_price_for_hedge:   bool,
    pub is_oms_container:                bool,
    pub discretionary_up_to_limit_price: bool,
    pub auto_cancel_date:                Option<String>,
    pub filled_quantity:                 Option<Decimal>,
    pub ref_futures_con_id:              Option<i32>,
    pub auto_cancel_parent:              bool,
    pub shareholder:                     Option<String>,
    pub imbalance_only:                  bool,
    pub route_marketable_to_bbo:         bool,
    pub parent_perm_id:                  Option<usize>,
    pub use_price_mgmt_algo:             Option<UsePriceMgmtAlgo>,
}

impl Order {
    #[allow(clippy::missing_const_for_fn)]
    fn new() -> Self {
        Order {
            transmit: true,
            open_close: Some(OrderOpenClose::Open),
            origin: Some(Origin::Customer),
            exempt_code: -1,
            auction_strategy: Some(AuctionStrategy::NoAuctionStrategy),
            ..Default::default()
        }
    }

    pub fn market(contract: Contract, action: Action, qty: Decimal) -> Self {
        let mut order = Order::new();
        order.action = action;
        order.contract = contract;
        order.total_qty = qty;
        order
    }

    pub fn market_on_close(contract: Contract, action: Action, qty: Decimal) -> Self {
        let mut order = Order::new();
        order.action = action;
        order.contract = contract;
        order.total_qty = qty;
        order.order_type = OrderType::MarketOnClose;
        order
    }

    pub fn relative_market(contract: Contract, action: Action, qty: Decimal) -> Self {
        let mut order = Order::new();
        order.action = action;
        order.contract = contract;
        order.total_qty = qty;
        order.order_type = OrderType::RelativeMarket;
        order
    }

    pub fn limit(
        contract: Contract,
        action: Action,
        qty: Decimal,
        lmt: Decimal,
        tif: TimeInForce,
    ) -> Self {
        let mut order = Order::new();
        order.action = action;
        order.contract = contract;
        order.total_qty = qty;
        order.order_type = OrderType::Limit;
        order.lmt_price = Some(lmt);
        order.tif = Some(tif);
        order
    }
}

impl Encodable for Order {
    fn encode(&self) -> String {
        let mut code = String::new();
        code.push_str(&self.contract.encode_for_order());
        code.push_str(&self.action.encode());
        code.push_str(&self.total_qty.encode());
        code.push_str(&self.order_type.encode());
        code.push_str(&self.lmt_price.encode());
        code.push_str(&self.aux_price.encode());
        code.push_str(&self.tif.encode());
        code.push_str(&self.oca_group.encode());
        code.push_str(&self.account.encode());
        code.push_str(&self.open_close.encode());
        code.push_str(&self.origin.encode());
        code.push_str(&self.order_ref.encode());
        code.push_str(&self.transmit.encode());
        code.push_str(&self.parent_id.encode());
        code.push_str(&self.block_order.encode());
        code.push_str(&self.sweep_to_fill.encode());
        code.push_str(&self.display_size.encode());
        code.push_str(&self.trigger_method.encode());
        code.push_str(&self.outside_rth.encode());
        code.push_str(&self.hidden.encode());
        let sec = &self.contract.sec_type;
        if *sec == SecType::Combo {
            match &self.contract.combo_legs {
                Some(legs) => {
                    code.push_str(&legs.len().encode());
                    for leg in legs {
                        code.push_str(&leg.con_id.encode());
                        code.push_str(&leg.ratio.encode());
                        code.push_str(&leg.action.encode());
                        code.push_str(&leg.exchange.encode());
                        code.push_str(&leg.open_close.encode());
                        code.push_str(&leg.shortsale_slot.encode());
                        code.push_str(&leg.designated_location.encode());
                        code.push_str(&leg.exempt_code.encode());
                    }
                },
                None => code.push_str("0\0"),
            }
            match &self.order_combo_legs {
                Some(legs) => {
                    code.push_str(&legs.len().encode());
                    for leg in legs {
                        code.push_str(&leg.encode());
                    }
                },
                None => code.push_str("0\0"),
            }
            match &self.smart_combo_routing_params {
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
        code.push_str(&self.discretionary_amt.encode());
        code.push_str(&self.good_after_time.encode());
        code.push_str(&self.good_till_date.encode());
        code.push_str(&self.fa_group.encode());
        code.push_str(&self.fa_method.encode());
        code.push_str(&self.fa_percentage.encode());
        code.push_str(&self.fa_profile.encode());
        code.push_str(&self.model_code.encode());
        code.push_str(&self.short_sale_slot.encode());
        code.push_str(&self.designated_location.encode());
        code.push_str(&self.exempt_code.encode());
        code.push_str(&self.oca_type.encode());
        code.push_str(&self.rule_80_a.encode());
        code.push_str(&self.settling_firm.encode());
        code.push_str(&self.all_or_none.encode());
        code.push_str(&self.min_qty.encode());
        code.push_str(&self.percent_offset.encode());
        code.push_str(&self.e_trade_only.encode());
        code.push_str(&self.firm_quote_only.encode());
        code.push_str(&self.nbbo_price_cap.encode());
        code.push_str(&self.auction_strategy.encode());
        code.push_str(&self.starting_price.encode());
        code.push_str(&self.stock_ref_price.encode());
        code.push_str(&self.delta.encode());
        code.push_str(&self.stock_range_lower.encode());
        code.push_str(&self.stock_range_upper.encode());
        code.push_str(&self.override_percentage_constraints.encode());
        code.push_str(&self.volatility.encode());
        code.push_str(&self.volatility_type.encode());
        code.push_str(&self.delta_neutral_order_type.encode());
        code.push_str(&self.delta_neutral_aux_price.encode());
        if self.delta_neutral_order_type.is_some() {
            code.push_str(&self.delta_neutral_con_id.encode());
            code.push_str(&self.delta_neutral_settling_firm.encode());
            code.push_str(&self.delta_neutral_clearing_account.encode());
            code.push_str(&self.delta_neutral_clearing_intent.encode());
            code.push_str(&self.delta_neutral_open_close.encode());
            code.push_str(&self.delta_neutral_short_sale.encode());
            code.push_str(&self.delta_neutral_designated_location.encode());
        }
        code.push_str(&self.continuous_update.encode());
        code.push_str(&self.reference_price_type.encode());
        code.push_str(&self.trail_stop_price.encode());
        code.push_str(&self.trailing_percent.encode());
        code.push_str(&self.scale_init_level_size.encode());
        code.push_str(&self.scale_subs_level_size.encode());
        code.push_str(&self.scale_price_increment.encode());
        if let Some(inc) = self.scale_price_increment {
            if inc > dec!(0.0) {
                code.push_str(&self.scale_price_adjust_value.encode());
                code.push_str(&self.scale_price_adjust_interval.encode());
                code.push_str(&self.scale_profit_offset.encode());
                code.push_str(&self.scale_auto_reset.encode());
                code.push_str(&self.scale_init_position.encode());
                code.push_str(&self.scale_init_fill_qty.encode());
                code.push_str(&self.scale_random_percent.encode());
            }
        }
        code.push_str(&self.scale_table.encode());
        code.push_str(&self.active_start_time.encode());
        code.push_str(&self.active_stop_time.encode());
        code.push_str(&self.hedge_type.encode());
        if self.hedge_type.is_some() {
            code.push_str(&self.hedge_param.encode());
        }
        code.push_str(&self.opt_out_smart_routing.encode());
        code.push_str(&self.clearing_account.encode());
        code.push_str(&self.clearing_intent.encode());
        code.push_str(&self.not_held.encode());
        match &self.contract.delta_neutral_contract {
            Some(dn) => {
                code.push_str("1\0");
                code.push_str(&dn.con_id.encode());
                code.push_str(&dn.delta.encode());
                code.push_str(&dn.price.encode());
            },
            None => code.push_str("0\0"),
        };
        code.push_str(&self.algo_strategy.encode());
        if self.algo_strategy.is_some() {
            match &self.algo_params {
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
        code.push_str(&self.algo_id.encode());
        code.push_str(&self.what_if.encode());
        code.push_str(&self.order_misc_options.encode());
        code.push_str(&self.solicited.encode());
        code.push_str(&self.randomize_size.encode());
        code.push_str(&self.randomize_price.encode());

        if self.order_type == OrderType::PeggedToBenchmark {
            code.push_str(&self.reference_contract_id.encode());
            code.push_str(&self.is_pegged_change_amount_decrease.encode());
            code.push_str(&self.pegged_change_amount.encode());
            code.push_str(&self.reference_change_amount.encode());
            code.push_str(&self.reference_exchange_id.encode());
        }

        match &self.conditions {
            Some(conds) => {
                code.push_str(&conds.len().encode());
                for cond in conds {
                    // C++ API has some facility for external notification here
                    code.push_str(&cond.encode());
                }
                code.push_str(&self.conditions_ignore_rth.encode());
                code.push_str(&self.conditions_cancel_order.encode());
            },
            None => code.push_str("0\0"),
        }

        code.push_str(&self.adjusted_order_type.encode());
        code.push_str(&self.trigger_price.encode());
        code.push_str(&self.lmt_price_offset.encode());
        code.push_str(&self.adjusted_stop_price.encode());
        code.push_str(&self.adjusted_stop_limit_price.encode());
        code.push_str(&self.adjusted_trailing_amount.encode());
        code.push_str(&self.adjustable_trailing_unit.encode());
        code.push_str(&self.ext_operator.encode());
        match &self.soft_dollar_tier {
            Some(tier) => {
                code.push_str(&tier.name.encode());
                code.push_str(&tier.val.encode());
            },
            None => code.push_str("\0\0"),
        }
        code.push_str(&self.cash_qty.encode());

        code.push_str(&self.mifid_2_decision_maker.encode());
        code.push_str(&self.mifid_2_decision_algo.encode());
        code.push_str(&self.mifid_2_execution_trader.encode());
        code.push_str(&self.mifid_2_execution_algo.encode());

        code.push_str(&self.dont_use_auto_price_for_hedge.encode());
        code.push_str(&self.is_oms_container.encode());
        code.push_str(&self.discretionary_up_to_limit_price.encode());
        code.push_str(&self.use_price_mgmt_algo.encode());
        code
    }
}

#[derive(Default, Debug, Clone)]
pub struct OrderState {
    pub order_id:                      Option<OrderId>,
    pub perm_id:                       i32,
    pub status:                        OrderStatus, // Option<String>,
    pub init_margin_before:            Option<Decimal>,
    pub maint_margin_before:           Option<Decimal>,
    pub init_margin_change:            Option<Decimal>,
    pub equity_with_loan_value_before: Option<Decimal>,
    pub maint_margin_change:           Option<Decimal>,
    pub equity_with_loan_change:       Option<Decimal>,
    pub init_margin_after:             Option<Decimal>,
    pub maint_margin_after:            Option<Decimal>,
    pub equity_with_loan_after:        Option<Decimal>,
    pub commission:                    Option<Decimal>,
    pub min_commission:                Option<Decimal>,
    pub max_commission:                Option<Decimal>,
    pub commission_currency:           Option<String>,
    pub warning_text:                  Option<String>,
    pub completed_time:                Option<TimeStamp>,
    pub completed_status:              Option<String>,
}

#[derive(Default, Debug, Clone)]
pub struct OrderInformation {
    pub order:       Order,
    pub order_state: OrderState,
}
impl ParseIbkrFrame for OrderInformation {
    #[allow(clippy::cognitive_complexity)]
    fn try_parse_frame(msg_id: Incoming, it: &mut Split<&str>) -> ParseResult<Self>
    where
        Self: Sized,
    {
        if !matches!(msg_id, Incoming::OpenOrder | Incoming::CompletedOrder) {
            return Err(ParseError::UnexpectedMessage);
        }
        let completed = matches!(msg_id, Incoming::CompletedOrder);
        tracing::debug!("decode Orders: {:#?}", msg_id);
        let order_id = if !completed { decode(it)? } else { None };
        tracing::debug!("orderid: {:#?}", order_id);
        // decode contract
        let contract = Contract::try_parse_frame(msg_id, it)?;
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
            // rule_80_a: decode(it)?,
            percent_offset: decode(it)?,
            settling_firm: { decode(it)? },

            short_sale_slot: decode(it)?,
            designated_location: decode(it)?,
            exempt_code: decode(it)?.unwrap(),

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
            let mut combo_params: Vec<(String, String)> = Vec::with_capacity(n);
            for _i in 0..n {
                combo_params.push((decode(it)?.unwrap(), decode(it)?.unwrap()));
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
            warning_text: if !completed { decode(it)? } else { None },
            ..Default::default()
        };
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
    fn try_parse_frame(msg_id: Incoming, it: &mut Split<&str>) -> ParseResult<Self>
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

#[derive(Debug, Clone)]
pub struct Execution {
    pub exec_id:        String,
    pub time:           TimeStamp,
    pub acct_number:    String,
    pub exchange:       Option<String>,
    pub side:           Side,
    pub shares:         Decimal,
    pub price:          Decimal,
    pub perm_id:        i32,
    pub client_id:      ClientId,
    pub order_id:       OrderId,
    pub contract:       Contract,
    pub liquidation:    i32,
    pub cum_qty:        Decimal,
    pub avg_price:      Decimal,
    pub order_ref:      Option<String>,
    pub ev_rule:        Option<String>,
    pub ev_multiplier:  Option<Decimal>,
    pub model_code:     Option<String>,
    pub last_liquidity: Option<i32>,
}

impl ParseIbkrFrame for Execution {
    fn try_parse_frame(msg_id: Incoming, it: &mut Split<&str>) -> ParseResult<Self>
    where
        Self: Sized,
    {
        if !matches!(msg_id, Incoming::ExecutionData) {
            return Err(ParseError::UnexpectedMessage);
        }
        it.next(); // skip version
        let order_id: i32 = decode(it)?.unwrap();
        let contract = Contract::try_parse_frame(msg_id, it)?;

        Ok(Self {
            order_id,
            contract,
            exec_id: decode(it)?.unwrap(),
            time: decode(it)?.unwrap(),
            acct_number: decode(it)?.unwrap(),
            exchange: decode(it)?,
            side: decode(it)?.unwrap(),
            shares: decode(it)?.unwrap(),
            price: decode(it)?.unwrap(),
            perm_id: decode(it)?.unwrap(),
            client_id: decode(it)?.unwrap(),
            liquidation: decode(it)?.unwrap(),
            cum_qty: decode(it)?.unwrap(),
            avg_price: decode(it)?.unwrap(),
            order_ref: decode(it)?,
            ev_rule: decode(it)?,
            ev_multiplier: decode(it)?,
            model_code: decode(it)?,
            last_liquidity: decode(it)?,
        })
    }
}

#[derive(Default, Debug, Clone)]
pub struct CommissionReport {
    pub exec_id:               String,
    pub commission:            Decimal,
    pub currency:              String,
    pub realized_pnl:          Option<Decimal>,
    pub yield_amount:          Option<Decimal>,
    pub yield_redemption_date: Option<i32>,
}
impl ParseIbkrFrame for CommissionReport {
    fn try_parse_frame(msg_id: Incoming, it: &mut Split<&str>) -> ParseResult<Self>
    where
        Self: Sized,
    {
        if !matches!(msg_id, Incoming::CommissionReport) {
            return Err(ParseError::UnexpectedMessage);
        }
        it.next(); // skip version
        Ok(Self {
            exec_id:               decode(it)?.unwrap(),
            commission:            decode(it)?.unwrap(),
            currency:              decode(it)?.unwrap(),
            realized_pnl:          decode(it)?,
            yield_amount:          decode(it)?,
            yield_redemption_date: decode(it)?,
        })
    }
}

#[derive(Clone, Debug, Default)]
pub struct ExecutionFilter {
    pub client_id:    Option<ClientId>,
    pub account_code: AccountCode,
    pub time:         String,
    pub symbol:       String,
    pub sec_type:     String,
    pub exchange:     String,
    pub side:         Option<Side>,
}

impl ExecutionFilter {
    pub const fn new(
        client_id: ClientId,
        acct_code: AccountCode,
        time: String,
        symbol: String,
        sec_type: String,
        exchange: String,
        side: Option<Side>,
    ) -> Self {
        ExecutionFilter {
            client_id: Some(client_id),
            account_code: acct_code,
            time,
            symbol,
            sec_type,
            exchange,
            side,
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ComboAction {
    Buy,
    Sell,
    ShortSell,
}

impl Encodable for ComboAction {
    fn encode(&self) -> String {
        match self {
            ComboAction::Buy => "BUY\0",
            ComboAction::Sell => "SELL\0",
            ComboAction::ShortSell => "SSELL\0",
        }
        .to_string()
    }
}

impl FromStr for ComboAction {
    type Err = ParseEnumError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "BUY" => Ok(ComboAction::Buy),
            "SELL" => Ok(ComboAction::Sell),
            "SSELL" => Ok(ComboAction::ShortSell),
            &_ => Err(ParseEnumError),
        }
    }
}

impl Decodable for ComboAction {}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OptionOpenClose {
    Same,
    Open,
    Close,
    Unknown,
}

impl Encodable for OptionOpenClose {
    fn encode(&self) -> String {
        match self {
            OptionOpenClose::Same => "0\0",
            OptionOpenClose::Open => "1\0",
            OptionOpenClose::Close => "2\0",
            OptionOpenClose::Unknown => "3\0",
        }
        .to_string()
    }
}

impl FromStr for OptionOpenClose {
    type Err = ParseEnumError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(OptionOpenClose::Same),
            "1" => Ok(OptionOpenClose::Open),
            "2" => Ok(OptionOpenClose::Close),
            "3" => Ok(OptionOpenClose::Unknown),
            &_ => Err(ParseEnumError),
        }
    }
}

impl Decodable for OptionOpenClose {}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShortSaleSlot {
    NoSlot,
    Broker,
    ThirdParty,
}

impl Encodable for ShortSaleSlot {
    fn encode(&self) -> String {
        match self {
            ShortSaleSlot::NoSlot => "0\0",
            ShortSaleSlot::Broker => "1\0",
            ShortSaleSlot::ThirdParty => "2\0",
        }
        .to_string()
    }
}

impl FromStr for ShortSaleSlot {
    type Err = ParseEnumError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(ShortSaleSlot::NoSlot),
            "1" => Ok(ShortSaleSlot::Broker),
            "2" => Ok(ShortSaleSlot::ThirdParty),
            &_ => Err(ParseEnumError),
        }
    }
}

impl Decodable for ShortSaleSlot {}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
#[derive(Debug, Clone, Copy, Default)]
pub enum Action {
    #[default]
    Buy,
    Sell,
    SellShort,
    SellLong,
}

impl Encodable for Action {
    fn encode(&self) -> String {
        match self {
            Action::Buy => "BUY\0",
            Action::Sell => "SELL\0",
            Action::SellShort => "SSELL\0",
            Action::SellLong => "SLONG\0",
        }
        .to_string()
    }
}

impl FromStr for Action {
    type Err = ParseEnumError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "BUY" => Ok(Action::Buy),
            "SELL" => Ok(Action::Sell),
            "SSELL" => Ok(Action::SellShort),
            "SLONG" => Ok(Action::SellLong),
            &_ => Err(ParseEnumError),
        }
    }
}

impl Decodable for Action {}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
#[derive(Debug, Clone, Copy)]
pub enum OrderOpenClose {
    Open,
    Close,
}

impl Encodable for OrderOpenClose {
    fn encode(&self) -> String {
        match self {
            OrderOpenClose::Open => "O\0",
            OrderOpenClose::Close => "C\0",
        }
        .to_string()
    }
}

impl FromStr for OrderOpenClose {
    type Err = ParseEnumError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "O" => Ok(OrderOpenClose::Open),
            "C" => Ok(OrderOpenClose::Close),
            &_ => Err(ParseEnumError),
        }
    }
}

impl Decodable for OrderOpenClose {}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
pub enum OrderType {
    NoOrderType, // only legit for deltaNeutralOrderType
    Limit,
    #[default]
    Market,
    MarketIfTouched,
    MarketOnClose,
    MarketOnOpen,
    PeggedToMarket,
    PeggedToStock,
    PeggedToPrimary,
    BoxTop,
    LimitIfTouched,
    LimitOnClose,
    PassiveRelative,
    PeggedToMidpoint,
    MarketToLimit,
    MarketWithProtection,
    MidPrice,
    Stop,
    StopLimit,
    StopWithProtection,
    TrailingStop,
    TrailingStopLimit,
    RelativeLimit,
    RelativeMarket,
    Volatility,
    PeggedToBenchmark,
}
impl Encodable for OrderType {
    fn encode(&self) -> String {
        match self {
            OrderType::NoOrderType => "None\0",
            OrderType::Limit => "LMT\0",
            OrderType::Market => "MKT\0",
            OrderType::MarketIfTouched => "MIT\0",
            OrderType::MarketOnClose => "MOC\0",
            OrderType::MarketOnOpen => "MOO\0",
            OrderType::PeggedToMarket => "PEG MKT\0",
            OrderType::PeggedToStock => "PEG STK\0",
            OrderType::PeggedToPrimary => "REL\0",
            OrderType::BoxTop => "BOX TOP\0",
            OrderType::LimitIfTouched => "LIT\0",
            OrderType::LimitOnClose => "LOC\0",
            OrderType::PassiveRelative => "PASSV REL\0",
            OrderType::PeggedToMidpoint => "PEG MID\0",
            OrderType::MarketToLimit => "MTL\0",
            OrderType::MarketWithProtection => "MKT PRT\0",
            OrderType::MidPrice => "MIDPRICE\0",
            OrderType::Stop => "STP\0",
            OrderType::StopLimit => "STP LMT\0",
            OrderType::StopWithProtection => "STP PRT\0",
            OrderType::TrailingStop => "TRAIL\0",
            OrderType::TrailingStopLimit => "TRAIL LIMIT\0",
            OrderType::RelativeLimit => "Rel + LMT\0",
            OrderType::RelativeMarket => "Rel + MKT\0",
            OrderType::Volatility => "VOL\0",
            OrderType::PeggedToBenchmark => "PEG BENCH\0",
        }
        .to_string()
    }
}

impl FromStr for OrderType {
    type Err = ParseEnumError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "None" => Ok(OrderType::NoOrderType),
            "Keine" => Ok(OrderType::NoOrderType),
            "LMT" => Ok(OrderType::Limit),
            "MKT" => Ok(OrderType::Market),
            "MIT" => Ok(OrderType::MarketIfTouched),
            "MOC" => Ok(OrderType::MarketOnClose),
            "MOO" => Ok(OrderType::MarketOnOpen),
            "PEG MKT" => Ok(OrderType::PeggedToMarket),
            "PEG STK" => Ok(OrderType::PeggedToStock),
            "REL" => Ok(OrderType::PeggedToPrimary),
            "BOX TOP" => Ok(OrderType::BoxTop),
            "LIT" => Ok(OrderType::LimitIfTouched),
            "LOC" => Ok(OrderType::LimitOnClose),
            "PASSV REL" => Ok(OrderType::PassiveRelative),
            "PEG MID" => Ok(OrderType::PeggedToMidpoint),
            "MTL" => Ok(OrderType::MarketToLimit),
            "MKT PRT" => Ok(OrderType::MarketWithProtection),
            "MIDPRICE" => Ok(OrderType::MidPrice),
            "STP" => Ok(OrderType::Stop),
            "STP LMT" => Ok(OrderType::StopLimit),
            "STP PRT" => Ok(OrderType::StopWithProtection),
            "TRAIL" => Ok(OrderType::TrailingStop),
            "TRAIL LIMIT" => Ok(OrderType::TrailingStopLimit),
            "REL + LMT" => Ok(OrderType::RelativeLimit),
            "REL + MKT" => Ok(OrderType::RelativeMarket),
            "VOL" => Ok(OrderType::Volatility),
            "PEG BENCH" => Ok(OrderType::PeggedToBenchmark),
            &_ => Err(ParseEnumError),
        }
    }
}
impl Decodable for OrderType {}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
#[derive(Debug, Clone, Copy)]
pub enum TriggerMethod {
    Default,
    DoubleBidAsk,
    Last,
    DoubleLast,
    BidAsk,
    LastOrBidAsk,
    MidPoint,
}

impl Encodable for TriggerMethod {
    fn encode(&self) -> String {
        match self {
            TriggerMethod::Default => "0\0",
            TriggerMethod::DoubleBidAsk => "1\0",
            TriggerMethod::Last => "2\0",
            TriggerMethod::DoubleLast => "3\0",
            TriggerMethod::BidAsk => "4\0",
            TriggerMethod::LastOrBidAsk => "7\0",
            TriggerMethod::MidPoint => "8\0",
        }
        .to_string()
    }
}

impl FromStr for TriggerMethod {
    type Err = ParseEnumError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(TriggerMethod::Default),
            "1" => Ok(TriggerMethod::DoubleBidAsk),
            "2" => Ok(TriggerMethod::Last),
            "3" => Ok(TriggerMethod::DoubleLast),
            "4" => Ok(TriggerMethod::BidAsk),
            "7" => Ok(TriggerMethod::LastOrBidAsk),
            "8" => Ok(TriggerMethod::MidPoint),
            &_ => Err(ParseEnumError),
        }
    }
}

impl Decodable for TriggerMethod {}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
#[derive(Debug, Clone, Copy)]
pub enum TimeInForce {
    Day,
    GoodTillCancel,
    ImmediateOrCancel,
    GoodUntilDate,
    GoodOnOpen,
    FillOrKill,
    DayUntilCancel,
}

impl Encodable for TimeInForce {
    fn encode(&self) -> String {
        match self {
            TimeInForce::Day => "DAY\0",
            TimeInForce::GoodTillCancel => "GTC\0",
            TimeInForce::ImmediateOrCancel => "IOC\0",
            TimeInForce::GoodUntilDate => "GTD\0",
            TimeInForce::GoodOnOpen => "OPG\0",
            TimeInForce::FillOrKill => "FOK\0",
            TimeInForce::DayUntilCancel => "DTC\0",
        }
        .to_string()
    }
}

impl FromStr for TimeInForce {
    type Err = ParseEnumError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "DAY" => Ok(TimeInForce::Day),
            "GTC" => Ok(TimeInForce::GoodTillCancel),
            "IOC" => Ok(TimeInForce::ImmediateOrCancel),
            "GTD" => Ok(TimeInForce::GoodTillCancel),
            "OPG" => Ok(TimeInForce::GoodOnOpen),
            "FOK" => Ok(TimeInForce::FillOrKill),
            "DTC" => Ok(TimeInForce::DayUntilCancel),
            &_ => Err(ParseEnumError),
        }
    }
}

impl Decodable for TimeInForce {}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
#[derive(Debug, Clone, Copy)]
pub enum Rule80A {
    Individual,
    Agency,
    AgentOtherMember,
    IndividualPTIA,
    AgencyPTIA,
    AgentOtherMemberPTIA,
    IndividualPT,
    AgencyPT,
    AgentOtherMemberPT,
    None,
}

impl Encodable for Rule80A {
    fn encode(&self) -> String {
        match *self {
            Rule80A::Individual => "I\0",
            Rule80A::Agency => "A\0",
            Rule80A::AgentOtherMember => "W\0",
            Rule80A::IndividualPTIA => "J\0",
            Rule80A::AgencyPTIA => "U\0",
            Rule80A::AgentOtherMemberPTIA => "M\0",
            Rule80A::IndividualPT => "K\0",
            Rule80A::AgencyPT => "Y\0",
            Rule80A::AgentOtherMemberPT => "N\0",
            Rule80A::None => "0\0",
        }
        .to_string()
    }
}

impl FromStr for Rule80A {
    type Err = ParseEnumError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "I" => Ok(Rule80A::Individual),
            "A" => Ok(Rule80A::Agency),
            "W" => Ok(Rule80A::AgentOtherMember),
            "J" => Ok(Rule80A::IndividualPTIA),
            "U" => Ok(Rule80A::AgencyPTIA),
            "M" => Ok(Rule80A::AgentOtherMemberPTIA),
            "K" => Ok(Rule80A::IndividualPT),
            "Y" => Ok(Rule80A::AgencyPT),
            "N" => Ok(Rule80A::AgentOtherMemberPT),
            "0" => Ok(Rule80A::None),
            &_ => Err(ParseEnumError),
        }
    }
}

impl Decodable for Rule80A {}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
#[derive(Debug, Clone, Copy)]
pub enum Origin {
    Customer,
    Firm,
    Unknown,
}

impl Encodable for Origin {
    fn encode(&self) -> String {
        match self {
            Origin::Customer => "0\0",
            Origin::Firm => "1\0",
            Origin::Unknown => "2\0",
        }
        .to_string()
    }
}

impl FromStr for Origin {
    type Err = ParseEnumError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(Origin::Customer),
            "1" => Ok(Origin::Firm),
            "2" => Ok(Origin::Unknown),
            &_ => Err(ParseEnumError),
        }
    }
}

impl Decodable for Origin {}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
#[derive(Debug, Clone, Copy)]
pub enum AuctionStrategy {
    NoAuctionStrategy,
    Match,
    Improvement,
    Transparent,
}

impl Encodable for AuctionStrategy {
    fn encode(&self) -> String {
        match self {
            Self::NoAuctionStrategy => "0\0",
            Self::Match => "1\0",
            Self::Improvement => "2\0",
            Self::Transparent => "3\0",
        }
        .to_string()
    }
}

impl FromStr for AuctionStrategy {
    type Err = ParseEnumError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(Self::NoAuctionStrategy),
            "1" => Ok(Self::Match),
            "2" => Ok(Self::Improvement),
            "3" => Ok(Self::Transparent),

            &_ => Err(ParseEnumError),
        }
    }
}

impl Decodable for AuctionStrategy {}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
#[derive(Debug, Clone, Copy)]
pub enum OCAType {
    NoOCAType,
    CancelWithBlock,
    ReduceWithBlock,
    ReduceNonBlock,
}

impl Encodable for OCAType {
    fn encode(&self) -> String {
        match self {
            OCAType::NoOCAType => "0\0",
            OCAType::CancelWithBlock => "1\0",
            OCAType::ReduceWithBlock => "2\0",
            OCAType::ReduceNonBlock => "3\0",
        }
        .to_string()
    }
}

impl FromStr for OCAType {
    type Err = ParseEnumError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(OCAType::NoOCAType),
            "1" => Ok(OCAType::CancelWithBlock),
            "2" => Ok(OCAType::ReduceWithBlock),
            "3" => Ok(OCAType::ReduceNonBlock),
            &_ => Err(ParseEnumError),
        }
    }
}

impl Decodable for OCAType {}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
#[derive(Debug, Clone, Copy)]
pub enum VolatilityType {
    NoVolType,
    Daily,
    Annual,
}

impl Encodable for VolatilityType {
    fn encode(&self) -> String {
        match self {
            VolatilityType::NoVolType => "0\0",
            VolatilityType::Daily => "1\0",
            VolatilityType::Annual => "2\0",
        }
        .to_string()
    }
}

impl FromStr for VolatilityType {
    type Err = ParseEnumError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(VolatilityType::NoVolType),
            "1" => Ok(VolatilityType::Daily),
            "2" => Ok(VolatilityType::Annual),
            &_ => Err(ParseEnumError),
        }
    }
}

impl Decodable for VolatilityType {}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
#[derive(Debug, Clone, Copy)]
pub enum ReferencePriceType {
    NoRefPriceType,
    Average,
    BidOrAsk,
}

impl Encodable for ReferencePriceType {
    fn encode(&self) -> String {
        match self {
            ReferencePriceType::NoRefPriceType => "0\0",
            ReferencePriceType::Average => "1\0",
            ReferencePriceType::BidOrAsk => "2\0",
        }
        .to_string()
    }
}

impl FromStr for ReferencePriceType {
    type Err = ParseEnumError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(ReferencePriceType::NoRefPriceType),
            "1" => Ok(ReferencePriceType::Average),
            "2" => Ok(ReferencePriceType::BidOrAsk),
            &_ => Err(ParseEnumError),
        }
    }
}

impl Decodable for ReferencePriceType {}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
#[derive(Debug, Clone, Copy)]
pub enum BasisPointsType {
    Undefined,
}

impl Encodable for BasisPointsType {
    fn encode(&self) -> String {
        match self {
            BasisPointsType::Undefined => "?\0",
        }
        .to_string()
    }
}

impl FromStr for BasisPointsType {
    type Err = ParseEnumError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "?" => Ok(BasisPointsType::Undefined),
            &_ => Err(ParseEnumError),
        }
    }
}

impl Decodable for BasisPointsType {}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
#[derive(PartialEq, Debug, Clone, Copy, Eq)]
pub enum HedgeType {
    Undefined,
    Delta,
    Beta,
    Forex,
    Pair,
}

impl Encodable for HedgeType {
    fn encode(&self) -> String {
        match self {
            HedgeType::Undefined => "?\0",
            HedgeType::Delta => "D\0",
            HedgeType::Beta => "B\0",
            HedgeType::Forex => "F\0",
            HedgeType::Pair => "P\0",
        }
        .to_string()
    }
}

impl FromStr for HedgeType {
    type Err = ParseEnumError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "?" => Ok(HedgeType::Undefined),
            "D" => Ok(HedgeType::Delta),
            "B" => Ok(HedgeType::Beta),
            "F" => Ok(HedgeType::Forex),
            "P" => Ok(HedgeType::Pair),
            &_ => Err(ParseEnumError),
        }
    }
}

impl Decodable for HedgeType {}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
#[derive(Debug, Clone, Copy)]
pub enum ClearingIntent {
    InteractiveBrokers,
    Away,
    PTA,
}

impl Encodable for ClearingIntent {
    fn encode(&self) -> String {
        match self {
            ClearingIntent::InteractiveBrokers => "IB\0",
            ClearingIntent::Away => "Away\0",
            ClearingIntent::PTA => "PTA\0",
        }
        .to_string()
    }
}

impl FromStr for ClearingIntent {
    type Err = ParseEnumError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "IB" => Ok(ClearingIntent::InteractiveBrokers),
            "Away" => Ok(ClearingIntent::Away),
            "PTA" => Ok(ClearingIntent::PTA),
            &_ => Err(ParseEnumError),
        }
    }
}

impl Decodable for ClearingIntent {}
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
#[derive(Debug, Clone, Copy)]
pub enum Side {
    Buy,
    Sell,
}

impl Encodable for Side {
    fn encode(&self) -> String {
        match self {
            Side::Buy => "BOT\0",
            Side::Sell => "SLD\0",
        }
        .to_string()
    }
}

impl FromStr for Side {
    type Err = ParseEnumError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "BOT" => Ok(Side::Buy),
            "SLD" => Ok(Side::Sell),
            "BUY" => Ok(Side::Sell),
            "SELL" => Ok(Side::Sell),
            &_ => Err(ParseEnumError),
        }
    }
}

impl Decodable for Side {}

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

impl Decodable for OrderConditionType {}
