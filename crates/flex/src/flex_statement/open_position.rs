use chrono::NaiveDateTime;
use rust_decimal::Decimal;
use serde::Deserialize;

use crate::{enums::LongShort, flex_statement::contract::Contract, utils::de::*};

#[derive(Debug, Deserialize, Clone)]
pub struct OpenPositions {
    #[serde(rename = "OpenPosition", default)]
    pub positions: Vec<OpenPosition>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OpenPosition {
    #[serde(rename = "@accountId")]
    pub account_id: String,

    #[serde(deserialize_with = "deserialize_option_from_str")]
    #[serde(rename = "@acctAlias")]
    pub acct_alias: Option<String>,

    #[serde(deserialize_with = "deserialize_option_from_str")]
    #[serde(rename = "@model")]
    pub model: Option<String>,

    #[serde(rename = "@fxRateToBase")]
    #[serde(deserialize_with = "deserialize_option_from_str")]
    pub fx_rate_to_base: Option<Decimal>,

    #[serde(flatten)]
    pub contract: Option<Contract>,

    #[serde(rename = "@position")]
    #[serde(deserialize_with = "deserialize_option_from_str")]
    pub position: Option<i64>,

    #[serde(rename = "@markPrice")]
    #[serde(deserialize_with = "deserialize_option_from_str")]
    pub mark_price: Option<Decimal>,

    #[serde(rename = "@positionValue")]
    #[serde(deserialize_with = "deserialize_option_from_str")]
    pub position_value: Option<Decimal>,

    #[serde(rename = "@openPrice")]
    #[serde(deserialize_with = "deserialize_option_from_str")]
    pub open_price: Option<Decimal>,

    #[serde(rename = "@costBasisPrice")]
    #[serde(deserialize_with = "deserialize_option_from_str")]
    pub cost_basis_price: Option<Decimal>,

    #[serde(rename = "@costBasisMoney")]
    #[serde(deserialize_with = "deserialize_option_from_str")]
    pub cost_basis_money: Option<Decimal>,

    #[serde(rename = "@percentOfNAV")]
    #[serde(deserialize_with = "deserialize_option_from_str")]
    pub percent_of_nav: Option<Decimal>,

    #[serde(rename = "@fifoPnlUnrealized")]
    #[serde(deserialize_with = "deserialize_option_from_str")]
    pub fifo_pnl_unrealized: Option<Decimal>,

    #[serde(rename = "@side")]
    pub side: LongShort,

    #[serde(rename = "@levelOfDetail")]
    pub level_of_detail: String,

    #[serde(rename = "@openDateTime")]
    #[serde(deserialize_with = "some_naive_date_time_from_str")]
    pub open_date_time: Option<NaiveDateTime>,

    #[serde(rename = "@holdingPeriodDateTime")]
    #[serde(deserialize_with = "some_naive_date_time_from_str")]
    pub holding_period_date_time: Option<NaiveDateTime>,

    #[serde(rename = "@code")]
    #[serde(deserialize_with = "deserialize_option_from_str")]
    pub code: Option<String>,

    #[serde(rename = "@originatingOrderID")]
    #[serde(deserialize_with = "deserialize_option_from_str")]
    pub originating_order_id: Option<i64>,

    #[serde(rename = "@originatingTransactionID")]
    #[serde(deserialize_with = "deserialize_option_from_str")]
    pub originating_transaction_id: Option<i64>,

    #[serde(rename = "@accruedInt")]
    #[serde(deserialize_with = "deserialize_option_from_str")]
    pub accrued_int: Option<Decimal>,
}
#[cfg(test)]
mod tests {
    use quick_xml::de::from_str;

    use super::*;

    #[test]
    fn flex_deserialize_open_position() {
        let xml = r#"
        <OpenPosition accountId="U11213636" acctAlias="" model="" currency="EUR" fxRateToBase="1" assetCategory="BOND" symbol="DBR 1 08/15/24" description="DBR 1 08/15/24" conid="165809631" securityID="DE0001102366" securityIDType="ISIN" cusip="" isin="DE0001102366" listingExchange="" underlyingConid="" underlyingSymbol="" underlyingSecurityID="" underlyingListingExchange="" issuer="" multiplier="1" strike="" expiry="" putCall="" principalAdjustFactor="1" reportDate="2023-12-29" position="2000" markPrice="98.562" positionValue="1971.24" openPrice="97.75" costBasisPrice="97.75" costBasisMoney="1955" percentOfNAV="7.60" fifoPnlUnrealized="16.24" side="Long" levelOfDetail="SUMMARY" openDateTime="" holdingPeriodDateTime="" vestingDate="" code="" originatingOrderID="" originatingTransactionID="" accruedInt="7.7" serialNumber="" deliveryType="" commodityType="" fineness="0.0" weight="0.0"/>
<OpenPosition accountId="U11213636" acctAlias="" model="" currency="EUR" fxRateToBase="1" assetCategory="BOND" symbol="DBR 1 3/4 02/15/24" description="DBR 1 3/4 02/15/24" conid="142870711" securityID="DE0001102333" securityIDType="ISIN" cusip="" isin="DE0001102333" listingExchange="" underlyingConid="" underlyingSymbol="" underlyingSecurityID="" underlyingListingExchange="" issuer="" multiplier="1" strike="" expiry="" putCall="" principalAdjustFactor="1" reportDate="2023-12-29" position="15000" markPrice="99.777" positionValue="14966.55" openPrice="99.133333333" costBasisPrice="99.133333333" costBasisMoney="14870" percentOfNAV="57.71" fifoPnlUnrealized="96.55" side="Long" levelOfDetail="SUMMARY" openDateTime="" holdingPeriodDateTime="" vestingDate="" code="" originatingOrderID="" originatingTransactionID="" accruedInt="231.58" serialNumber="" deliveryType="" commodityType="" fineness="0.0" weight="0.0"/>
"#;
        let response: OpenPosition = from_str(xml).unwrap();
        println!("{response:#?}");
    }
    // assert_eq!(response.contract.symbol.unwrap(), "VWRL");
}
