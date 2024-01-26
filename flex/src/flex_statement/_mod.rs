use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
mod enums;
//mod trade_confirms;
use enums::*;

#[derive(Debug, Deserialize)]
pub(crate) enum FlexStatementStatus {
    Success,
    Warn,
    Fail,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct FlexStatementStatusResponse {
    pub status: FlexStatementStatus,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct FlexStatementRequestResponse {
    pub reference_code: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct FlexStatementGetResponse {
    pub flex_statements: Option<FlexStatements>,
}

#[derive(Debug, Deserialize)]
pub struct FlexStatements {
    #[serde(rename = "FlexStatement", default)]
    pub items: Vec<FlexStatement>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FlexStatement {
    #[serde(rename = "Trades", default)]
    pub trades: Option<Trades>,
    //pub order: Option<Order>,
    pub cash_transactions: Option<CashTransactions>,
    pub open_positions: Option<OpenPositions>,
    pub fx_positions: Option<FxPositions>,
}
#[derive(Debug, Deserialize)]
pub struct Trades {
    #[serde(rename = "Trade", default)]
    items: Vec<Trade>,
}
#[derive(Default, Debug, Clone,Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Contract {
    pub con_id:                            Option<i32>,
    pub symbol:                            Option<String>,
    pub sec_type:                          Option<SecType>,
    pub last_trade_date_or_contract_month: Option<String>,
#[serde(skip_deserializing)]
    pub strike:                            Option<Decimal>,
    pub right:                             Option<OptionRight>,
    pub multiplier:                        Option<String>,
    pub exchange:                          Option<String>,
    pub currency:                          Option<String>,
    pub local_symbol:                      Option<String>,
    pub primary_exchange:                  Option<String>,
    pub trading_class:                     Option<String>,
    pub include_expired:                   Option<bool>,
    pub sec_id_type:                       Option<SecIdType>,
    pub sec_id:                            Option<String>,
    pub combo_legs_description:            Option<String>,

//#[serde(skip_deserializing)]
//    pub combo_legs:                        Option<Vec<ComboLeg>>,
//#[serde(skip_deserializing)]
//    pub delta_neutral_contract:            Option<DeltaNeutralContract>,
}
#[derive(Debug, PartialEq, Eq, Clone,Deserialize)]
pub enum SecType {
#[serde(rename = "STK")]
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
#[derive(Debug, Clone,Deserialize)]
pub enum SecIdType {
    Isin,
    Cusip,
}
#[derive(Debug, Clone,Deserialize)]
pub enum OptionRight {
    Undefined,
    Put,
    Call,
}
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Trade {
    #[serde(flatten)]
    contract:Contract,
    currency: String,
    symbol: String,
    description: String,
    #[serde(rename = "transactionID")]
    transaction_id: String,
    date_time: String,
    quantity: i32,
    trade_price: Decimal,
    trade_money: Decimal,
    ib_commission: Decimal,
}

#[derive(Debug, Deserialize)]
pub struct CashTransactions {
    #[serde(rename = "CashTransaction", default)]
    pub items: Vec<CashTransaction>,
}

#[derive(Debug, Deserialize)]
pub struct OpenPositions {
    #[serde(rename = "OpenPosition", default)]
    pub items: Vec<OpenPosition>,
}

#[derive(Debug, Deserialize)]
pub struct FxPositions {
    #[serde(rename = "FxPosition", default)]
    pub items: Vec<FxPosition>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CashTransaction {
    pub currency: String,
    pub description: String,
    #[serde(rename = "transactionID")]
    pub transaction_id: String,
    pub date_time: String,
    pub amount: Decimal,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenPosition {
    pub currency: String,
    pub symbol: String,
    pub description: String,
    pub position: Decimal,
    pub mark_price: Decimal,
    pub position_value: Decimal,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FxPosition {
    /// The default account currency
    pub functional_currency: String,
    /// The currency of this forex position
    pub fx_currency: String,
    /// The amount of forex currency
    pub quantity: Decimal,
    /// The value of the forex currency in the default account currency
    pub value: Decimal,
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use quick_xml::de::{from_str, DeError};
    use serde::Deserialize;
    use tracing_test::traced_test;

    use crate::domain::{FlexStatement, Trade};
    #[test]
    #[traced_test]
    fn deserialize_trade() {
        let xml = r#"
<Trade accountId="U7502027" acctAlias="" model="" currency="EUR" fxRateToBase="1" 
        assetCategory="STK" 
        symbol="VWRL" 
        description="VANG FTSE AW USDD" conid="128831206" securityID="IE00B3RBWM25" securityIDType="ISIN"
        cusip="" isin="IE00B3RBWM25" listingExchange="AEB" underlyingConid="" underlyingSymbol=""
        underlyingSecurityID="" underlyingListingExchange="" issuer="" multiplier="1" strike="" expiry=""
   tradeID="29464420" putCall="" reportDate="2021-02-26" principalAdjustFactor="" dateTime="2021-02-26;10:01:35" tradeDate="2021-02-26" settleDateTarget="2021-03-02" transactionType="ExchTrade" exchange="AEB" quantity="10" tradePrice="89.5" tradeMoney="895" proceeds="-895" taxes="0" ibCommission="-4" ibCommissionCurrency="EUR" netCash="-899" closePrice="89.9" openCloseIndicator="O" notes="" cost="899" fifoPnlRealized="0" fxPnl="0" mtmPnl="4" origTradePrice="0" origTradeDate="" origTradeID="" origOrderID="0" clearingFirmID="" transactionID="85594826" buySell="BUY" ibOrderID="23947654" ibExecID="0000e0c2.60389192.01.01" brokerageOrderID="0004f96a.00014c43.603893de.0001" orderReference="" volatilityOrderLink="" exchOrderId="N/A" extExecID="669953IE00B3RBWM25/B" orderTime="2021-02-26;09:22:05" openDateTime="" holdingPeriodDateTime="" whenRealized="" whenReopened="" levelOfDetail="EXECUTION" changeInPrice="0" changeInQuantity="0" orderType="LMT" traderID="" isAPIOrder="N" accruedInt="0" serialNumber="" deliveryType="" commodityType="" fineness="0.0" weight="0.0 ()"/>
"#;
        let response: Trade = from_str(xml).unwrap();
        tracing::info!("Trade:{:?}", response);
        assert_eq!(response.symbol, "VWRL");
    }
    #[test]
    #[traced_test]
    fn deserialize_flex_statement() {
        let xml = r#"
<FlexStatement accountId="U7502027" fromDate="2021-01-04" toDate="2021-09-27" period="YearToDate" whenGenerated="2021-09-28;04:59:31">
<Trades>
<Trade accountId="U7502027" acctAlias="" model="" currency="EUR" fxRateToBase="1" 
            assetCategory="STK" 
            symbol="VWRL" 
            description="VANG FTSE AW USDD" 
            conid="128831206" 
            securityID="IE00B3RBWM25" 
            securityIDType="ISIN" 
            cusip="" 
            isin="IE00B3RBWM25" 
            listingExchange="AEB" 
            underlyingConid="" 
            underlyingSymbol="" 
            underlyingSecurityID="" 
            underlyingListingExchange="" 
            issuer="" 
            multiplier="1" 
            strike="" 
            expiry="" 
   tradeID="29464420" putCall="" reportDate="2021-02-26" principalAdjustFactor="" dateTime="2021-02-26;10:01:35" tradeDate="2021-02-26" settleDateTarget="2021-03-02" transactionType="ExchTrade" exchange="AEB" quantity="10" tradePrice="89.5" tradeMoney="895" proceeds="-895" taxes="0" ibCommission="-4" ibCommissionCurrency="EUR" netCash="-899" closePrice="89.9" openCloseIndicator="O" notes="" cost="899" fifoPnlRealized="0" fxPnl="0" mtmPnl="4" origTradePrice="0" origTradeDate="" origTradeID="" origOrderID="0" clearingFirmID="" transactionID="85594826" buySell="BUY" ibOrderID="23947654" ibExecID="0000e0c2.60389192.01.01" brokerageOrderID="0004f96a.00014c43.603893de.0001" orderReference="" volatilityOrderLink="" exchOrderId="N/A" extExecID="669953IE00B3RBWM25/B" orderTime="2021-02-26;09:22:05" openDateTime="" holdingPeriodDateTime="" whenRealized="" whenReopened="" levelOfDetail="EXECUTION" changeInPrice="0" changeInQuantity="0" orderType="LMT" traderID="" isAPIOrder="N" accruedInt="0" serialNumber="" deliveryType="" commodityType="" fineness="0.0" weight="0.0 ()"/>
</Trades>
</FlexStatement>
"#;
        let response: FlexStatement = from_str(xml).unwrap();
        tracing::info!("Trade:{:?}", response);
        //assert_eq!(response.symbol, "VWRL");
    }
}
