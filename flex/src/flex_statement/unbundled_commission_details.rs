use chrono::NaiveDateTime;
use iso_currency::Currency;
use rust_decimal::Decimal;
use serde::Deserialize;

use crate::{enums::*,
            flex_statement::contract::Contract,
            utils::de::{deserialize_empty_string_is_none,
                        deserialize_from_str,
                        naive_date_time_from_str}};

#[derive(Debug, Deserialize, Clone)]
pub struct UnbundledCommissionDetails {
    #[serde(rename = "$value")]
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub details: Vec<UnbundledCommissionDetail>,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct UnbundledCommissionDetail {
    #[serde(rename = "@accountId")]
    pub account_id: String,

    #[serde(deserialize_with = "deserialize_empty_string_is_none")]
    #[serde(rename = "@acctAlias")]
    pub acct_alias: Option<String>,

    #[serde(rename = "@quantity")]
    pub quantity: Option<Decimal>,

    #[serde(flatten)]
    pub contract: Contract,

    #[serde(deserialize_with = "deserialize_from_str")]
    #[serde(rename = "@currency")]
    pub currency: Currency,

    #[serde(rename = "@buySell")]
    pub buy_sell: BuySell,

    #[serde(rename = "@exchange")]
    pub exchange: String,

    #[serde(deserialize_with = "naive_date_time_from_str")]
    #[serde(rename = "@dateTime")]
    pub date_time: NaiveDateTime,

    #[serde(rename = "@tradeId")]
    pub trade_id: Option<i64>,

    #[serde(deserialize_with = "deserialize_empty_string_is_none")]
    #[serde(rename = "@model")]
    pub model: Option<String>,

    #[serde(rename = "@fxRateToBase")]
    pub fx_rate_to_base: Decimal,

    #[serde(rename = "@other")]
    pub other: Decimal,

    #[serde(rename = "@regOther")]
    pub reg_other: Decimal,

    #[serde(rename = "@regSection31TransactionFee")]
    pub reg_section31_transaction_fee: Decimal,

    #[serde(rename = "@regFINRATradingActivityFee")]
    pub reg_finratrading_activity_fee: Decimal,

    #[serde(rename = "@thirdPartyRegulatoryCharge")]
    pub third_party_regulatory_charge: Decimal,

    #[serde(rename = "@thirdPartyClearingCharge")]
    pub third_party_clearing_charge: Decimal,

    #[serde(rename = "@thirdPartyExecutionCharge")]
    pub third_party_execution_charge: Decimal,

    #[serde(rename = "@brokerClearingCharge")]
    pub broker_clearing_charge: Decimal,

    #[serde(rename = "@brokerExecutionCharge")]
    pub broker_execution_charge: Decimal,

    #[serde(rename = "@totalCommission")]
    pub total_commission: Decimal,

    #[serde(rename = "@price")]
    pub price: Decimal,
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use iso_currency::Currency;
    use pretty_assertions::assert_eq;
    use quick_xml::de::from_str;
    use rust_decimal_macros::dec;

    use super::*;
    use crate::{enums::{AssetCategory, SecIdType},
                flex_statement::contract::Contract};

    #[test]
    fn flex_deserialize_unbundled_commission_detail() {
        let xml = r#"
<UnbundledCommissionDetail accountId="U7502027" acctAlias="" model="" currency="USD" fxRateToBase="0.88444" assetCategory="STK" symbol="F" description="FORD MOTOR CO" conid="9599491" securityID="US3453708600" securityIDType="ISIN" cusip="345370860" isin="US3453708600" listingExchange="NYSE" underlyingConid="" underlyingSymbol="" underlyingSecurityID="" underlyingListingExchange="" issuer="" multiplier="1" strike="" expiry="" putCall="" principalAdjustFactor="" dateTime="2022-02-1409:36:30" exchange="IEX" buySell="BUY" quantity="100" price="17.605" tradeID="336654608" orderReference="ChartTrader428018967" totalCommission="-0.5" brokerExecutionCharge="-0.5" brokerClearingCharge="0" thirdPartyExecutionCharge="0" thirdPartyClearingCharge="0" thirdPartyRegulatoryCharge="0" regFINRATradingActivityFee="0" regSection31TransactionFee="0" regOther="0" other="0" serialNumber="" deliveryType="" commodityType="" fineness="0.0" weight="0.0 ()"/>
"#;
        let response: UnbundledCommissionDetail = from_str(xml).unwrap();
        assert_eq!(
            UnbundledCommissionDetail {
                account_id:                    "U7502027".to_string(),
                acct_alias:                    None,
                quantity:                      Some(dec!(100)),
                contract:                      Contract {
                    asset_category:              AssetCategory::STK,
                    symbol:                      "F".to_string(),
                    description:                 "FORD MOTOR CO".to_string(),
                    con_id:                      9599491,
                    security_id:                 Some("US3453708600".to_string()),
                    security_id_type:            Some(SecIdType::Isin,),
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
                },
                currency:                      Currency::USD,
                buy_sell:                      BuySell::Buy,
                exchange:                      "IEX".to_string(),
                date_time:                     NaiveDateTime::from_str("2022-02-14T09:36:30")
                    .unwrap(),
                trade_id:                      None,
                model:                         None,
                fx_rate_to_base:               dec!(0.88444),
                other:                         dec!(0),
                reg_other:                     dec!(0),
                reg_section31_transaction_fee: dec!(0),
                reg_finratrading_activity_fee: dec!(0),
                third_party_regulatory_charge: dec!(0),
                third_party_clearing_charge:   dec!(0),
                third_party_execution_charge:  dec!(0),
                broker_clearing_charge:        dec!(0),
                broker_execution_charge:       dec!(-0.5),
                total_commission:              dec!(-0.5),
                price:                         dec!(17.605),
            },
            response
        );
    }
}
