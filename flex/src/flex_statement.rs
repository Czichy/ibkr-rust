use chrono::{NaiveDate, NaiveDateTime};
use iso_currency::Currency;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::{contract::Contract,
            flex_statement::{account_information::AccountInformation, trades::Trades},
            stmt_funds::StmtFunds,
            unbundled_commission_details::UnbundledCommissionDetails,
            utils::de::{naive_date_from_str, naive_date_time_from_str}};
pub mod account_information;
pub mod contract;
pub mod stmt_funds;
pub mod trades;
pub mod unbundled_commission_details;

#[derive(Debug, Deserialize)]
pub(crate) enum FlexStatementStatus {
    #[serde(rename = "Success")]
    Success,
    #[serde(rename = "Warn")]
    Warn,
    #[serde(rename = "Fail")]
    Fail,
}
#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
// #[serde(rename = "FlexStatementResponse")]
pub(crate) struct FlexStatementStatusResponse {
    pub status: String,
    // pub error_code: u16,
    // pub error_message: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct FlexStatementRequestResponse {
    pub reference_code: String,
    pub url:            String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct FlexQueryResponse {
    #[allow(dead_code)]
    #[serde(rename = "errors", default)]
    pub errors: Vec<ErrorMessage>,

    pub flex_statements: FlexStatements,

    #[serde(rename = "queryName")]
    pub query_name: String,

    #[allow(dead_code)]
    #[serde(rename = "type", default)]
    pub query_type: String,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct ErrorMessage {
    pub object: String,

    pub message: String,
}

#[derive(Debug, Deserialize)]
pub struct FlexStatements {
    #[serde(rename = "FlexStatement", default)]
    pub statements: Vec<FlexStatement>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct FlexStatement {
    #[serde(rename = "StmtFunds", default)]
    pub statement_of_funds:           Option<StmtFunds>,
    pub cash_transactions:            Option<CashTransactions>,
    pub fx_positions:                 Option<FxPositions>,
    pub open_positions:               Option<OpenPositions>,
    pub trades:                       Option<Trades>,
    pub transaction_taxes:            Option<TransactionTaxes>,
    pub account_information:          Option<AccountInformation>,
    //* pub CFDCharges:CFDCharges, */
    //* *
    // * * pub ChangeInDividendAccruals:ChangeInDividendAccruals, */
    // * *
    // * * pub ComplexPositions:ComplexPositions, */
    // * *
    // * * pub ConversionRates:ConversionRates, */
    // * *
    // * * pub CorporateActions:String, */
    // * *
    // * * pub EquitySummaryInBase:EquitySummaryInBase, */
    // * *
    // * * pub InterestAccruals:InterestAccruals, */
    // * *
    // * * pub OpenDividendAccruals:OpenDividendAccruals, */
    // * *
    // * * pub OpenPositions:OpenPositions, */
    // * *
    // * * pub OptionEAEs:OptionEAEs, */
    // * *
    // * * pub PriorPeriodPositions:PriorPeriodPositions, */
    // * *
    // * * pub SecuritiesInfo:SecuritiesInfo, */
    // * *
    // * * pub SLBActivities:SLBActivities, */
    // * *
    // pub slb_fees: Option<SLBFees>,
    // * *
    // pub statement_of_funds:           Option<StmtFunds>,
    // * *
    // * * pub TierInterestDetails:TierInterestDetails, */
    // * *
    // * * pub TradeConfirms:TradeConfirms, */
    // * *
    // * * pub Transfers:Transfers, */
    // * *
    pub unbundled_commission_details: Option<UnbundledCommissionDetails>,

    #[serde(rename = "accountId")]
    pub account_id: String,

    #[serde(rename = "fromDate")]
    #[serde(deserialize_with = "naive_date_from_str")]
    pub from_date: NaiveDate,

    #[serde(rename = "toDate")]
    #[serde(deserialize_with = "naive_date_from_str")]
    pub to_date: NaiveDate,

    #[serde(rename = "period")]
    pub period: String,

    #[serde(rename = "whenGenerated")]
    #[serde(deserialize_with = "naive_date_time_from_str")]
    pub when_generated: NaiveDateTime,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CashTransactions {
    #[serde(rename = "CashTransaction", default)]
    pub transactions: Vec<CashTransaction>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct OpenPositions {
    #[serde(rename = "OpenPosition", default)]
    pub positions: Vec<OpenPosition>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct FxPositions {
    #[serde(rename = "FxPosition", default)]
    pub positions: Vec<FxPosition>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CashTransaction {
    pub currency:       String,
    pub description:    String,
    #[serde(rename = "transactionID")]
    pub transaction_id: String,
    pub date_time:      String,
    pub amount:         Decimal,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OpenPosition {
    pub currency:       String,
    pub symbol:         String,
    pub description:    String,
    pub position:       Decimal,
    pub mark_price:     Decimal,
    pub position_value: Decimal,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FxPosition {
    /// The default account currency
    pub functional_currency: String,
    /// The currency of this forex position
    pub fx_currency:         String,
    /// The amount of forex currency
    pub quantity:            Decimal,
    /// The value of the forex currency in the default account currency
    pub value:               Decimal,
}

#[derive(Debug, Deserialize, Clone)]
pub struct TransactionTaxes {
    #[serde(rename = "TransactionTaxes", default)]
    pub transaction_taxes: Vec<TransactionTax>,
}
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TransactionTax {
    #[serde(flatten)]
    pub contract:   Contract,
    pub account_id: String,

    pub acct_alias: String,

    pub model: String,

    pub currency: Option<Currency>,

    pub fx_rate_to_base: Option<Decimal>,

    pub principal_adjust_factor: String,

    pub date: Option<NaiveDateTime>,

    pub tax_description: String,

    pub quantity: Option<Decimal>,

    // Note: The reportDate XML attribute may contain either a date or aString, i.e.
    // reportDate="MULTI"
    pub report_date: String,

    pub tax_amount: Option<Decimal>,

    pub trade_id: Option<i64>,

    pub trade_price: Option<Decimal>,

    pub source: String,

    pub code: String,

    pub level_of_detail: String,
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDateTime;
    use pretty_assertions::assert_eq;
    use quick_xml::de::from_str;

    use super::*;

    #[test]
    fn deserialize_date_time() {
        let date_fmt = &vec![
            "%Y-%m-%d", "%Y%m%d", "%m/%d/%Y", "%m/%d/%y", "%d/%m/%Y", "%d/%m/%y", "%d-%m-%y",
        ];
        let delim_fmt = &vec![",", ";", " ", ""];
        let time_fmt = &vec!["%H:%M:%S", "%H%M%S"];
        let all = date_fmt
            .iter()
            .flat_map(move |&a| {
                delim_fmt
                    .iter()
                    .flat_map(move |&b| time_fmt.iter().map(move |&c| format!("{}{}{}", a, b, c)))
            })
            .collect::<Vec<_>>();
        let date_str = vec![
            "2021-08-03;11:53:15".to_string(),
            "20210803115315".to_string(),
        ];
        for date in date_str.iter() {
            tracing::info!("{:?}", date);
            for fmt in all.iter() {
                let parsed_date = NaiveDateTime::parse_from_str(date, fmt);
                if let Ok(parsed_date) = parsed_date {
                    tracing::info!("parsed {:?}\tinto\t{:?}", date, parsed_date);
                }
            }
        }
    }
    #[test]
    fn flex_deserialize_status_response() {
        let xml = r#"
<FlexStatementResponse timestamp='01 October, 2021 10:38 AM EDT'>
<Status>Warn</Status>
<ErrorCode>1019</ErrorCode>
<ErrorMessage>Statement generation in progress. Please try again shortly.</ErrorMessage>
</FlexStatementResponse>
"#;
        let response: FlexStatementStatusResponse = from_str(xml).unwrap();
        tracing::info!("status:{:?}", response);
    }
    #[test]
    fn flex_deserialize_statement() {
        let xml = r#"
<FlexStatement accountId="U7502027" fromDate="2021-01-04" toDate="2021-09-27" period="YearToDate" whenGenerated="2021-09-28;04:59:31">
        <StmtFunds>
<StatementOfFundsLine accountId="U11213636" acctAlias="" model="" currency="EUR" fxRateToBase="1" assetCategory="CASH" symbol="" description="" conid="" securityID="" securityIDType="" cusip="" isin="" listingExchange="" underlyingConid="" underlyingSymbol="" underlyingSecurityID="" underlyingListingExchange="" issuer="" multiplier="0" strike="" expiry="" putCall="" principalAdjustFactor="" reportDate="2023-01-30" date="2023-01-30" settleDate="2023-01-31" activityCode="DEP" activityDescription="Cash Transfer" tradeID="" orderID="" buySell="" tradeQuantity="0" tradePrice="0" tradeGross="0" tradeCommission="0" tradeTax="0" debit="" credit="15724.15" amount="15724.15" tradeCode="" balance="15724.15" levelOfDetail="Currency" transactionID="1635131224"/>
<StatementOfFundsLine accountId="U11213636" acctAlias="" model="" currency="EUR" fxRateToBase="1" assetCategory="" symbol="" description="" conid="" securityID="" securityIDType="" cusip="" isin="" listingExchange="" underlyingConid="" underlyingSymbol="" underlyingSecurityID="" underlyingListingExchange="" issuer="" multiplier="0" strike="" expiry="" putCall="" principalAdjustFactor="" reportDate="2023-02-03" date="2023-02-03" settleDate="2023-02-03" activityCode="CINT" activityDescription="EUR Credit Interest for Jan-2023" tradeID="" orderID="" buySell="" tradeQuantity="0" tradePrice="0" tradeGross="0" tradeCommission="0" tradeTax="0" debit="" credit="0.06" amount="0.06" tradeCode="" balance="15724.21" levelOfDetail="Currency" transactionID="1648819218"/>
        </StmtFunds>
<Trades>
<Order accountId="U7502027" acctAlias="" model="" currency="EUR" fxRateToBase="1" assetCategory="STK" symbol="VWRL" description="VANG FTSE AW USDD" conid="128831206" securityID="IE00B3RBWM25" securityIDType="ISIN" cusip="" isin="IE00B3RBWM25" listingExchange="AEB" underlyingConid="" underlyingSymbol="" underlyingSecurityID="" underlyingListingExchange="" issuer="" multiplier="1" strike="" expiry="" tradeID="" putCall="" reportDate="2021-02-26" principalAdjustFactor="" dateTime="2021-02-26;10:01:35" tradeDate="2021-02-26" settleDateTarget="2021-03-02" transactionType="" exchange="" quantity="10" tradePrice="89.5" tradeMoney="895" proceeds="-895" taxes="0" ibCommission="-4" ibCommissionCurrency="EUR" netCash="-899" closePrice="89.9" openCloseIndicator="O" notes="O" cost="899" fifoPnlRealized="0" fxPnl="0" mtmPnl="4" origTradePrice="" origTradeDate="" origTradeID="" origOrderID="" clearingFirmID="" transactionID="" buySell="BUY" ibOrderID="23947654" ibExecID="" brokerageOrderID="" orderReference="" volatilityOrderLink="" exchOrderId="" extExecID="" orderTime="2021-02-26;09:22:05" openDateTime="" holdingPeriodDateTime="" whenRealized="" whenReopened="" levelOfDetail="ORDER" changeInPrice="" changeInQuantity="" orderType="LMT" traderID="" isAPIOrder="" accruedInt="0" serialNumber="" deliveryType="" commodityType="" fineness="0.0" weight="0.0 ()"/>
<Trade accountId="U7502027" acctAlias="" model="" currency="EUR" fxRateToBase="1" assetCategory="STK" symbol="VWRL" description="VANG FTSE AW USDD" conid="128831206" securityID="IE00B3RBWM25" securityIDType="ISIN" cusip="" isin="IE00B3RBWM25" listingExchange="AEB" underlyingConid="" underlyingSymbol="" underlyingSecurityID="" underlyingListingExchange="" issuer="" multiplier="1" strike="" expiry="" tradeID="29464420" putCall="" reportDate="2021-02-26" principalAdjustFactor="" dateTime="2021-02-26;10:01:35" tradeDate="2021-02-26" settleDateTarget="2021-03-02" transactionType="ExchTrade" exchange="AEB" quantity="10" tradePrice="89.5" tradeMoney="895" proceeds="-895" taxes="0" ibCommission="-4" ibCommissionCurrency="EUR" netCash="-899" closePrice="89.9" openCloseIndicator="O" notes="" cost="899" fifoPnlRealized="0" fxPnl="0" mtmPnl="4" origTradePrice="0" origTradeDate="" origTradeID="" origOrderID="0" clearingFirmID="" transactionID="85594826" buySell="BUY" ibOrderID="23947654" ibExecID="0000e0c2.60389192.01.01" brokerageOrderID="0004f96a.00014c43.603893de.0001" orderReference="" volatilityOrderLink="" exchOrderId="N/A" extExecID="669953IE00B3RBWM25/B" orderTime="2021-02-26;09:22:05" openDateTime="" holdingPeriodDateTime="" whenRealized="" whenReopened="" levelOfDetail="EXECUTION" changeInPrice="0" changeInQuantity="0" orderType="LMT" traderID="" isAPIOrder="N" accruedInt="0" serialNumber="" deliveryType="" commodityType="" fineness="0.0" weight="0.0 ()"/>
<Order accountId="U7502027" acctAlias="" model="" currency="EUR" fxRateToBase="1" assetCategory="STK" symbol="VWRL" description="VANG FTSE AW USDD" conid="128831206" securityID="IE00B3RBWM25" securityIDType="ISIN" cusip="" isin="IE00B3RBWM25" listingExchange="AEB" underlyingConid="" underlyingSymbol="" underlyingSecurityID="" underlyingListingExchange="" issuer="" multiplier="1" strike="" expiry="" tradeID="" putCall="" reportDate="2021-03-04" principalAdjustFactor="" dateTime="2021-03-04;04:45:02" tradeDate="2021-03-04" settleDateTarget="2021-03-08" transactionType="" exchange="" quantity="15" tradePrice="90" tradeMoney="1350" proceeds="-1350" taxes="0" ibCommission="-4" ibCommissionCurrency="EUR" netCash="-1354" closePrice="90.55" openCloseIndicator="O" notes="O" cost="1354" fifoPnlRealized="0" fxPnl="0" mtmPnl="8.25" origTradePrice="" origTradeDate="" origTradeID="" origOrderID="" clearingFirmID="" transactionID="" buySell="BUY" ibOrderID="27685565" ibExecID="" brokerageOrderID="" orderReference="" volatilityOrderLink="" exchOrderId="" extExecID="" orderTime="2021-03-03;10:40:09" openDateTime="" holdingPeriodDateTime="" whenRealized="" whenReopened="" levelOfDetail="ORDER" changeInPrice="" changeInQuantity="" orderType="LMT" traderID="" isAPIOrder="" accruedInt="0" serialNumber="" deliveryType="" commodityType="" fineness="0.0" weight="0.0 ()"/>
<Trade accountId="U7502027" acctAlias="" model="" currency="EUR" fxRateToBase="1" assetCategory="STK" symbol="VWRL" description="VANG FTSE AW USDD" conid="128831206" securityID="IE00B3RBWM25" securityIDType="ISIN" cusip="" isin="IE00B3RBWM25" listingExchange="AEB" underlyingConid="" underlyingSymbol="" underlyingSecurityID="" underlyingListingExchange="" issuer="" multiplier="1" strike="" expiry="" tradeID="33780582" putCall="" reportDate="2021-03-04" principalAdjustFactor="" dateTime="2021-03-04;04:45:02" tradeDate="2021-03-04" settleDateTarget="2021-03-08" transactionType="ExchTrade" exchange="IBIS2" quantity="15" tradePrice="90" tradeMoney="1350" proceeds="-1350" taxes="0" ibCommission="-4" ibCommissionCurrency="EUR" netCash="-1354" closePrice="90.55" openCloseIndicator="O" notes="" cost="1354" fifoPnlRealized="0" fxPnl="0" mtmPnl="8.25" origTradePrice="0" origTradeDate="" origTradeID="" origOrderID="0" clearingFirmID="" transactionID="98279929" buySell="BUY" ibOrderID="27685565" ibExecID="0000dcf9.6040806c.01.01" brokerageOrderID="0004f96a.00014c43.603f2c96.0001" orderReference="" volatilityOrderLink="" exchOrderId="N/A" extExecID="2749247:6904:20210304" orderTime="2021-03-03;10:40:09" openDateTime="" holdingPeriodDateTime="" whenRealized="" whenReopened="" levelOfDetail="EXECUTION" changeInPrice="0" changeInQuantity="0" orderType="LMT" traderID="" isAPIOrder="N" accruedInt="0" serialNumber="" deliveryType="" commodityType="" fineness="0.0" weight="0.0 ()"/>
<Order accountId="U7502027" acctAlias="" model="" currency="EUR" fxRateToBase="1" assetCategory="STK" symbol="VWRL" description="VANG FTSE AW USDD" conid="128831206" securityID="IE00B3RBWM25" securityIDType="ISIN" cusip="" isin="IE00B3RBWM25" listingExchange="AEB" underlyingConid="" underlyingSymbol="" underlyingSecurityID="" underlyingListingExchange="" issuer="" multiplier="1" strike="" expiry="" tradeID="" putCall="" reportDate="2021-05-04" principalAdjustFactor="" dateTime="2021-05-04;09:46:08" tradeDate="2021-05-04" settleDateTarget="2021-05-06" transactionType="" exchange="" quantity="20" tradePrice="95.71" tradeMoney="1914.2" proceeds="-1914.2" taxes="0" ibCommission="-1.25" ibCommissionCurrency="EUR" netCash="-1915.45" closePrice="95.3" openCloseIndicator="O" notes="O" cost="1915.45" fifoPnlRealized="0" fxPnl="0" mtmPnl="-8.2" origTradePrice="" origTradeDate="" origTradeID="" origOrderID="" clearingFirmID="" transactionID="" buySell="BUY" ibOrderID="67571073" ibExecID="" brokerageOrderID="" orderReference="" volatilityOrderLink="" exchOrderId="" extExecID="" orderTime="2021-04-30;10:55:15" openDateTime="" holdingPeriodDateTime="" whenRealized="" whenReopened="" levelOfDetail="ORDER" changeInPrice="" changeInQuantity="" orderType="LMT" traderID="" isAPIOrder="" accruedInt="0" serialNumber="" deliveryType="" commodityType="" fineness="0.0" weight="0.0 ()"/>
<Trade accountId="U7502027" acctAlias="" model="" currency="EUR" fxRateToBase="1" assetCategory="STK" symbol="VWRL" description="VANG FTSE AW USDD" conid="128831206" securityID="IE00B3RBWM25" securityIDType="ISIN" cusip="" isin="IE00B3RBWM25" listingExchange="AEB" underlyingConid="" underlyingSymbol="" underlyingSecurityID="" underlyingListingExchange="" issuer="" multiplier="1" strike="" expiry="" tradeID="78760796" putCall="" reportDate="2021-05-04" principalAdjustFactor="" dateTime="2021-05-04;09:46:08" tradeDate="2021-05-04" settleDateTarget="2021-05-06" transactionType="ExchTrade" exchange="GETTEX2" quantity="20" tradePrice="95.71" tradeMoney="1914.2" proceeds="-1914.2" taxes="0" ibCommission="-1.25" ibCommissionCurrency="EUR" netCash="-1915.45" closePrice="95.3" openCloseIndicator="O" notes="" cost="1915.45" fifoPnlRealized="0" fxPnl="0" mtmPnl="-8.2" origTradePrice="0" origTradeDate="" origTradeID="" origOrderID="0" clearingFirmID="" transactionID="225760527" buySell="BUY" ibOrderID="67571073" ibExecID="0000d349.609100e5.01.01" brokerageOrderID="0004f96a.00014c43.608253ec.0006" orderReference="" volatilityOrderLink="" exchOrderId="N/A" extExecID="43642036" orderTime="2021-04-30;10:55:15" openDateTime="" holdingPeriodDateTime="" whenRealized="" whenReopened="" levelOfDetail="EXECUTION" changeInPrice="0" changeInQuantity="0" orderType="LMT" traderID="" isAPIOrder="N" accruedInt="0" serialNumber="" deliveryType="" commodityType="" fineness="0.0" weight="0.0 ()"/>
</Trades>
<TransactionTaxes>
</TransactionTaxes>
        <OpenPositions>
<OpenPosition accountId="U11213636" acctAlias="" model="" currency="EUR" fxRateToBase="1" assetCategory="OPT" symbol="P ODXS 20231215 14800 M" description="DAX 15DEC23 14800 P" conid="611912519" securityID="" securityIDType="" cusip="" isin="" listingExchange="EUREX" underlyingConid="825711" underlyingSymbol="DAX" underlyingSecurityID="" underlyingListingExchange="" issuer="" multiplier="1" strike="14800" expiry="2023-12-15" putCall="P" principalAdjustFactor="" reportDate="2023-11-09" position="4" markPrice="82.7" positionValue="330.8" openPrice="104.2" costBasisPrice="104.2" costBasisMoney="416.8" percentOfNAV="100.00" fifoPnlUnrealized="-86" side="Long" levelOfDetail="SUMMARY" openDateTime="" holdingPeriodDateTime="" vestingDate="" code="" originatingOrderID="" originatingTransactionID="" accruedInt="" serialNumber="" deliveryType="" commodityType="" fineness="0.0" weight="0.0 ()"/>
<OpenPosition accountId="U11213636" acctAlias="" model="" currency="EUR" fxRateToBase="1" assetCategory="BOND" symbol="DBR 1 08/15/24" description="DBR 1 08/15/24" conid="165809631" securityID="DE0001102366" securityIDType="ISIN" cusip="" isin="DE0001102366" listingExchange="" underlyingConid="" underlyingSymbol="" underlyingSecurityID="" underlyingListingExchange="" issuer="" multiplier="1" strike="" expiry="" putCall="" principalAdjustFactor="1" reportDate="2023-11-09" position="5000" markPrice="98.113" positionValue="4905.65" openPrice="97.75" costBasisPrice="97.75" costBasisMoney="4887.5" percentOfNAV="20.05" fifoPnlUnrealized="18.15" side="Long" levelOfDetail="SUMMARY" openDateTime="" holdingPeriodDateTime="" vestingDate="" code="" originatingOrderID="" originatingTransactionID="" accruedInt="12.3" serialNumber="" deliveryType="" commodityType="" fineness="0.0" weight="0.0 ()"/>
<OpenPosition accountId="U11213636" acctAlias="" model="" currency="EUR" fxRateToBase="1" assetCategory="BOND" symbol="DBR 1 3/4 02/15/24" description="DBR 1 3/4 02/15/24" conid="142870711" securityID="DE0001102333" securityIDType="ISIN" cusip="" isin="DE0001102333" listingExchange="" underlyingConid="" underlyingSymbol="" underlyingSecurityID="" underlyingListingExchange="" issuer="" multiplier="1" strike="" expiry="" putCall="" principalAdjustFactor="1" reportDate="2023-11-09" position="15000" markPrice="99.502" positionValue="14925.3" openPrice="99.133333333" costBasisPrice="99.133333333" costBasisMoney="14870" percentOfNAV="61.01" fifoPnlUnrealized="55.3" side="Long" levelOfDetail="SUMMARY" openDateTime="" holdingPeriodDateTime="" vestingDate="" code="" originatingOrderID="" originatingTransactionID="" accruedInt="194.9" serialNumber="" deliveryType="" commodityType="" fineness="0.0" weight="0.0 ()"/>
<OpenPosition accountId="U11213636" acctAlias="" model="" currency="USD" fxRateToBase="0.93732" assetCategory="BOND" symbol="T 2 1/4 03/31/24" description="T 2 1/4 03/31/24" conid="553749289" securityID="US91282CEG24" securityIDType="ISIN" cusip="91282CEG2" isin="US91282CEG24" listingExchange="" underlyingConid="" underlyingSymbol="" underlyingSecurityID="" underlyingListingExchange="" issuer="" multiplier="1" strike="" expiry="" putCall="" principalAdjustFactor="1" reportDate="2023-11-09" position="5000" markPrice="98.828125" positionValue="4941.41" openPrice="96.9284" costBasisPrice="96.9284" costBasisMoney="4846.42" percentOfNAV="18.93" fifoPnlUnrealized="94.99" side="Long" levelOfDetail="SUMMARY" openDateTime="" holdingPeriodDateTime="" vestingDate="" code="" originatingOrderID="" originatingTransactionID="" accruedInt="12.6" serialNumber="" deliveryType="" commodityType="" fineness="0.0" weight="0.0 ()"/>
</OpenPositions>
</FlexStatement>
"#;
        let response: FlexStatement = from_str(xml).unwrap();
        tracing::error!("{response:#?}");
        if let super::trades::TradeElements::Trade(trade) =
            &response.trades.as_ref().unwrap().items[1]
        {
            assert_eq!(trade.contract.symbol, "VWRL");
        } else {
            panic!("");
        }
    }
}
