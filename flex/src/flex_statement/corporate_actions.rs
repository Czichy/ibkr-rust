use chrono::{NaiveDate, NaiveDateTime};
use iso_currency::Currency;
// use ibkr_rust_api::prelude::*;
use rust_decimal::Decimal;
use serde::Deserialize;

use crate::{enums::{notes_deserialize, open_close_deserialize, OrderType, *},
            flex_statement::contract::Contract,
            trades::Trade,
            utils::de::{deserialize_from_str,
                        deserialize_option_from_str,
                        naive_date_from_str,
                        naive_date_time_from_str,
                        some_naive_date_from_str,
                        some_naive_date_time_from_str}};

#[derive(Debug, Deserialize, Clone)]
pub struct CorporateActions {
    #[serde(rename = "$value")]
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub items: Vec<CorporateAction>,
}
#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CorporateAction {
    #[serde(rename = "@accountId")]
    pub account_id: String,

    #[serde(deserialize_with = "deserialize_option_from_str")]
    #[serde(rename = "@acctAlias")]
    pub acct_alias: Option<String>,

    #[serde(deserialize_with = "deserialize_option_from_str")]
    #[serde(rename = "@model")]
    pub model: Option<String>,

    #[serde(flatten)]
    pub contract: Contract,

    #[serde(deserialize_with = "deserialize_from_str")]
    #[serde(rename = "@currency")]
    pub currency: Currency,

    #[serde(rename = "@actionDescription")]
    pub action_description: String,

    // #[serde(rename = "@fxRateToBase")]
    // pub fx_rate_to_base: Option<Decimal>,
    #[serde(rename = "@type")]
    #[serde(deserialize_with = "deserialize_from_str")]
    pub action_type: ActionType,

    #[serde(rename = "@amount")]
    #[serde(deserialize_with = "deserialize_option_from_str")]
    pub amount: Option<Decimal>,

    // #[serde(rename = "@tradeID")]
    // #[serde(deserialize_with = "deserialize_option_from_str")]
    // pub trade_id: Option<i32>,

    // #[serde(rename = "@ibOrderID")]
    // #[serde(deserialize_with = "deserialize_option_from_str")]
    // pub ib_order_id: Option<u32>,

    // #[serde(rename = "@ibExecID")]
    // #[serde(deserialize_with = "deserialize_option_from_str")]
    // pub ib_exec_id: Option<String>,

    // #[serde(rename = "@brokerageOrderID")]
    // #[serde(deserialize_with = "deserialize_option_from_str")]
    // pub brokerage_order_id: Option<String>,

    // #[serde(deserialize_with = "deserialize_option_from_str")]
    // #[serde(rename = "@orderReference")]
    // pub order_reference: Option<String>,

    // #[serde(rename = "@volatilityOrderLink")]
    // #[serde(deserialize_with = "deserialize_option_from_str")]
    // pub volatility_order_link: Option<String>,

    // #[serde(rename = "@clearingFirmID")]
    // #[serde(deserialize_with = "deserialize_option_from_str")]
    // pub clearing_firm_id: Option<String>,

    // #[serde(rename = "@origTradePrice")]
    // pub orig_trade_price: Option<Decimal>,

    // // pub orig_trade_date: Option<NaiveDateTime>,
    // #[serde(rename = "@origTradeID")]
    // #[serde(deserialize_with = "deserialize_option_from_str")]
    // pub orig_trade_id: Option<i32>,

    // #[serde(deserialize_with = "some_naive_date_time_from_str")]
    // #[serde(rename = "@orderTime")]
    // pub order_time: Option<NaiveDateTime>,

    // #[serde(deserialize_with = "some_naive_date_time_from_str")]
    // #[serde(rename = "@openDateTime")]
    // pub open_date_time: Option<NaiveDateTime>,

    // #[serde(rename = "@dateTime")]
    // #[serde(deserialize_with = "naive_date_time_from_str")]
    // pub trade_date_time: NaiveDateTime,

    // Note: The reportDate XML attribute may contain either a date or a String, i.e.     //
    // reportDate="MULTI"
    #[serde(rename = "@reportDate")]
    #[serde(deserialize_with = "some_naive_date_from_str")]
    pub report_date: Option<NaiveDate>,

    // Note: The settleDateTarget XML attribute may contain either a date or a String, i.e.     //
    // settleDateTarget="MULTI" #[serde(rename = "@settleDateTarget")]
    // #[serde(deserialize_with = "some_naive_date_from_str")]
    // pub settle_date_target: Option<NaiveDate>,
    // // Note: The tradeDate XML attribute may contain either a date or a String, i.e.     //
    // tradeDate="MULTI" #[serde(deserialize_with = "naive_date_from_str")]
    // #[serde(rename = "@tradeDate")]
    // pub trade_date:         NaiveDate,

    // #[serde(rename = "@exchange")]
    // pub exchange: String,
    #[serde(rename = "@transactionID")]
    pub transaction_id: String,

    // #[serde(rename = "@buySell")]
    // pub buy_sell: BuySell,

    // alternative format
    #[serde(rename = "@quantity")]
    pub quantity: Decimal,

    // #[serde(rename = "@tradePrice")]
    // // pub trade_price: Decimal,

    // #[serde(rename = "@tradeMoney")]
    // pub trade_money: Decimal,
    #[serde(rename = "@proceeds")]
    pub proceeds: Decimal,

    // #[serde(rename = "@ibCommission")]
    // pub ib_commission: Decimal,

    // #[serde(deserialize_with = "deserialize_option_from_str")]
    // #[serde(rename = "@exchOrderId")]
    // pub exch_order_id: Option<String>,

    // #[serde(rename = "@extExecID")]
    // #[serde(deserialize_with = "deserialize_option_from_str")]
    // pub ext_exec_id: Option<String>,

    // #[serde(default)]
    // #[serde(deserialize_with = "some_naive_date_time_from_str")]
    // #[serde(rename = "@holdingPeriodDateTime")]
    // pub holding_period_date_time: Option<NaiveDateTime>,

    // #[serde(deserialize_with = "deserialize_option_from_str")]
    // #[serde(rename = "@whenRealized")]
    // pub when_realized: Option<String>,

    // #[serde(deserialize_with = "deserialize_option_from_str")]
    // #[serde(rename = "@whenReopened")]
    // pub when_reopened: Option<String>,
    #[serde(rename = "@levelOfDetail")]
    pub level_of_detail: String,

    // #[serde(rename = "@changeInPrice")]
    // pub change_in_price: Option<Decimal>,

    // #[serde(rename = "@changeInQuantity")]
    // pub change_in_quantity: Option<Decimal>,

    // #[serde(deserialize_with = "deserialize_from_str")]
    // #[serde(rename = "@orderType")]
    // pub order_type: OrderType,

    // #[serde(rename = "@isAPIOrder")]
    // pub is_api_order: String,

    // #[serde(rename = "@accruedInterest")]
    // pub accrued_interest: Option<Decimal>,

    // #[serde(rename = "@traderID")]
    // pub trader_id: String,

    // #[serde(rename = "@taxes")]
    // pub taxes: Decimal,

    // #[serde(rename = "@ibCommissionCurrency")]
    // pub ib_commission_currency: Option<Currency>,

    // #[serde(rename = "@netCash")]
    // pub net_cash: Decimal,

    // #[serde(rename = "@closePrice")]
    // pub close_price: Decimal,

    // #[serde(deserialize_with = "open_close_deserialize")]
    // // #[serde(deserialize_with = "deserialize_option_from_str")]
    // #[serde(rename = "@openCloseIndicator")]
    // pub open_close_indicator: Option<OpenClose>,

    // #[serde(deserialize_with = "notes_deserialize")]
    // #[serde(rename = "@notes")]
    // pub notes: Vec<Notes>,

    // #[serde(rename = "@cost")]
    // pub cost: Decimal,
    #[serde(rename = "@fifoPnlRealized")]
    pub fifo_pnl_realized: Decimal,

    // #[serde(rename = "@fxPnl")]
    // pub fx_pnl: Option<Decimal>,
    #[serde(rename = "@mtmPnl")]
    pub mtm_pnl: Option<Decimal>,
    // #[serde(rename = "@origOrderID")]
    // pub orig_order_id: Option<i64>,
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
    fn flex_deserialize_corporate_action() {
        let xml = r#"<CorporateActions>
        <CorporateAction accountId="U11213636" acctAlias="" model="" currency="EUR" fxRateToBase="1" assetCategory="BOND" subCategory="" symbol="DBR 1 3/4 02/15/24" description="(DE0001102333) BOND MATURITY FOR EUR 1.00 PER BOND (DBR 1 3/4 02/15/24, DBR 1 3/4 02/15/24, DE0001102333)" conid="142870711" securityID="DE0001102333" securityIDType="ISIN" cusip="" isin="DE0001102333" figi="BBG005WQQ0X0" listingExchange="" underlyingConid="" underlyingSymbol="" underlyingSecurityID="" underlyingListingExchange="" issuer="" issuerCountryCode="DE" multiplier="1" strike="" expiry="" putCall="" principalAdjustFactor="1" reportDate="2024-02-15" dateTime="2024-02-14;20:25:00" actionDescription="(DE0001102333) BOND MATURITY FOR EUR 1.00 PER BOND (DBR 1 3/4 02/15/24, DBR 1 3/4 02/15/24, DE0001102333)" amount="-15000" proceeds="15000" value="0" quantity="-15000" fifoPnlRealized="130" mtmPnl="0" code="" type="BM" transactionID="2495820369" actionID="134853519" levelOfDetail="DETAIL" serialNumber="" deliveryType="" commodityType="" fineness="0.0" weight="0.0"/>
        </CorporateActions>
"#;
        let response: CorporateActions = from_str(xml).unwrap();
        assert_eq!(
            vec![CorporateAction {
                account_id:         "U11213636".to_string(),
                acct_alias:         None,
                model:              None,
                contract:           Contract {
                    asset_category:              AssetCategory::BOND,
                    symbol:                      "DBR 1 3/4 02/15/24".to_string(),
                    description:                 "(DE0001102333) BOND MATURITY FOR EUR 1.00 PER \
                                                  BOND (DBR 1 3/4 02/15/24, DBR 1 3/4 02/15/24, \
                                                  DE0001102333)"
                        .to_string(),
                    con_id:                      142870711,
                    security_id:                 Some("DE0001102333".to_string()),
                    security_id_type:            Some(SecIdType::Isin),
                    cusip:                       None,
                    isin:                        Some("DE0001102333".to_string()),
                    listing_exchange:            "".to_string(),
                    underlying_con_id:           None,
                    underlying_symbol:           None,
                    underlying_security_id:      None,
                    underlying_listing_exchange: None,
                    issuer:                      None,
                    multiplier:                  Some(Decimal::ONE),
                    strike:                      None,
                    expiry:                      None,
                    put_call:                    None,
                    principal_adjust_factor:     Some("1".to_string()),
                },
                currency:           Currency::EUR,
                fifo_pnl_realized:  dec!(130),
                mtm_pnl:            Some(Decimal::ZERO),
                action_description: "(DE0001102333) BOND MATURITY FOR EUR 1.00 PER BOND (DBR 1 \
                                     3/4 02/15/24, DBR 1 3/4 02/15/24, DE0001102333)"
                    .to_string(),
                action_type:        ActionType::BondMaturity,
                report_date:        NaiveDate::from_str("2024-02-15").ok(),
                transaction_id:     "2495820369".to_string(),
                level_of_detail:    "DETAIL".to_string(),
                amount:             dec!(-15000).into(),
                quantity:           dec!(-15000),
                proceeds:           dec!(15000),
            }],
            response.items
        );
    }
    // assert_eq!(response.contract.symbol.unwrap(), "VWRL");
}
