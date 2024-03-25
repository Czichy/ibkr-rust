use chrono::NaiveDate;
use iso_currency::Currency;
use rust_decimal::Decimal;
use serde::Deserialize;

use crate::{enums::{AssetCategory, SecIdType},
            utils::de::*};

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Contract {
    #[serde(rename = "@assetCategory")]
    pub asset_category: AssetCategory,

    #[serde(rename = "@symbol")]
    pub symbol: String,

    // #[serde(deserialize_with = "deserialize_from_str")]
    // #[serde(rename = "@currency")]
    // pub currency: Currency,
    #[serde(rename = "@description")]
    pub description: String,

    #[serde(rename = "@conid")]
    #[serde(deserialize_with = "deserialize_from_str")]
    pub con_id: i32,

    #[serde(rename = "@securityID")]
    #[serde(deserialize_with = "deserialize_option_from_str")]
    pub security_id: Option<String>,

    #[serde(rename = "@securityIDType")]
    #[serde(deserialize_with = "deserialize_option_from_str")]
    pub security_id_type: Option<SecIdType>,

    #[serde(deserialize_with = "deserialize_empty_string_is_none")]
    #[serde(rename = "@cusip")]
    pub cusip: Option<String>,

    #[serde(deserialize_with = "deserialize_empty_string_is_none")]
    #[serde(rename = "@isin")]
    pub isin: Option<String>,

    #[serde(rename = "@listingExchange")]
    pub listing_exchange: String,

    #[serde(rename = "@underlyingConid", default)]
    #[serde(deserialize_with = "deserialize_option")]
    pub underlying_con_id: Option<i32>,

    #[serde(deserialize_with = "deserialize_empty_string_is_none")]
    #[serde(rename = "@underlyingSymbol")]
    pub underlying_symbol: Option<String>,

    #[serde(rename = "@underlyingSecurityID")]
    #[serde(deserialize_with = "deserialize_empty_string_is_none")]
    pub underlying_security_id: Option<String>,

    #[serde(deserialize_with = "deserialize_empty_string_is_none")]
    #[serde(rename = "@underlyingListingExchange")]
    pub underlying_listing_exchange: Option<String>,

    #[serde(deserialize_with = "deserialize_empty_string_is_none")]
    #[serde(rename = "@issuer")]
    pub issuer: Option<String>,

    #[serde(deserialize_with = "deserialize_option")]
    #[serde(rename = "@multiplier")]
    pub multiplier: Option<Decimal>,

    #[serde(deserialize_with = "deserialize_option")]
    #[serde(rename = "@strike")]
    pub strike: Option<Decimal>,

    #[serde(deserialize_with = "some_naive_date_from_str")]
    #[serde(rename = "@expiry")]
    pub expiry: Option<NaiveDate>,

    #[serde(deserialize_with = "deserialize_empty_string_is_none")]
    #[serde(rename = "@putCall")]
    pub put_call: Option<String>,

    #[serde(deserialize_with = "deserialize_empty_string_is_none")]
    #[serde(rename = "@principalAdjustFactor")]
    pub principal_adjust_factor: Option<String>,
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use quick_xml::de::from_str;

    use super::*;
    use crate::{enums::{AssetCategory, SecIdType},
                flex_statement::contract::Contract};

    #[test]
    fn flex_deserialize_contract() {
        let xml = r#"
<Trade accountId="U7502027" acctAlias="" model="" currency="EUR" fxRateToBase="1" 
        assetCategory="STK" 
        symbol="VWRL" 
        description="VANG FTSE AW USDD" conid="128831206" securityID="IE00B3RBWM25" securityIDType="ISIN"
        cusip="" isin="IE00B3RBWM25" listingExchange="AEB" underlyingConid="" underlyingSymbol=""
        underlyingSecurityID="" underlyingListingExchange="" issuer="" multiplier="1" strike="" expiry=""
   tradeID="29464420" putCall="" reportDate="2021-02-26" principalAdjustFactor="" dateTime="2021-02-26;10:01:35" tradeDate="2021-02-26" settleDateTarget="2021-03-02" transactionType="ExchTrade" exchange="AEB" quantity="10" tradePrice="89.5" tradeMoney="895" proceeds="-895" taxes="0" ibCommission="-4" ibCommissionCurrency="EUR" netCash="-899" closePrice="89.9" openCloseIndicator="O" notes="" cost="899" fifoPnlRealized="0" fxPnl="0" mtmPnl="4" origTradePrice="0" origTradeDate="" origTradeID="" origOrderID="0" clearingFirmID="" transactionID="85594826" buySell="BUY" ibOrderID="23947654" ibExecID="0000e0c2.60389192.01.01" brokerageOrderID="0004f96a.00014c43.603893de.0001" orderReference="" volatilityOrderLink="" exchOrderId="N/A" extExecID="669953IE00B3RBWM25/B" orderTime="2021-02-26;09:22:05" openDateTime="" holdingPeriodDateTime="" whenRealized="" whenReopened="" levelOfDetail="EXECUTION" changeInPrice="0" changeInQuantity="0" orderType="LMT" traderID="" isAPIOrder="N" accruedInt="0" serialNumber="" deliveryType="" commodityType="" fineness="0.0" weight="0.0 ()"/>
"#;
        let response: Contract = from_str(xml).unwrap();
        tracing::warn!("Contract:{:?}", response);
        assert_eq!(response.con_id, 128831206);
        assert_eq!(response.symbol, "VWRL");
        assert_eq!(response.asset_category, AssetCategory::STK);
        assert_eq!(response.strike, None);
        assert_eq!(response.security_id_type, Some(SecIdType::Isin));
        assert_eq!(response.listing_exchange, "AEB");
        // assert_eq!(response.currency, Currency::EUR);
        assert_eq!(response.security_id, Some("IE00B3RBWM25".into()));
        assert_eq!(response.principal_adjust_factor, None);
    }
}
