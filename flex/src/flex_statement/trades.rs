use chrono::{NaiveDate, NaiveDateTime};
use iso_currency::Currency;
// use ibkr_rust_api::prelude::*;
use rust_decimal::Decimal;
use serde::Deserialize;

use crate::{enums::{notes_deserialize, open_close_deserialize, OrderType, *},
            flex_statement::contract::Contract,
            utils::de::{deserialize_from_str,
                        deserialize_option_from_str,
                        naive_date_from_str,
                        naive_date_time_from_str,
                        some_naive_date_time_from_str}};

#[derive(Debug, Deserialize, Clone)]
pub struct Trades {
    #[serde(rename = "$value")]
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub items: Vec<TradeElements>,
}
#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub enum TradeElements {
    Trade(Trade),
    Order(Trade),
}
impl TradeElements {
    pub fn trade(self) -> Option<Trade> {
        match self {
            TradeElements::Trade(t) => Some(t),
            _ => None,
        }
    }

    pub fn order(self) -> Option<Trade> {
        match self {
            TradeElements::Order(t) => Some(t),
            _ => None,
        }
    }
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Trade {
    #[serde(rename = "accountId")]
    pub account_id: String,

    #[serde(deserialize_with = "deserialize_option_from_str")]
    pub acct_alias: Option<String>,

    #[serde(deserialize_with = "deserialize_option_from_str")]
    pub model: Option<String>,

    #[serde(flatten)]
    pub contract: Contract,

    pub fx_rate_to_base: Option<Decimal>,

    pub transaction_type: String,

    #[serde(rename = "tradeID")]
    #[serde(deserialize_with = "deserialize_option_from_str")]
    pub trade_id: Option<i32>,

    #[serde(rename = "ibOrderID")]
    #[serde(deserialize_with = "deserialize_option_from_str")]
    pub ib_order_id: Option<u32>,

    #[serde(rename = "ibExecID")]
    #[serde(deserialize_with = "deserialize_option_from_str")]
    pub ib_exec_id: Option<String>,

    #[serde(rename = "brokerageOrderID")]
    #[serde(deserialize_with = "deserialize_option_from_str")]
    pub brokerage_order_id: Option<String>,

    #[serde(deserialize_with = "deserialize_option_from_str")]
    pub order_reference: Option<String>,

    pub volatility_order_link: Option<String>,

    #[serde(rename = "clearingFirmID")]
    #[serde(deserialize_with = "deserialize_option_from_str")]
    pub clearing_firm_id: Option<String>,

    pub orig_trade_price: Option<Decimal>,

    // pub orig_trade_date: Option<NaiveDateTime>,
    #[serde(rename = "origTradeID")]
    #[serde(deserialize_with = "deserialize_option_from_str")]
    pub orig_trade_id: Option<i32>,

    #[serde(deserialize_with = "some_naive_date_time_from_str")]
    pub order_time: Option<NaiveDateTime>,

    #[serde(deserialize_with = "some_naive_date_time_from_str")]
    pub open_date_time: Option<NaiveDateTime>,

    #[serde(rename = "dateTime")]
    #[serde(deserialize_with = "naive_date_time_from_str")]
    pub trade_date_time: NaiveDateTime,

    // Note: The reportDate XML attribute may contain either a date or aString, i.e.
    // reportDate="MULTI"
    pub report_date:        String,
    // Note: The settleDateTarget XML attribute may contain either a date or aString, i.e.
    // settleDateTarget="MULTI"
    pub settle_date_target: String,
    // Note: The tradeDate XML attribute may contain either a date or aString, i.e.
    // tradeDate="MULTI"
    #[serde(deserialize_with = "naive_date_from_str")]
    pub trade_date:         NaiveDate,

    pub exchange:       String,
    #[serde(rename = "transactionID")]
    pub transaction_id: String,

    pub buy_sell: BuySell,

    // alternative format
    pub quantity: Decimal,

    pub trade_price: Decimal,

    pub trade_money: Decimal,

    pub proceeds: Decimal,

    pub ib_commission: Decimal,

    #[serde(deserialize_with = "deserialize_option_from_str")]
    pub exch_order_id: Option<String>,

    #[serde(rename = "extExecID")]
    #[serde(deserialize_with = "deserialize_option_from_str")]
    pub ext_exec_id: Option<String>,

    #[serde(default)]
    #[serde(deserialize_with = "some_naive_date_time_from_str")]
    pub holding_period_date_time: Option<NaiveDateTime>,

    #[serde(deserialize_with = "deserialize_option_from_str")]
    pub when_realized: Option<String>,

    #[serde(deserialize_with = "deserialize_option_from_str")]
    pub when_reopened: Option<String>,

    pub level_of_detail: String,

    pub change_in_price: Option<Decimal>,

    pub change_in_quantity: Option<Decimal>,

    #[serde(deserialize_with = "deserialize_from_str")]
    pub order_type: OrderType,

    #[serde(rename = "isAPIOrder")]
    pub is_api_order: String,

    pub accrued_interest: Option<Decimal>,

    #[serde(rename = "traderID")]
    pub trader_id: String,

    pub taxes: Decimal,

    pub ib_commission_currency: Option<Currency>,

    pub net_cash: Decimal,

    pub close_price: Decimal,

    #[serde(deserialize_with = "open_close_deserialize")]
    pub open_close_indicator: Vec<OpenClose>,

    #[serde(deserialize_with = "notes_deserialize")]
    pub notes: Vec<Notes>,

    pub cost: Decimal,

    pub fifo_pnl_realized: Decimal,

    pub fx_pnl: Option<Decimal>,

    pub mtm_pnl: Option<Decimal>,

    #[serde(rename = "origOrderID")]
    pub orig_order_id: Option<i64>,
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use chrono::NaiveDateTime;
    use pretty_assertions::assert_eq;
    use quick_xml::de::from_str;
    use rust_decimal_macros::dec;

    use super::*;
    use crate::{enums::{AssetCategory, SecIdType},
                flex_statement::{contract::Contract, trades::Trade}};

    #[test]
    fn flex_deserialize_trade() {
        let xml = r#"
<Trade accountId="U7502027" acctAlias="" model="" currency="EUR" fxRateToBase="1" 
        assetCategory="STK" 
        symbol="VWRL" 
        description="VANG FTSE AW USDD" conid="128831206" securityID="IE00B3RBWM25" securityIDType="ISIN"
        cusip="" isin="IE00B3RBWM25" listingExchange="AEB" underlyingConid="" underlyingSymbol=""
        underlyingSecurityID="" underlyingListingExchange="" issuer="" multiplier="1" strike="" expiry=""
   tradeID="29464420" putCall="" reportDate="2021-02-26" principalAdjustFactor="" dateTime="2021-02-26;10:01:35" tradeDate="2021-02-26" settleDateTarget="2021-03-02" transactionType="ExchTrade" exchange="AEB" quantity="10" tradePrice="89.5" tradeMoney="895" proceeds="-895" taxes="0" 
   ibCommission="-4" ibCommissionCurrency="EUR" 
   netCash="-899" closePrice="89.9" openCloseIndicator="O" notes="" cost="899" fifoPnlRealized="0" fxPnl="0" mtmPnl="4" origTradePrice="0" origTradeDate="" origTradeID="" origOrderID="0" clearingFirmID="" transactionID="85594826" buySell="BUY" ibOrderID="23947654" ibExecID="0000e0c2.60389192.01.01" brokerageOrderID="0004f96a.00014c43.603893de.0001" orderReference="" volatilityOrderLink="" exchOrderId="N/A" extExecID="669953IE00B3RBWM25/B" orderTime="2021-02-26;09:22:05" openDateTime="" holdingPeriodDateTime="" whenRealized="" whenReopened="" levelOfDetail="EXECUTION" changeInPrice="0" changeInQuantity="0" orderType="LMT" traderID="" isAPIOrder="N" accruedInt="0" serialNumber="" deliveryType="" commodityType="" fineness="0.0" weight="0.0 ()"/>
"#;
        let response: Trade = from_str(xml).unwrap();
        assert_eq!(
            Trade {
                account_id:               "U7502027".to_string(),
                acct_alias:               None,
                model:                    None,
                contract:                 Contract {
                    asset_category:              AssetCategory::STK,
                    symbol:                      "VWRL".to_string(),
                    currency:                    Currency::EUR,
                    description:                 "VANG FTSE AW USDD".to_string(),
                    con_id:                      128831206,
                    security_id:                 Some("IE00B3RBWM25".to_string()),
                    security_id_type:            Some(SecIdType::Isin),
                    cusip:                       None,
                    isin:                        Some("IE00B3RBWM25".to_string()),
                    listing_exchange:            "AEB".to_string(),
                    underlying_con_id:           None,
                    underlying_symbol:           None,
                    underlying_security_id:      None,
                    underlying_listing_exchange: None,
                    issuer:                      None,
                    multiplier:                  Some(Decimal::new(100, 2)),
                    strike:                      None,
                    expiry:                      None,
                    put_call:                    None,
                    principal_adjust_factor:     None,
                },
                fx_rate_to_base:          Some(dec!(1)),
                transaction_type:         "ExchTrade".to_string(),
                trade_id:                 Some(29464420),
                ib_order_id:              Some(23947654),
                ib_exec_id:               Some("0000e0c2.60389192.01.01".to_string()),
                brokerage_order_id:       Some("0004f96a.00014c43.603893de.0001".to_string()),
                order_reference:          None,
                volatility_order_link:    None,
                clearing_firm_id:         None,
                orig_trade_price:         Some(dec!(0)),
                orig_trade_id:            None,
                order_time:               NaiveDateTime::from_str("2021-02-26T09:22:05").ok(),
                open_date_time:           None,
                trade_date_time:          NaiveDateTime::from_str("2021-02-26T10:01:35").unwrap(),
                report_date:              "2021-02-26".to_string(),
                settle_date_target:       "2021-03-02".to_string(),
                trade_date:               NaiveDate::from_str("2021-02-26").unwrap(),
                exchange:                 "AEB".to_string(),
                transaction_id:           "85594826".to_string(),
                buy_sell:                 crate::enums::BuySell::BUY,
                quantity:                 dec!(10),
                trade_price:              dec!(89.5),
                trade_money:              dec!(895),
                proceeds:                 dec!(-895),
                ib_commission:            dec!(-4),
                exch_order_id:            Some("N/A".to_string()),
                ext_exec_id:              Some("669953IE00B3RBWM25/B".to_string()),
                holding_period_date_time: None,
                when_realized:            None,
                when_reopened:            None,
                level_of_detail:          "EXECUTION".to_string(),
                change_in_price:          Some(dec!(0)),
                change_in_quantity:       Some(dec!(0)),
                order_type:               OrderType::Limit,
                is_api_order:             "N".to_string(),
                accrued_interest:         None,
                trader_id:                "".to_string(),
                taxes:                    dec!(0),
                ib_commission_currency:   Some(Currency::EUR),
                net_cash:                 dec!(-899),
                close_price:              dec!(89.9),
                open_close_indicator:     vec![OpenClose::O],
                notes:                    vec![],
                cost:                     dec!(899),
                fifo_pnl_realized:        dec!(0),
                fx_pnl:                   Some(dec!(0)),
                mtm_pnl:                  Some(dec!(4)),
                orig_order_id:            Some(0),
            },
            response
        );
    }
    // assert_eq!(response.contract.symbol.unwrap(), "VWRL");
}
