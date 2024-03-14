use std::{fmt::{Display, Formatter},
          str::{FromStr, Split}};

use chrono::{DateTime, NaiveDateTime, TimeZone};
use chrono_tz::{Tz, US, UTC};
use rust_decimal::prelude::*;

// #[cfg(feature = "serde")]
// use serde::{Deserialize, Serialize};
use crate::{enums::*,
            ib_frame::{ParseError, ParseIbkrFrame, ParseResult},
            order::{ComboAction, OptionOpenClose, ShortSaleSlot},
            prelude::ib_message::{decode, Decodable},
            utils::ib_message::Encodable};
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ComboLeg {
    pub con_id:              i32,
    pub ratio:               i32,
    pub action:              ComboAction,
    pub exchange:            String,
    pub open_close:          Option<OptionOpenClose>,
    pub shortsale_slot:      Option<ShortSaleSlot>,
    pub designated_location: Option<String>,
    pub exempt_code:         Option<i32>,
}

impl ComboLeg {
    pub fn new(con_id: i32, ratio: i32, action: ComboAction, exchange: &str) -> ComboLeg {
        ComboLeg {
            con_id,
            ratio,
            action,
            exchange: exchange.to_string(),
            open_close: None,
            shortsale_slot: None,
            designated_location: None,
            exempt_code: None,
        }
    }
}
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DeltaNeutralContract {
    pub con_id: i32,
    pub delta:  Decimal,
    pub price:  Decimal,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct Contract {
    pub con_id:                            Option<i32>,
    pub symbol:                            String,
    pub sec_type:                          SecType,
    pub last_trade_date_or_contract_month: Option<String>,
    pub strike:                            Option<Decimal>,
    pub right:                             Option<OptionRight>,
    pub multiplier:                        Option<Decimal>,
    pub exchange:                          Option<String>,
    pub currency:                          String,
    pub local_symbol:                      Option<String>,
    pub primary_exchange:                  Option<String>,
    pub trading_class:                     Option<String>,
    pub include_expired:                   Option<bool>,
    pub sec_id_type:                       Option<SecIdType>,
    pub sec_id:                            Option<String>,
    pub combo_legs_description:            Option<String>,
    pub combo_legs:                        Option<Vec<ComboLeg>>,
    pub delta_neutral_contract:            Option<DeltaNeutralContract>,
    pub issuer_id:                         Option<String>,
}

impl Encodable for Contract {
    fn encode(&self) -> String {
        let mut code = String::new();
        code.push_str(&self.con_id.encode());
        code.push_str(&self.symbol.encode());
        code.push_str(&self.sec_type.encode());
        code.push_str(&self.last_trade_date_or_contract_month.encode());
        code.push_str(&self.strike.encode());
        code.push_str(&self.right.encode());
        code.push_str(&self.multiplier.encode());
        code.push_str(&self.exchange.encode());
        code.push_str(&self.primary_exchange.encode());
        code.push_str(&self.currency.encode());
        code.push_str(&self.local_symbol.encode());
        code.push_str(&self.trading_class.encode());
        code.push_str(&self.include_expired.encode());
        code.push_str(&self.sec_id_type.encode());
        code.push_str(&self.sec_id.encode());
        code.push_str(&self.issuer_id.encode());
        code
    }
}

impl ParseIbkrFrame for Contract {
    fn try_parse_frame(msg_id: Incoming, it: &mut Split<&str>) -> ParseResult<Self>
    where
        Self: Sized,
    {
        match msg_id {
            Incoming::ContractData => {
                tracing::debug!("decode ContractData");
                Ok(Self {
                    symbol: decode(it)?.unwrap(),
                    sec_type: decode(it)?.unwrap(),
                    last_trade_date_or_contract_month: decode(it)?,
                    strike: decode(it)?,
                    right: decode(it)?,
                    exchange: decode(it)?,
                    currency: decode(it)?.unwrap(),
                    local_symbol: decode(it)?,
                    // issuer_id: decode(it)?,
                    ..Default::default()
                })
            },
            Incoming::OpenOrder
            | Incoming::CompletedOrder
            | Incoming::PortfolioValue
            | Incoming::ExecutionData => {
                Ok(Contract {
                    con_id: decode(it)?,
                    symbol: decode(it)?.unwrap(),
                    sec_type: decode(it)?.unwrap(),
                    last_trade_date_or_contract_month: decode(it)?,
                    strike: decode(it)?,
                    right: decode(it)?,
                    multiplier: decode(it)?,
                    exchange: decode(it)?,
                    currency: decode(it)?.unwrap(),
                    local_symbol: decode(it)?,
                    trading_class: decode(it)?,
                    ..Default::default()
                })
            },
            _ => Err(ParseError::Incomplete),
        }
    }
}

impl Contract {
    pub fn encode_for_order(&self) -> String {
        let mut code = String::new();
        code.push_str(&self.con_id.encode());
        code.push_str(&self.symbol.encode());
        code.push_str(&self.sec_type.encode());
        code.push_str(&self.last_trade_date_or_contract_month.encode());
        code.push_str(&self.strike.encode());
        code.push_str(&self.right.encode());
        code.push_str(&self.multiplier.encode());
        code.push_str(&self.exchange.encode());
        code.push_str(&self.primary_exchange.encode());
        code.push_str(&self.currency.encode());
        code.push_str(&self.local_symbol.encode());
        code.push_str(&self.trading_class.encode());
        code.push_str(&self.sec_id_type.encode());
        code.push_str(&self.sec_id.encode());
        code
    }

    pub fn encode_for_ticker(&self) -> String {
        let mut code = String::new();
        code.push_str(&self.con_id.encode());
        code.push_str(&self.symbol.encode());
        code.push_str(&self.sec_type.encode());
        code.push_str(&self.last_trade_date_or_contract_month.encode());
        code.push_str(&self.strike.encode());
        code.push_str(&self.right.encode());
        code.push_str(&self.multiplier.encode());
        code.push_str(&self.exchange.encode());
        code.push_str(&self.primary_exchange.encode());
        code.push_str(&self.currency.encode());
        code.push_str(&self.local_symbol.encode());
        code.push_str(&self.trading_class.encode());
        code
    }

    pub fn encode_for_hist_data(&self) -> String {
        let mut code = String::new();
        code.push_str(&self.con_id.encode());
        code.push_str(&self.symbol.encode());
        code.push_str(&self.sec_type.encode());
        code.push_str(&self.last_trade_date_or_contract_month.encode());
        code.push_str(&self.strike.encode());
        code.push_str(&self.right.encode());
        code.push_str(&self.multiplier.encode());
        code.push_str(&self.exchange.encode());
        code.push_str(&self.primary_exchange.encode());
        code.push_str(&self.currency.encode());
        code.push_str(&self.local_symbol.encode());
        code.push_str(&self.trading_class.encode());
        code.push_str(&self.include_expired.encode());
        code
    }

    // pub fn stock_spread_smart_usd(
    //    contract_1: &Contract,
    //    ratio_1: i32,
    //    contract_2: &Contract,
    //    ratio_2: i32,
    //) -> Option<Contract> {
    //    let mut ret = None;
    //    // if let Some(con_id_1) = contract_1.con_id {
    //    //    if let Some(con_id_2) = contract_2.con_id {
    //    //        if let Some(symbol_1) = &contract_1.symbol {
    //    //            if let Some(symbol_2) = &contract_2.symbol {
    //    let legs = vec![
    //        ComboLeg::new(contract_1.con_id, ratio_1, ComboAction::Buy, "SMART"),
    // // IBKR        ComboLeg::new(contract_2.con_id, ratio_2,
    // ComboAction::Sell, "SMART"),    ]; // MCD
    //    ret = Some(Contract {
    //        symbol: contract_1.symbol.clone() + "," + &contract_2.symbol,
    //        exchange: Some("SMART".to_string()),
    //        sec_type: SecType::Combo,
    //        currency: "USD".to_string(),
    //        combo_legs: Some(legs),
    //        ..Default::default()
    //    });
    //    //            }
    //    //        }
    //    //    }
    //    //}
    //    ret
    //}
}

#[derive(Default, Debug, Clone)]
pub struct ContractDetails {
    pub contract:             Contract,
    pub market_name:          Option<String>,
    pub min_tick:             Option<f64>,
    pub price_magnifier:      Option<i32>,
    pub order_types:          Option<String>,
    pub valid_exchanges:      Option<String>,
    pub under_con_id:         Option<i32>,
    pub long_name:            Option<String>,
    pub contract_month:       Option<String>,
    pub industry:             Option<String>,
    pub category:             Option<String>,
    pub subcategory:          Option<String>,
    pub timezone_id:          Option<String>,
    pub trading_hours:        Option<String>,
    pub liquid_hours:         Option<String>,
    pub ev_rule:              Option<String>,
    pub ev_multiplier:        Option<String>,
    pub md_size_multiplier:   Option<String>,
    pub agg_group:            Option<i32>,
    pub sec_id_list:          Option<Vec<(String, String)>>,
    pub under_symbol:         Option<String>,
    pub under_sec_type:       Option<SecType>,
    pub market_rule_ids:      Option<String>,
    pub real_expiration_date: Option<String>,
    pub last_trade_time:      Option<String>,
    pub stock_type:           Option<String>,
    pub cusip:                Option<String>,
    pub ratings:              Option<String>,
    pub desc_append:          Option<String>,
    pub bond_type:            Option<String>,
    pub coupon_type:          Option<String>,
    pub callable:             Option<bool>,
    pub putable:              Option<bool>,
    pub coupon:               Option<bool>,
    pub convertible:          Option<bool>,
    pub maturity:             Option<bool>,
    pub issue_date:           Option<bool>,
    pub next_option_date:     Option<bool>,
    pub next_option_type:     Option<bool>,
    pub notes:                Option<String>,
}

impl ParseIbkrFrame for ContractDetails {
    fn try_parse_frame(msg_id: Incoming, it: &mut Split<&str>) -> ParseResult<Self>
    where
        Self: Sized,
    {
        match msg_id {
            Incoming::ContractData => {
                tracing::debug!("decode ContractData");
                let mut contract = Contract::try_parse_frame(msg_id, it)?;
                tracing::debug!("decoded contract: {:#?}", contract);
                let mut details = ContractDetails {
                    market_name: decode(it)?,
                    ..Default::default()
                };
                contract.trading_class = decode(it)?;
                // new field???
                // TODO: ab Version 183
                // let _: Option<String> = decode(it)?;
                contract.con_id = decode(it)?;
                details.min_tick = decode(it)?;
                contract.multiplier = decode(it)?;
                details.order_types = decode(it)?;
                details.valid_exchanges = decode(it)?;
                details.price_magnifier = decode(it)?;
                details.under_con_id = decode(it)?;
                details.long_name = decode(it)?;
                contract.primary_exchange = decode(it)?;
                details.contract_month = decode(it)?;
                details.industry = decode(it)?;
                details.category = decode(it)?;
                details.subcategory = decode(it)?;
                details.timezone_id = decode(it)?;
                details.trading_hours = decode(it)?;
                details.liquid_hours = decode(it)?;
                details.ev_rule = decode(it)?;
                details.ev_multiplier = decode(it)?;
                let sec_id_list_count: Option<usize> = decode(it)?;
                details.sec_id_list = match sec_id_list_count {
                    Some(count) => {
                        let mut sec_ids: Vec<(String, String)> = Vec::with_capacity(count);
                        for _i in 0..count {
                            sec_ids.push((decode(it)?.unwrap(), decode(it)?.unwrap()));
                        }
                        Some(sec_ids)
                    },
                    None => None,
                };
                details.agg_group = decode(it)?;
                details.under_symbol = decode(it)?;
                details.under_sec_type = decode(it)?;
                details.market_rule_ids = decode(it)?;
                details.real_expiration_date = decode(it)?;
                details.contract = contract;
                details.stock_type = decode(it)?;
                Ok(details)
            },
            _ => Err(ParseError::Incomplete),
        }
    }
}
impl ContractDetails {
    pub fn liquid_hours(&self) -> Option<Vec<(DateTime<Tz>, DateTime<Tz>)>> {
        let liq_hours_it = self.liquid_hours.as_ref()?.split(';');
        let mut ret = Vec::new();
        for liq_hours in liq_hours_it {
            if liq_hours == "CLOSED" {
                // ret.push(None);
            } else {
                let mut hours_it = liq_hours.split('-');
                let open_dt =
                    NaiveDateTime::parse_from_str(hours_it.next()?, "%Y%m%d:%H%M").unwrap();
                let close_dt =
                    NaiveDateTime::parse_from_str(hours_it.next()?, "%Y%m%d:%H%M").unwrap();
                if let Some(tz) = &self.timezone_id {
                    if tz.contains("EST") {
                        ret.push((
                            US::Eastern.from_local_datetime(&open_dt).unwrap(),
                            US::Eastern.from_local_datetime(&close_dt).unwrap(),
                        ));
                    } else {
                        ret.push((
                            UTC.from_local_datetime(&open_dt).unwrap(),
                            UTC.from_local_datetime(&close_dt).unwrap(),
                        ));
                    }
                }
            }
        }
        Some(ret)
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ContractDescription {
    contract:                  Option<Contract>,
    derivative_sec_types_list: Option<Vec<String>>,
}

pub type ContractDescriptionList = Vec<ContractDescription>;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum SecType {
    Stock,
    Option,
    Future,
    OptionOnFuture,
    Index,
    Forex,
    Combo,
    Warrant,
    Bond,
    Commodity,
    News,
    MutualFund,
}

impl Encodable for SecType {
    fn encode(&self) -> String {
        match self {
            SecType::Stock => "STK\0",
            SecType::Option => "OPT\0",
            SecType::Future => "FUT\0",
            SecType::OptionOnFuture => "FOP\0",
            SecType::Index => "IND\0",
            SecType::Forex => "CASH\0",
            SecType::Combo => "BAG\0",
            SecType::Warrant => "WAR\0",
            SecType::Bond => "BOND\0",
            SecType::Commodity => "CMDTY\0",
            SecType::News => "NEWS\0",
            SecType::MutualFund => "FUND\0",
        }
        .to_string()
    }
}
impl Default for SecType {
    fn default() -> Self { Self::Stock }
}

impl FromStr for SecType {
    type Err = ParseEnumError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let res = match s {
            "STK" => SecType::Stock,
            "OPT" => SecType::Option,
            "FUT" => SecType::Future,
            "FOP" => SecType::OptionOnFuture,
            "IND" => SecType::Index,
            "CASH" => SecType::Forex,
            "BAG" => SecType::Combo,
            "WAR" => SecType::Warrant,
            "BOND" => SecType::Bond,
            "CMDTY" => SecType::Commodity,
            "NEWS" => SecType::News,
            "FUND" => SecType::MutualFund,
            &_ => return Err(ParseEnumError),
        };
        Ok(res)
    }
}

impl Display for SecType {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result { write!(f, "{self:?}") }
}

impl Decodable for SecType {}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OptionRight {
    Undefined,
    Put,
    Call,
}

impl Encodable for OptionRight {
    fn encode(&self) -> String {
        match self {
            OptionRight::Undefined => "0\0",
            OptionRight::Put => "PUT\0",
            OptionRight::Call => "CALL\0",
        }
        .to_string()
    }
}

impl FromStr for OptionRight {
    type Err = ParseEnumError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let res = match s {
            "0" => OptionRight::Undefined,
            "?" => OptionRight::Undefined,
            "PUT" => OptionRight::Put,
            "P" => OptionRight::Put,
            "CALL" => OptionRight::Call,
            "C" => OptionRight::Call,
            &_ => return Err(ParseEnumError),
        };
        Ok(res)
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum SecIdType {
    Isin,
    Cusip,
}

impl Encodable for SecIdType {
    fn encode(&self) -> String {
        match self {
            SecIdType::Isin => "ISIN\0",
            SecIdType::Cusip => "CUSIP\0",
        }
        .to_string()
    }
}
impl FromStr for SecIdType {
    type Err = ParseEnumError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ISIN" => Ok(Self::Isin),
            "CUSIP" => Ok(Self::Cusip),
            &_ => Err(ParseEnumError),
        }
    }
}
