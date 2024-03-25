use chrono::NaiveDate;
use iso_currency::Currency;
use rust_decimal::Decimal;
use serde::Deserialize;

use crate::{flex_statement::contract::Contract, utils::de::*};

#[derive(Debug, Deserialize, Clone)]
pub struct StmtFunds {
    #[serde(rename = "$value")]
    // #[serde(rename = "StatementOfFundsLine", default)]
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub items: Vec<StatementOfFundsLine>,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
// TODO: Create Enume for different lines e.g. Activity = Starting Balance
pub struct StatementOfFundsLine {
    #[serde(rename = "@accountId")]
    pub account_id: String,

    #[serde(deserialize_with = "deserialize_option_from_str")]
    #[serde(rename = "@acctAlias")]
    pub acct_alias: Option<String>,

    #[serde(deserialize_with = "deserialize_option_from_str")]
    #[serde(rename = "@activityCode")]
    pub activity_code: Option<String>,

    #[serde(flatten)]
    pub contract: Option<Contract>,

    #[serde(deserialize_with = "deserialize_from_str")]
    #[serde(rename = "@currency")]
    pub currency: Currency,

    #[serde(rename = "@activityDescription")]
    pub activity_description: String,

    #[serde(rename = "@amount")]
    #[serde(deserialize_with = "deserialize_option_from_str")]
    pub amount: Option<Decimal>,

    #[serde(rename = "@balance")]
    #[serde(deserialize_with = "deserialize_option_from_str")]
    pub balance: Option<Decimal>,

    #[serde(rename = "@credit")]
    #[serde(deserialize_with = "deserialize_option_from_str")]
    pub credit: Option<Decimal>,

    #[serde(rename = "@date")]
    #[serde(deserialize_with = "naive_date_from_str")]
    pub date: NaiveDate,

    #[serde(rename = "@debit")]
    #[serde(deserialize_with = "deserialize_option_from_str")]
    pub debit: Option<Decimal>,

    #[serde(rename = "@fxRateToBase")]
    #[serde(deserialize_with = "deserialize_option_from_str")]
    pub fx_rate_to_base: Option<Decimal>,

    #[serde(rename = "@levelOfDetail")]
    pub level_of_detail: String,

    #[serde(rename = "@model")]
    #[serde(deserialize_with = "deserialize_option_from_str")]
    pub model: Option<String>,

    #[serde(rename = "@orderID")]
    #[serde(deserialize_with = "deserialize_option_from_str")]
    pub order_id: Option<u64>,

    #[serde(rename = "@reportDate")]
    #[serde(deserialize_with = "naive_date_from_str")]
    pub report_date: NaiveDate,

    #[serde(rename = "@settleDate")]
    #[serde(deserialize_with = "some_naive_date_from_str")]
    pub settle_date: Option<NaiveDate>,

    #[serde(rename = "@tradeCode")]
    #[serde(deserialize_with = "deserialize_option_from_str")]
    pub trade_code: Option<String>,

    #[serde(rename = "@tradeCommission")]
    #[serde(deserialize_with = "deserialize_option_from_str")]
    pub trade_commission: Option<Decimal>,

    #[serde(rename = "@tradeGross")]
    #[serde(deserialize_with = "deserialize_option_from_str")]
    pub trade_gross: Option<Decimal>,

    #[serde(rename = "@tradeID")]
    #[serde(deserialize_with = "deserialize_option_from_str")]
    pub trade_id: Option<String>,

    #[serde(rename = "@tradePrice")]
    #[serde(deserialize_with = "deserialize_option_from_str")]
    pub trade_price: Option<Decimal>,

    #[serde(rename = "@tradeQuantity")]
    #[serde(deserialize_with = "deserialize_option_from_str")]
    pub trade_quantity: Option<Decimal>,

    #[serde(rename = "@tradeTax")]
    #[serde(deserialize_with = "deserialize_option_from_str")]
    pub trade_tax: Option<Decimal>,

    #[serde(rename = "@transactionID")]
    #[serde(deserialize_with = "deserialize_option_from_str")]
    pub transaction_id: Option<u64>,
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use chrono::NaiveDate;
    use iso_currency::Currency;
    use pretty_assertions::assert_eq;
    use quick_xml::de::from_str;
    use rust_decimal_macros::dec;

    use super::*;

    #[test]
    fn flex_deserialize_stmt_funds() {
        let xml = r#"
<StmtFunds>
<StatementOfFundsLine accountId="U7502027" acctAlias="" model="" currency="EUR" fxRateToBase="1" assetCategory="" symbol="" description="" conid="" securityID="" securityIDType="" cusip="" isin="" listingExchange="" underlyingConid="" underlyingSymbol="" underlyingSecurityID="" underlyingListingExchange="" issuer="" multiplier="" strike="" expiry="" putCall="" principalAdjustFactor="" reportDate="2022-02-15" date="2022-02-15" settleDate="" activityCode="" activityDescription="Starting Balance" tradeID="" orderID="" buySell="" tradeQuantity="0" tradePrice="0" tradeGross="0" tradeCommission="0" tradeTax="0" debit="" credit="" amount="0" tradeCode="" balance="6655.368396105" levelOfDetail="BaseCurrency" transactionID="" serialNumber="" deliveryType="" commodityType="" fineness="" weight="" />
<StatementOfFundsLine accountId="U11213636" acctAlias="" model="" currency="EUR" fxRateToBase="1" assetCategory="CASH" symbol="" description="" conid="" securityID="" securityIDType="" cusip="" isin="" listingExchange="" underlyingConid="" underlyingSymbol="" underlyingSecurityID="" underlyingListingExchange="" issuer="" multiplier="0" strike="" expiry="" putCall="" principalAdjustFactor="" reportDate="2023-01-30" date="2023-01-30" settleDate="2023-01-31" activityCode="DEP" activityDescription="Cash Transfer" tradeID="" orderID="" buySell="" tradeQuantity="0" tradePrice="0" tradeGross="0" tradeCommission="0" tradeTax="0" debit="" credit="15724.15" amount="15724.15" tradeCode="" balance="15724.15" levelOfDetail="Currency" transactionID="1635131224"/>
<StatementOfFundsLine accountId="U7502027" acctAlias="" model="" currency="EUR" fxRateToBase="1" assetCategory="STK" symbol="F" description="FORD MOTOR CO" conid="9599491" securityID="US3453708600" securityIDType="ISIN" cusip="345370860" isin="US3453708600" listingExchange="NYSE" underlyingConid="" underlyingSymbol="" underlyingSecurityID="" underlyingListingExchange="" issuer="" multiplier="1" strike="" expiry="" putCall="" principalAdjustFactor="" reportDate="2022-02-14" date="2022-02-14" settleDate="2022-02-16" activityCode="BUY" activityDescription="Buy 100 FORD MOTOR CO " tradeID="336654462" orderID="299500993" buySell="BUY" tradeQuantity="100" tradePrice="17.61" tradeGross="-1557.49884" tradeCommission="-0.88444" tradeTax="0" debit="-1558.38328" credit="" amount="-1558.38328" tradeCode="P" balance="5103.133989064" levelOfDetail="BaseCurrency" transactionID="929316089" serialNumber="" deliveryType="" commodityType="" fineness="0.0" weight="0.0 ()"/>
</StmtFunds>

"#;

        let response: StmtFunds = from_str(xml).unwrap();
        tracing::warn!("StmtFunds:{:#?}", response);
        assert_eq!(
            vec![
                StatementOfFundsLine {
                    account_id:           "U7502027".to_string(),
                    acct_alias:           None,
                    activity_code:        None,
                    contract:             None,
                    currency:             Currency::EUR,
                    activity_description: "Starting Balance".to_string(),
                    amount:               Some(dec!(0),),
                    balance:              Some(dec!(6655.368396105),),
                    credit:               None,
                    date:                 NaiveDate::from_str("2022-02-15").unwrap(),
                    debit:                None,
                    fx_rate_to_base:      Some(dec!(1),),
                    level_of_detail:      "BaseCurrency".to_string(),
                    model:                None,
                    order_id:             None,
                    report_date:          NaiveDate::from_str("2022-02-15").unwrap(),
                    settle_date:          None,
                    trade_code:           None,
                    trade_commission:     Some(dec!(0),),
                    trade_gross:          Some(dec!(0)),
                    trade_id:             None,
                    trade_price:          Some(dec!(0),),
                    trade_quantity:       Some(dec!(0),),
                    trade_tax:            Some(dec!(0),),
                    transaction_id:       None,
                },
                StatementOfFundsLine {
                    account_id:           "U11213636".to_string(),
                    acct_alias:           None,
                    activity_code:        Some("DEP".to_string()),
                    contract:             None,
                    currency:             Currency::EUR,
                    activity_description: "Cash Transfer".to_string(),
                    amount:               Some(dec!(15724.15)),
                    balance:              Some(dec!(15724.15)),
                    credit:               Some(dec!(15724.15)),
                    date:                 NaiveDate::from_str("2023-01-30").unwrap(),
                    debit:                None,
                    fx_rate_to_base:      Some(dec!(1)),
                    level_of_detail:      "Currency".to_string(),
                    model:                None,
                    order_id:             None,
                    report_date:          NaiveDate::from_str("2023-01-30").unwrap(),
                    settle_date:          Some(NaiveDate::from_str("2023-01-31").unwrap()),
                    trade_code:           None,
                    trade_commission:     Some(dec!(0)),
                    trade_gross:          Some(dec!(0)),
                    trade_id:             None,
                    trade_price:          Some(dec!(0)),
                    trade_quantity:       Some(dec!(0)),
                    trade_tax:            Some(dec!(0)),
                    transaction_id:       Some(1635131224),
                },
                StatementOfFundsLine {
                    account_id:           "U7502027".to_string(),
                    acct_alias:           None,
                    activity_code:        Some("BUY".to_string()),
                    contract:             Some(Contract {
                        asset_category:              crate::enums::AssetCategory::STK,
                        symbol:                      "F".to_string(),
                        // currency:                    Currency::EUR,
                        description:                 "FORD MOTOR CO".to_string(),
                        con_id:                      9599491,
                        security_id:                 Some("US3453708600".to_string()),
                        security_id_type:            Some(crate::enums::SecIdType::Isin,),
                        cusip:                       Some("345370860".to_string()),
                        isin:                        Some("US3453708600".to_string()),
                        listing_exchange:            "NYSE".to_string(),
                        underlying_con_id:           None,
                        underlying_symbol:           None,
                        underlying_security_id:      None,
                        underlying_listing_exchange: None,
                        issuer:                      None,
                        multiplier:                  Some(dec!(1),),
                        strike:                      None,
                        expiry:                      None,
                        put_call:                    None,
                        principal_adjust_factor:     None,
                    },),
                    currency:             Currency::EUR,
                    activity_description: "Buy 100 FORD MOTOR CO ".to_string(),
                    amount:               Some(dec!(-1558.38328),),
                    balance:              Some(dec!(5103.133989064),),
                    credit:               None,
                    date:                 NaiveDate::from_str("2022-02-14").unwrap(),
                    debit:                Some(dec!(-1558.38328),),
                    fx_rate_to_base:      Some(dec!(1),),
                    level_of_detail:      "BaseCurrency".to_string(),
                    model:                None,
                    order_id:             Some(299500993),
                    report_date:          NaiveDate::from_str("2022-02-14").unwrap(),
                    settle_date:          Some(NaiveDate::from_str("2022-02-16").unwrap()),
                    trade_code:           Some("P".to_string()),
                    trade_commission:     Some(dec!(-0.88444),),
                    trade_gross:          Some(dec!(-1557.49884),),
                    trade_id:             Some("336654462".to_string(),),
                    trade_price:          Some(dec!(17.61),),
                    trade_quantity:       Some(dec!(100),),
                    trade_tax:            Some(dec!(0),),
                    transaction_id:       Some(929316089,),
                },
            ],
            response.items,
        );
    }
}
