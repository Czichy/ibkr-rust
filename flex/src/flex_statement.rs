use chrono::{NaiveDate, NaiveDateTime};
use iso_currency::Currency;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use self::open_position::OpenPositions;
use crate::{contract::Contract,
            flex_statement::{account_information::AccountInformation, trades::Trades},
            stmt_funds::StmtFunds,
            unbundled_commission_details::UnbundledCommissionDetails,
            utils::de::{naive_date_from_str, naive_date_time_from_str}};

pub mod account_information;
pub mod contract;
pub mod open_position;
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

    #[serde(rename = "@queryName")]
    pub query_name: String,

    #[allow(dead_code)]
    #[serde(rename = "@type", default)]
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

    #[serde(rename = "@accountId")]
    pub account_id: String,

    #[serde(rename = "@fromDate")]
    #[serde(deserialize_with = "naive_date_from_str")]
    pub from_date: NaiveDate,

    #[serde(rename = "@toDate")]
    #[serde(deserialize_with = "naive_date_from_str")]
    pub to_date: NaiveDate,

    #[serde(rename = "@period")]
    pub period: String,

    #[serde(rename = "@whenGenerated")]
    #[serde(deserialize_with = "naive_date_time_from_str")]
    pub when_generated: NaiveDateTime,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CashTransactions {
    #[serde(rename = "CashTransaction", default)]
    pub transactions: Vec<CashTransaction>,
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
    pub contract: Contract,

    #[serde(rename = "@accountId")]
    pub account_id: String,

    #[serde(rename = "@acctAlias")]
    pub acct_alias: String,

    #[serde(rename = "@model")]
    pub model: String,

    #[serde(rename = "@currency")]
    pub currency: Option<Currency>,

    #[serde(rename = "@fxRateToBase")]
    pub fx_rate_to_base: Option<Decimal>,

    #[serde(rename = "@principalAdjustFactor")]
    pub principal_adjust_factor: String,

    #[serde(rename = "@date")]
    pub date: Option<NaiveDateTime>,

    #[serde(rename = "@taxDescription")]
    pub tax_description: String,

    #[serde(rename = "@quantity")]
    pub quantity: Option<Decimal>,

    // Note: The reportDate XML attribute may contain either a date or aString, i.e.
    // reportDate="MULTI"
    #[serde(rename = "@reportDate")]
    pub report_date: String,

    #[serde(rename = "@taxAmount")]
    pub tax_amount: Option<Decimal>,

    #[serde(rename = "@tradeId")]
    pub trade_id: Option<i64>,

    #[serde(rename = "@tradePrice")]
    pub trade_price: Option<Decimal>,

    #[serde(rename = "@source")]
    pub source: String,

    #[serde(rename = "@code")]
    pub code: String,

    #[serde(rename = "@levelOfDetail")]
    pub level_of_detail: String,
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDateTime;
    use pretty_assertions::assert_eq;
    use quick_xml::de::from_str;

    use super::*;
    use crate::flex_statement_from_file;

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
        let date_str = [
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
<StatementOfFundsLine accountId="U11213636" acctAlias="" model="" currency="EUR" fxRateToBase="1" assetCategory="" symbol="" description="" conid="" securityID="" securityIDType="" cusip="" isin="" listingExchange="" underlyingConid="" underlyingSymbol="" underlyingSecurityID="" underlyingListingExchange="" issuer="" multiplier="" strike="" expiry="" putCall="" principalAdjustFactor="" reportDate="2023-06-13" date="2023-06-13" settleDate="2023-06-13" activityCode="FOREX" activityDescription="Commission from Forex Trade" tradeID="557621623" orderID="" buySell="" tradeQuantity="0" tradePrice="0" tradeGross="0" tradeCommission="0" tradeTax="0" debit="-1.8591" credit="" amount="-1.8591" tradeCode="" balance="12685.5953445" levelOfDetail="Currency" transactionID="1931694399"/>
<StatementOfFundsLine accountId="U11213636" acctAlias="" model="" currency="EUR" fxRateToBase="1" assetCategory="" symbol="" description="" conid="" securityID="" securityIDType="" cusip="" isin="" listingExchange="" underlyingConid="" underlyingSymbol="" underlyingSecurityID="" underlyingListingExchange="" issuer="" multiplier="" strike="" expiry="" putCall="" principalAdjustFactor="" reportDate="2023-06-13" date="2023-06-13" settleDate="2023-06-15" activityCode="FOREX" activityDescription="Traded Currency Leg from Forex Trade" tradeID="557621623" orderID="" buySell="" tradeQuantity="0" tradePrice="0" tradeGross="0" tradeCommission="0" tradeTax="0" debit="" credit="7555.25" amount="7555.25" tradeCode="" balance="12687.4544445" levelOfDetail="Currency" transactionID="1931691616"/>
<StatementOfFundsLine accountId="U11213636" acctAlias="" model="" currency="EUR" fxRateToBase="1" assetCategory="" symbol="" description="" conid="" securityID="" securityIDType="" cusip="" isin="" listingExchange="" underlyingConid="" underlyingSymbol="" underlyingSecurityID="" underlyingListingExchange="" issuer="" multiplier="" strike="" expiry="" putCall="" principalAdjustFactor="" reportDate="2023-07-26" date="2023-07-26" settleDate="2023-07-26" activityCode="FOREX" activityDescription="Commission from Forex Trade" tradeID="578824562" orderID="" buySell="" tradeQuantity="0" tradePrice="0" tradeGross="0" tradeCommission="0" tradeTax="0" debit="-1.80904" credit="" amount="-1.80904" tradeCode="" balance="8546.767607978" levelOfDetail="Currency" transactionID="2024794785"/>
<StatementOfFundsLine accountId="U11213636" acctAlias="" model="" currency="EUR" fxRateToBase="1" assetCategory="" symbol="" description="" conid="" securityID="" securityIDType="" cusip="" isin="" listingExchange="" underlyingConid="" underlyingSymbol="" underlyingSecurityID="" underlyingListingExchange="" issuer="" multiplier="" strike="" expiry="" putCall="" principalAdjustFactor="" reportDate="2023-07-26" date="2023-07-26" settleDate="2023-07-26" activityCode="FOREX" activityDescription="Commission from Forex Trade" tradeID="578831405" orderID="" buySell="" tradeQuantity="0" tradePrice="0" tradeGross="0" tradeCommission="0" tradeTax="0" debit="-1.80904" credit="" amount="-1.80904" tradeCode="" balance="4997.958567978" levelOfDetail="Currency" transactionID="2024807866"/>
<StatementOfFundsLine accountId="U11213636" acctAlias="" model="" currency="EUR" fxRateToBase="1" assetCategory="" symbol="" description="" conid="" securityID="" securityIDType="" cusip="" isin="" listingExchange="" underlyingConid="" underlyingSymbol="" underlyingSecurityID="" underlyingListingExchange="" issuer="" multiplier="" strike="" expiry="" putCall="" principalAdjustFactor="" reportDate="2023-07-26" date="2023-07-26" settleDate="2023-07-26" activityCode="FOREX" activityDescription="Commission from Forex Trade" tradeID="578831432" orderID="" buySell="" tradeQuantity="0" tradePrice="0" tradeGross="0" tradeCommission="0" tradeTax="0" debit="" credit="" amount="0" tradeCode="" balance="4997.958567978" levelOfDetail="Currency" transactionID="2024807868"/>
<StatementOfFundsLine accountId="U11213636" acctAlias="" model="" currency="EUR" fxRateToBase="1" assetCategory="" symbol="" description="" conid="" securityID="" securityIDType="" cusip="" isin="" listingExchange="" underlyingConid="" underlyingSymbol="" underlyingSecurityID="" underlyingListingExchange="" issuer="" multiplier="" strike="" expiry="" putCall="" principalAdjustFactor="" reportDate="2023-07-26" date="2023-07-26" settleDate="2023-07-28" activityCode="FOREX" activityDescription="Traded Currency Leg from Forex Trade" tradeID="578824562" orderID="" buySell="" tradeQuantity="0" tradePrice="0" tradeGross="0" tradeCommission="0" tradeTax="0" debit="-3000" credit="" amount="-3000" tradeCode="" balance="8548.576647978" levelOfDetail="Currency" transactionID="2024791862"/>
<StatementOfFundsLine accountId="U11213636" acctAlias="" model="" currency="EUR" fxRateToBase="1" assetCategory="" symbol="" description="" conid="" securityID="" securityIDType="" cusip="" isin="" listingExchange="" underlyingConid="" underlyingSymbol="" underlyingSecurityID="" underlyingListingExchange="" issuer="" multiplier="" strike="" expiry="" putCall="" principalAdjustFactor="" reportDate="2023-07-26" date="2023-07-26" settleDate="2023-07-28" activityCode="FOREX" activityDescription="Traded Currency Leg from Forex Trade" tradeID="578831405" orderID="" buySell="" tradeQuantity="0" tradePrice="0" tradeGross="0" tradeCommission="0" tradeTax="0" debit="-1837" credit="" amount="-1837" tradeCode="" balance="6709.767607978" levelOfDetail="Currency" transactionID="2024806571"/>
<StatementOfFundsLine accountId="U11213636" acctAlias="" model="" currency="EUR" fxRateToBase="1" assetCategory="" symbol="" description="" conid="" securityID="" securityIDType="" cusip="" isin="" listingExchange="" underlyingConid="" underlyingSymbol="" underlyingSecurityID="" underlyingListingExchange="" issuer="" multiplier="" strike="" expiry="" putCall="" principalAdjustFactor="" reportDate="2023-07-26" date="2023-07-26" settleDate="2023-07-28" activityCode="FOREX" activityDescription="Traded Currency Leg from Forex Trade" tradeID="578831432" orderID="" buySell="" tradeQuantity="0" tradePrice="0" tradeGross="0" tradeCommission="0" tradeTax="0" debit="-1710" credit="" amount="-1710" tradeCode="" balance="4999.767607978" levelOfDetail="Currency" transactionID="2024806572"/>
<StatementOfFundsLine accountId="U11213636" acctAlias="" model="" currency="EUR" fxRateToBase="1" assetCategory="" symbol="" description="" conid="" securityID="" securityIDType="" cusip="" isin="" listingExchange="" underlyingConid="" underlyingSymbol="" underlyingSecurityID="" underlyingListingExchange="" issuer="" multiplier="0" strike="" expiry="" putCall="" principalAdjustFactor="" reportDate="2023-02-03" date="2023-02-03" settleDate="2023-02-03" activityCode="CINT" activityDescription="EUR Credit Interest for Jan-2023" tradeID="" orderID="" buySell="" tradeQuantity="0" tradePrice="0" tradeGross="0" tradeCommission="0" tradeTax="0" debit="" credit="0.06" amount="0.06" tradeCode="" balance="15724.21" levelOfDetail="Currency" transactionID="1648819218"/>
<StatementOfFundsLine accountId="U11213636" acctAlias="" model="" currency="EUR" fxRateToBase="1" assetCategory="" symbol="" description="" conid="" securityID="" securityIDType="" cusip="" isin="" listingExchange="" underlyingConid="" underlyingSymbol="" underlyingSecurityID="" underlyingListingExchange="" issuer="" multiplier="0" strike="" expiry="" putCall="" principalAdjustFactor="" reportDate="2023-02-03" date="2023-02-03" settleDate="2023-02-03" activityCode="CINT" activityDescription="EUR Credit Interest for Jan-2023" tradeID="" orderID="" buySell="" tradeQuantity="0" tradePrice="0" tradeGross="0" tradeCommission="0" tradeTax="0" debit="" credit="0.06" amount="0.06" tradeCode="" balance="15724.21" levelOfDetail="Currency" transactionID="1648819218"/>
<StatementOfFundsLine accountId="U11213636" acctAlias="" model="" currency="EUR" fxRateToBase="1" assetCategory="" symbol="" description="" conid="" securityID="" securityIDType="" cusip="" isin="" listingExchange="" underlyingConid="" underlyingSymbol="" underlyingSecurityID="" underlyingListingExchange="" issuer="" multiplier="0" strike="" expiry="" putCall="" principalAdjustFactor="" reportDate="2023-02-06" date="2023-02-03" settleDate="2023-02-03" activityCode="FRTAX" activityDescription="Withholding @ 20% on Credit Interest for Jan-2023" tradeID="" orderID="" buySell="" tradeQuantity="0" tradePrice="0" tradeGross="0" tradeCommission="0" tradeTax="0" debit="-0.01" credit="" amount="-0.01" tradeCode="" balance="15724.2" levelOfDetail="Currency" transactionID="1650918783"/>
<StatementOfFundsLine accountId="U11213636" acctAlias="" model="" currency="EUR" fxRateToBase="1" assetCategory="" symbol="" description="" conid="" securityID="" securityIDType="" cusip="" isin="" listingExchange="" underlyingConid="" underlyingSymbol="" underlyingSecurityID="" underlyingListingExchange="" issuer="" multiplier="0" strike="" expiry="" putCall="" principalAdjustFactor="" reportDate="2023-03-03" date="2023-03-03" settleDate="2023-03-03" activityCode="CINT" activityDescription="EUR Credit Interest for Feb-2023" tradeID="" orderID="" buySell="" tradeQuantity="0" tradePrice="0" tradeGross="0" tradeCommission="0" tradeTax="0" debit="" credit="2.34" amount="2.34" tradeCode="" balance="15726.54" levelOfDetail="Currency" transactionID="1709347105"/>
<StatementOfFundsLine accountId="U11213636" acctAlias="" model="" currency="EUR" fxRateToBase="1" assetCategory="" symbol="" description="" conid="" securityID="" securityIDType="" cusip="" isin="" listingExchange="" underlyingConid="" underlyingSymbol="" underlyingSecurityID="" underlyingListingExchange="" issuer="" multiplier="0" strike="" expiry="" putCall="" principalAdjustFactor="" reportDate="2023-03-03" date="2023-03-03" settleDate="2023-03-03" activityCode="FRTAX" activityDescription="Withholding @ 20% on Credit Interest for Feb-2023" tradeID="" orderID="" buySell="" tradeQuantity="0" tradePrice="0" tradeGross="0" tradeCommission="0" tradeTax="0" debit="-0.47" credit="" amount="-0.47" tradeCode="" balance="15726.07" levelOfDetail="Currency" transactionID="1711006218"/>
<StatementOfFundsLine accountId="U11213636" acctAlias="" model="" currency="EUR" fxRateToBase="1" assetCategory="" symbol="" description="" conid="" securityID="" securityIDType="" cusip="" isin="" listingExchange="" underlyingConid="" underlyingSymbol="" underlyingSecurityID="" underlyingListingExchange="" issuer="" multiplier="0" strike="" expiry="" putCall="" principalAdjustFactor="" reportDate="2023-04-05" date="2023-04-05" settleDate="2023-04-05" activityCode="CINT" activityDescription="EUR Credit Interest for Mar-2023" tradeID="" orderID="" buySell="" tradeQuantity="0" tradePrice="0" tradeGross="0" tradeCommission="0" tradeTax="0" debit="" credit="2.99" amount="2.99" tradeCode="" balance="15729.06" levelOfDetail="Currency" transactionID="1782588154"/>
<StatementOfFundsLine accountId="U11213636" acctAlias="" model="" currency="EUR" fxRateToBase="1" assetCategory="" symbol="" description="" conid="" securityID="" securityIDType="" cusip="" isin="" listingExchange="" underlyingConid="" underlyingSymbol="" underlyingSecurityID="" underlyingListingExchange="" issuer="" multiplier="0" strike="" expiry="" putCall="" principalAdjustFactor="" reportDate="2023-04-05" date="2023-04-05" settleDate="2023-04-05" activityCode="FRTAX" activityDescription="Withholding @ 20% on Credit Interest for Mar-2023" tradeID="" orderID="" buySell="" tradeQuantity="0" tradePrice="0" tradeGross="0" tradeCommission="0" tradeTax="0" debit="-0.6" credit="" amount="-0.6" tradeCode="" balance="15728.46" levelOfDetail="Currency" transactionID="1783786859"/>
<StatementOfFundsLine accountId="U11213636" acctAlias="" model="" currency="EUR" fxRateToBase="1" assetCategory="" symbol="" description="" conid="" securityID="" securityIDType="" cusip="" isin="" listingExchange="" underlyingConid="" underlyingSymbol="" underlyingSecurityID="" underlyingListingExchange="" issuer="" multiplier="0" strike="" expiry="" putCall="" principalAdjustFactor="" reportDate="2023-04-28" date="2023-04-28" settleDate="2023-04-28" activityCode="DEP" activityDescription="Electronic Fund Transfer" tradeID="" orderID="" buySell="" tradeQuantity="0" tradePrice="0" tradeGross="0" tradeCommission="0" tradeTax="0" debit="" credit="10000" amount="10000" tradeCode="" balance="10812.2" levelOfDetail="Currency" transactionID="1830863972"/>
<StatementOfFundsLine accountId="U11213636" acctAlias="" model="" currency="EUR" fxRateToBase="1" assetCategory="" symbol="" description="" conid="" securityID="" securityIDType="" cusip="" isin="" listingExchange="" underlyingConid="" underlyingSymbol="" underlyingSecurityID="" underlyingListingExchange="" issuer="" multiplier="0" strike="" expiry="" putCall="" principalAdjustFactor="" reportDate="2023-05-03" date="2023-05-03" settleDate="2023-05-03" activityCode="CINT" activityDescription="EUR Credit Interest for Apr-2023" tradeID="" orderID="" buySell="" tradeQuantity="0" tradePrice="0" tradeGross="0" tradeCommission="0" tradeTax="0" debit="" credit="2.05" amount="2.05" tradeCode="" balance="10814.25" levelOfDetail="Currency" transactionID="1841131906"/>
<StatementOfFundsLine accountId="U11213636" acctAlias="" model="" currency="EUR" fxRateToBase="1" assetCategory="" symbol="" description="" conid="" securityID="" securityIDType="" cusip="" isin="" listingExchange="" underlyingConid="" underlyingSymbol="" underlyingSecurityID="" underlyingListingExchange="" issuer="" multiplier="0" strike="" expiry="" putCall="" principalAdjustFactor="" reportDate="2023-05-03" date="2023-05-03" settleDate="2023-05-03" activityCode="FRTAX" activityDescription="Withholding @ 20% on Credit Interest for Apr-2023" tradeID="" orderID="" buySell="" tradeQuantity="0" tradePrice="0" tradeGross="0" tradeCommission="0" tradeTax="0" debit="-0.41" credit="" amount="-0.41" tradeCode="" balance="10813.84" levelOfDetail="Currency" transactionID="1842175502"/>
<StatementOfFundsLine accountId="U11213636" acctAlias="" model="" currency="EUR" fxRateToBase="1" assetCategory="" symbol="" description="" conid="" securityID="" securityIDType="" cusip="" isin="" listingExchange="" underlyingConid="" underlyingSymbol="" underlyingSecurityID="" underlyingListingExchange="" issuer="" multiplier="0" strike="" expiry="" putCall="" principalAdjustFactor="" reportDate="2023-06-05" date="2023-06-05" settleDate="2023-06-05" activityCode="CINT" activityDescription="EUR Credit Interest for May-2023" tradeID="" orderID="" buySell="" tradeQuantity="0" tradePrice="0" tradeGross="0" tradeCommission="0" tradeTax="0" debit="" credit="0.62" amount="0.62" tradeCode="" balance="10814.46" levelOfDetail="Currency" transactionID="1912545850"/>
<StatementOfFundsLine accountId="U11213636" acctAlias="" model="" currency="EUR" fxRateToBase="1" assetCategory="" symbol="" description="" conid="" securityID="" securityIDType="" cusip="" isin="" listingExchange="" underlyingConid="" underlyingSymbol="" underlyingSecurityID="" underlyingListingExchange="" issuer="" multiplier="0" strike="" expiry="" putCall="" principalAdjustFactor="" reportDate="2023-06-05" date="2023-06-05" settleDate="2023-06-05" activityCode="FRTAX" activityDescription="Withholding @ 20% on Credit Interest for May-2023" tradeID="" orderID="" buySell="" tradeQuantity="0" tradePrice="0" tradeGross="0" tradeCommission="0" tradeTax="0" debit="-0.12" credit="" amount="-0.12" tradeCode="" balance="10814.34" levelOfDetail="Currency" transactionID="1913723272"/>
<StatementOfFundsLine accountId="U11213636" acctAlias="" model="" currency="EUR" fxRateToBase="1" assetCategory="" symbol="" description="" conid="" securityID="" securityIDType="" cusip="" isin="" listingExchange="" underlyingConid="" underlyingSymbol="" underlyingSecurityID="" underlyingListingExchange="" issuer="" multiplier="0" strike="" expiry="" putCall="" principalAdjustFactor="" reportDate="2023-07-06" date="2023-07-06" settleDate="2023-07-06" activityCode="CINT" activityDescription="EUR Credit Interest for Jun-2023" tradeID="" orderID="" buySell="" tradeQuantity="0" tradePrice="0" tradeGross="0" tradeCommission="0" tradeTax="0" debit="" credit="1.22" amount="1.22" tradeCode="" balance="11933.816647978" levelOfDetail="Currency" transactionID="1982305065"/>
<StatementOfFundsLine accountId="U11213636" acctAlias="" model="" currency="EUR" fxRateToBase="1" assetCategory="" symbol="" description="" conid="" securityID="" securityIDType="" cusip="" isin="" listingExchange="" underlyingConid="" underlyingSymbol="" underlyingSecurityID="" underlyingListingExchange="" issuer="" multiplier="0" strike="" expiry="" putCall="" principalAdjustFactor="" reportDate="2023-07-06" date="2023-07-06" settleDate="2023-07-06" activityCode="FRTAX" activityDescription="Withholding @ 20% on Credit Interest for Jun-2023" tradeID="" orderID="" buySell="" tradeQuantity="0" tradePrice="0" tradeGross="0" tradeCommission="0" tradeTax="0" debit="-0.24" credit="" amount="-0.24" tradeCode="" balance="11933.576647978" levelOfDetail="Currency" transactionID="1983127388"/>
<StatementOfFundsLine accountId="U11213636" acctAlias="" model="" currency="EUR" fxRateToBase="1" assetCategory="" symbol="" description="" conid="" securityID="" securityIDType="" cusip="" isin="" listingExchange="" underlyingConid="" underlyingSymbol="" underlyingSecurityID="" underlyingListingExchange="" issuer="" multiplier="0" strike="" expiry="" putCall="" principalAdjustFactor="" reportDate="2023-08-03" date="2023-08-03" settleDate="2023-08-03" activityCode="CINT" activityDescription="EUR Credit Interest for Jul-2023" tradeID="" orderID="" buySell="" tradeQuantity="0" tradePrice="0" tradeGross="0" tradeCommission="0" tradeTax="0" debit="" credit="1.69" amount="1.69" tradeCode="" balance="4999.648567978" levelOfDetail="Currency" transactionID="2045305365"/>
<StatementOfFundsLine accountId="U11213636" acctAlias="" model="" currency="EUR" fxRateToBase="1" assetCategory="" symbol="" description="" conid="" securityID="" securityIDType="" cusip="" isin="" listingExchange="" underlyingConid="" underlyingSymbol="" underlyingSecurityID="" underlyingListingExchange="" issuer="" multiplier="0" strike="" expiry="" putCall="" principalAdjustFactor="" reportDate="2023-08-03" date="2023-08-03" settleDate="2023-08-03" activityCode="FRTAX" activityDescription="Withholding @ 20% on Credit Interest for Jul-2023" tradeID="" orderID="" buySell="" tradeQuantity="0" tradePrice="0" tradeGross="0" tradeCommission="0" tradeTax="0" debit="-0.34" credit="" amount="-0.34" tradeCode="" balance="4999.308567978" levelOfDetail="Currency" transactionID="2046035823"/>
<StatementOfFundsLine accountId="U11213636" acctAlias="" model="" currency="EUR" fxRateToBase="1" assetCategory="BOND" symbol="DBR 1 08/15/24" description="DBR 1 08/15/24" conid="165809631" securityID="DE0001102366" securityIDType="ISIN" cusip="" isin="DE0001102366" listingExchange="" underlyingConid="" underlyingSymbol="" underlyingSecurityID="" underlyingListingExchange="" issuer="" multiplier="1" strike="" expiry="" putCall="" principalAdjustFactor="1" reportDate="2023-06-13" date="2023-06-13" settleDate="2023-06-13" activityCode="INTP" activityDescription="Purchase Accrued Interest DBR 1 08/15/24" tradeID="557608096" orderID="" buySell="" tradeQuantity="0" tradePrice="0" tradeGross="0" tradeCommission="0" tradeTax="0" debit="-41.64" credit="" amount="-41.64" tradeCode="" balance="10019.7044445" levelOfDetail="Currency" transactionID="1931663089"/>
<StatementOfFundsLine accountId="U11213636" acctAlias="" model="" currency="EUR" fxRateToBase="1" assetCategory="BOND" symbol="DBR 1 08/15/24" description="DBR 1 08/15/24" conid="165809631" securityID="DE0001102366" securityIDType="ISIN" cusip="" isin="DE0001102366" listingExchange="" underlyingConid="" underlyingSymbol="" underlyingSecurityID="" underlyingListingExchange="" issuer="" multiplier="1" strike="" expiry="" putCall="" principalAdjustFactor="1" reportDate="2023-06-13" date="2023-06-13" settleDate="2023-06-15" activityCode="BUY" activityDescription="Buy 5,000 DBR 1 08/15/24 " tradeID="" orderID="" buySell="BUY" tradeQuantity="5000" tradePrice="97.65" tradeGross="-4882.5" tradeCommission="-5" tradeTax="0" debit="-4887.5" credit="" amount="-4887.5" tradeCode="" balance="5132.2044445" levelOfDetail="Currency" transactionID="1931663089"/>
<StatementOfFundsLine accountId="U11213636" acctAlias="" model="" currency="EUR" fxRateToBase="1" assetCategory="BOND" symbol="DBR 1 08/15/24" description="DBR 1 08/15/24" conid="165809631" securityID="DE0001102366" securityIDType="ISIN" cusip="" isin="DE0001102366" listingExchange="" underlyingConid="" underlyingSymbol="" underlyingSecurityID="" underlyingListingExchange="" issuer="" multiplier="1" strike="" expiry="" putCall="" principalAdjustFactor="1" reportDate="2023-08-15" date="2023-08-15" settleDate="2023-08-15" activityCode="INTR" activityDescription="Bond Coupon Payment (DBR 1 08/15/24 - BUNDESREPUB. DEUTSCHLAND DBR 1 08/15/24)" tradeID="" orderID="" buySell="" tradeQuantity="0" tradePrice="0" tradeGross="0" tradeCommission="0" tradeTax="0" debit="" credit="50" amount="50" tradeCode="" balance="4296.312431178" levelOfDetail="Currency" transactionID="2070487617"/>
<StatementOfFundsLine accountId="U11213636" acctAlias="" model="" currency="EUR" fxRateToBase="1" assetCategory="BOND" symbol="DBR 1 3/4 02/15/24" description="DBR 1 3/4 02/15/24" conid="142870711" securityID="DE0001102333" securityIDType="ISIN" cusip="" isin="DE0001102333" listingExchange="" underlyingConid="" underlyingSymbol="" underlyingSecurityID="" underlyingListingExchange="" issuer="" multiplier="1" strike="" expiry="" putCall="" principalAdjustFactor="1" reportDate="2023-04-17" date="2023-04-17" settleDate="2023-04-17" activityCode="INTP" activityDescription="Purchase Accrued Interest DBR 1 3/4 02/15/24" tradeID="528448834" orderID="" buySell="" tradeQuantity="0" tradePrice="0" tradeGross="0" tradeCommission="0" tradeTax="0" debit="-15.1" credit="" amount="-15.1" tradeCode="" balance="15713.36" levelOfDetail="Currency" transactionID="1803862630"/>
<StatementOfFundsLine accountId="U11213636" acctAlias="" model="" currency="EUR" fxRateToBase="1" assetCategory="BOND" symbol="DBR 1 3/4 02/15/24" description="DBR 1 3/4 02/15/24" conid="142870711" securityID="DE0001102333" securityIDType="ISIN" cusip="" isin="DE0001102333" listingExchange="" underlyingConid="" underlyingSymbol="" underlyingSecurityID="" underlyingListingExchange="" issuer="" multiplier="1" strike="" expiry="" putCall="" principalAdjustFactor="1" reportDate="2023-04-17" date="2023-04-17" settleDate="2023-04-19" activityCode="BUY" activityDescription="Buy 5,000 DBR 1 3/4 02/15/24 " tradeID="" orderID="" buySell="BUY" tradeQuantity="5000" tradePrice="99.05" tradeGross="-4952.5" tradeCommission="-5" tradeTax="0" debit="-4957.5" credit="" amount="-4957.5" tradeCode="" balance="10755.86" levelOfDetail="Currency" transactionID="1803862630"/>
<StatementOfFundsLine accountId="U11213636" acctAlias="" model="" currency="EUR" fxRateToBase="1" assetCategory="BOND" symbol="DBR 1 3/4 02/15/24" description="DBR 1 3/4 02/15/24" conid="142870711" securityID="DE0001102333" securityIDType="ISIN" cusip="" isin="DE0001102333" listingExchange="" underlyingConid="" underlyingSymbol="" underlyingSecurityID="" underlyingListingExchange="" issuer="" multiplier="1" strike="" expiry="" putCall="" principalAdjustFactor="1" reportDate="2023-04-19" date="2023-04-19" settleDate="2023-04-19" activityCode="INTP" activityDescription="Purchase Accrued Interest DBR 1 3/4 02/15/24" tradeID="530018221" orderID="" buySell="" tradeQuantity="0" tradePrice="0" tradeGross="0" tradeCommission="0" tradeTax="0" debit="-31.16" credit="" amount="-31.16" tradeCode="" balance="10724.7" levelOfDetail="Currency" transactionID="1810141457"/>
<StatementOfFundsLine accountId="U11213636" acctAlias="" model="" currency="EUR" fxRateToBase="1" assetCategory="BOND" symbol="DBR 1 3/4 02/15/24" description="DBR 1 3/4 02/15/24" conid="142870711" securityID="DE0001102333" securityIDType="ISIN" cusip="" isin="DE0001102333" listingExchange="" underlyingConid="" underlyingSymbol="" underlyingSecurityID="" underlyingListingExchange="" issuer="" multiplier="1" strike="" expiry="" putCall="" principalAdjustFactor="1" reportDate="2023-04-19" date="2023-04-19" settleDate="2023-04-21" activityCode="BUY" activityDescription="Buy 10,000 DBR 1 3/4 02/15/24 " tradeID="" orderID="" buySell="BUY" tradeQuantity="10000" tradePrice="99.025" tradeGross="-9902.5" tradeCommission="-10" tradeTax="0" debit="-9912.5" credit="" amount="-9912.5" tradeCode="" balance="812.2" levelOfDetail="Currency" transactionID="1810141457"/>
<StatementOfFundsLine accountId="U11213636" acctAlias="" model="" currency="EUR" fxRateToBase="1" assetCategory="CASH" symbol="" description="" conid="" securityID="" securityIDType="" cusip="" isin="" listingExchange="" underlyingConid="" underlyingSymbol="" underlyingSecurityID="" underlyingListingExchange="" issuer="" multiplier="0" strike="" expiry="" putCall="" principalAdjustFactor="" reportDate="2023-01-30" date="2023-01-30" settleDate="2023-01-31" activityCode="DEP" activityDescription="Cash Transfer" tradeID="" orderID="" buySell="" tradeQuantity="0" tradePrice="0" tradeGross="0" tradeCommission="0" tradeTax="0" debit="" credit="15724.15" amount="15724.15" tradeCode="" balance="15724.15" levelOfDetail="Currency" transactionID="1635131224"/>
<StatementOfFundsLine accountId="U11213636" acctAlias="" model="" currency="EUR" fxRateToBase="1" assetCategory="CASH" symbol="" description="" conid="" securityID="" securityIDType="" cusip="" isin="" listingExchange="" underlyingConid="" underlyingSymbol="" underlyingSecurityID="" underlyingListingExchange="" issuer="" multiplier="0" strike="" expiry="" putCall="" principalAdjustFactor="" reportDate="2023-01-30" date="2023-01-30" settleDate="2023-01-31" activityCode="DEP" activityDescription="Cash Transfer" tradeID="" orderID="" buySell="" tradeQuantity="0" tradePrice="0" tradeGross="0" tradeCommission="0" tradeTax="0" debit="" credit="15724.15" amount="15724.15" tradeCode="" balance="15724.15" levelOfDetail="Currency" transactionID="1635131224"/>
<StatementOfFundsLine accountId="U11213636" acctAlias="" model="" currency="EUR" fxRateToBase="1" assetCategory="CASH" symbol="" description="" conid="" securityID="" securityIDType="" cusip="" isin="" listingExchange="" underlyingConid="" underlyingSymbol="" underlyingSecurityID="" underlyingListingExchange="" issuer="" multiplier="0" strike="" expiry="" putCall="" principalAdjustFactor="" reportDate="2023-07-24" date="2023-07-24" settleDate="2023-07-24" activityCode="WITH" activityDescription="Cash Transfer" tradeID="" orderID="" buySell="" tradeQuantity="0" tradePrice="0" tradeGross="0" tradeCommission="0" tradeTax="0" debit="-100" credit="" amount="-100" tradeCode="" balance="11833.576647978" levelOfDetail="Currency" transactionID="2018788220"/>
<StatementOfFundsLine accountId="U11213636" acctAlias="" model="" currency="EUR" fxRateToBase="1" assetCategory="CASH" symbol="" description="" conid="" securityID="" securityIDType="" cusip="" isin="" listingExchange="" underlyingConid="" underlyingSymbol="" underlyingSecurityID="" underlyingListingExchange="" issuer="" multiplier="0" strike="" expiry="" putCall="" principalAdjustFactor="" reportDate="2023-07-24" date="2023-07-24" settleDate="2023-07-24" activityCode="WITH" activityDescription="Cash Transfer" tradeID="" orderID="" buySell="" tradeQuantity="0" tradePrice="0" tradeGross="0" tradeCommission="0" tradeTax="0" debit="-210" credit="" amount="-210" tradeCode="" balance="11623.576647978" levelOfDetail="Currency" transactionID="2019158007"/>
<StatementOfFundsLine accountId="U11213636" acctAlias="" model="" currency="EUR" fxRateToBase="1" assetCategory="CASH" symbol="" description="" conid="" securityID="" securityIDType="" cusip="" isin="" listingExchange="" underlyingConid="" underlyingSymbol="" underlyingSecurityID="" underlyingListingExchange="" issuer="" multiplier="0" strike="" expiry="" putCall="" principalAdjustFactor="" reportDate="2023-07-24" date="2023-07-24" settleDate="2023-07-24" activityCode="WITH" activityDescription="Cash Transfer" tradeID="" orderID="" buySell="" tradeQuantity="0" tradePrice="0" tradeGross="0" tradeCommission="0" tradeTax="0" debit="-75" credit="" amount="-75" tradeCode="" balance="11548.576647978" levelOfDetail="Currency" transactionID="2019585022"/>
<StatementOfFundsLine accountId="U11213636" acctAlias="" model="" currency="EUR" fxRateToBase="1" assetCategory="OPT" symbol="P ODAX 20230915 15500 M" description="DAX 15SEP23 15500 P" conid="568982876" securityID="" securityIDType="" cusip="" isin="" listingExchange="EUREX" underlyingConid="825711" underlyingSymbol="DAX" underlyingSecurityID="" underlyingListingExchange="" issuer="" multiplier="5" strike="15500" expiry="2023-09-15" putCall="P" principalAdjustFactor="" reportDate="2023-08-25" date="2023-08-25" settleDate="2023-08-28" activityCode="BUY" activityDescription="Buy 1 DAX 15SEP23 15500 P " tradeID="" orderID="" buySell="BUY" tradeQuantity="1" tradePrice="159.5" tradeGross="-797.5" tradeCommission="-1.7" tradeTax="0" debit="-799.2" credit="" amount="-799.2" tradeCode="" balance="4706.262431178" levelOfDetail="Currency" transactionID="2094918405"/>
<StatementOfFundsLine accountId="U11213636" acctAlias="" model="" currency="EUR" fxRateToBase="1" assetCategory="OPT" symbol="P ODAX 20230915 15500 M" description="DAX 15SEP23 15500 P" conid="568982876" securityID="" securityIDType="" cusip="" isin="" listingExchange="EUREX" underlyingConid="825711" underlyingSymbol="DAX" underlyingSecurityID="" underlyingListingExchange="" issuer="" multiplier="5" strike="15500" expiry="2023-09-15" putCall="P" principalAdjustFactor="" reportDate="2023-08-29" date="2023-08-29" settleDate="2023-08-30" activityCode="BUY" activityDescription="Buy 1 DAX 15SEP23 15500 P " tradeID="" orderID="" buySell="BUY" tradeQuantity="1" tradePrice="61.5" tradeGross="-307.5" tradeCommission="-1.7" tradeTax="0" debit="-309.2" credit="" amount="-309.2" tradeCode="" balance="4397.062431178" levelOfDetail="Currency" transactionID="2101352096"/>
<StatementOfFundsLine accountId="U11213636" acctAlias="" model="" currency="EUR" fxRateToBase="1" assetCategory="OPT" symbol="P ODAX 20230915 15500 M" description="DAX 15SEP23 15500 P" conid="568982876" securityID="" securityIDType="" cusip="" isin="" listingExchange="EUREX" underlyingConid="825711" underlyingSymbol="DAX" underlyingSecurityID="" underlyingListingExchange="" issuer="" multiplier="5" strike="15500" expiry="2023-09-15" putCall="P" principalAdjustFactor="" reportDate="2023-08-29" date="2023-08-29" settleDate="2023-08-30" activityCode="SELL" activityDescription="Sell -2 DAX 15SEP23 15500 P " tradeID="" orderID="" buySell="SELL" tradeQuantity="-2" tradePrice="59" tradeGross="590" tradeCommission="-3.4" tradeTax="0" debit="" credit="586.6" amount="586.6" tradeCode="" balance="4983.662431178" levelOfDetail="Currency" transactionID="2101352123"/>
<StatementOfFundsLine accountId="U11213636" acctAlias="" model="" currency="EUR" fxRateToBase="1" assetCategory="OPT" symbol="P ODAX 20230915 15800 M" description="DAX 15SEP23 15800 P" conid="565045125" securityID="" securityIDType="" cusip="" isin="" listingExchange="EUREX" underlyingConid="825711" underlyingSymbol="DAX" underlyingSecurityID="" underlyingListingExchange="" issuer="" multiplier="5" strike="15800" expiry="2023-09-15" putCall="P" principalAdjustFactor="" reportDate="2023-09-04" date="2023-09-04" settleDate="2023-09-05" activityCode="BUY" activityDescription="Buy 1 DAX 15SEP23 15800 P " tradeID="" orderID="" buySell="BUY" tradeQuantity="1" tradePrice="89.5" tradeGross="-447.5" tradeCommission="-1.7" tradeTax="0" debit="-449.2" credit="" amount="-449.2" tradeCode="" balance="4534.462431178" levelOfDetail="Currency" transactionID="2113642759"/>
<StatementOfFundsLine accountId="U11213636" acctAlias="" model="" currency="EUR" fxRateToBase="1" assetCategory="OPT" symbol="P ODAX 20230915 15800 M" description="DAX 15SEP23 15800 P" conid="565045125" securityID="" securityIDType="" cusip="" isin="" listingExchange="EUREX" underlyingConid="825711" underlyingSymbol="DAX" underlyingSecurityID="" underlyingListingExchange="" issuer="" multiplier="5" strike="15800" expiry="2023-09-15" putCall="P" principalAdjustFactor="" reportDate="2023-09-05" date="2023-09-05" settleDate="2023-09-06" activityCode="SELL" activityDescription="Sell -1 DAX 15SEP23 15800 P " tradeID="" orderID="" buySell="SELL" tradeQuantity="-1" tradePrice="123" tradeGross="615" tradeCommission="-1.7" tradeTax="0" debit="" credit="613.3" amount="613.3" tradeCode="" balance="5147.762431178" levelOfDetail="Currency" transactionID="2115658678"/>
<StatementOfFundsLine accountId="U11213636" acctAlias="" model="" currency="EUR" fxRateToBase="1" assetCategory="OPT" symbol="P ODAX 20231103 14600 W" description="DAX 03NOV23 14600 P" conid="656318775" securityID="" securityIDType="" cusip="" isin="" listingExchange="EUREX" underlyingConid="825711" underlyingSymbol="DAX" underlyingSecurityID="" underlyingListingExchange="" issuer="" multiplier="5" strike="14600" expiry="2023-11-03" putCall="P" principalAdjustFactor="" reportDate="2023-10-26" date="2023-10-26" settleDate="2023-10-27" activityCode="BUY" activityDescription="Buy 1 DAX 03NOV23 14600 P " tradeID="" orderID="" buySell="BUY" tradeQuantity="1" tradePrice="115" tradeGross="-575" tradeCommission="-1.7" tradeTax="0" debit="-576.7" credit="" amount="-576.7" tradeCode="" balance="6034.427475778" levelOfDetail="Currency" transactionID="2231671543"/>
<StatementOfFundsLine accountId="U11213636" acctAlias="" model="" currency="EUR" fxRateToBase="1" assetCategory="OPT" symbol="P ODAX 20231103 14600 W" description="DAX 03NOV23 14600 P" conid="656318775" securityID="" securityIDType="" cusip="" isin="" listingExchange="EUREX" underlyingConid="825711" underlyingSymbol="DAX" underlyingSecurityID="" underlyingListingExchange="" issuer="" multiplier="5" strike="14600" expiry="2023-11-03" putCall="P" principalAdjustFactor="" reportDate="2023-10-27" date="2023-10-27" settleDate="2023-10-30" activityCode="BUY" activityDescription="Buy 1 DAX 03NOV23 14600 P " tradeID="" orderID="" buySell="BUY" tradeQuantity="1" tradePrice="71" tradeGross="-355" tradeCommission="-1.7" tradeTax="0" debit="-356.7" credit="" amount="-356.7" tradeCode="" balance="5677.727475778" levelOfDetail="Currency" transactionID="2234804849"/>
<StatementOfFundsLine accountId="U11213636" acctAlias="" model="" currency="EUR" fxRateToBase="1" assetCategory="OPT" symbol="P ODAX 20231103 14600 W" description="DAX 03NOV23 14600 P" conid="656318775" securityID="" securityIDType="" cusip="" isin="" listingExchange="EUREX" underlyingConid="825711" underlyingSymbol="DAX" underlyingSecurityID="" underlyingListingExchange="" issuer="" multiplier="5" strike="14600" expiry="2023-11-03" putCall="P" principalAdjustFactor="" reportDate="2023-10-27" date="2023-10-27" settleDate="2023-10-30" activityCode="SELL" activityDescription="Sell -1 DAX 03NOV23 14600 P " tradeID="" orderID="" buySell="SELL" tradeQuantity="-1" tradePrice="85" tradeGross="425" tradeCommission="-1.7" tradeTax="0" debit="" credit="423.3" amount="423.3" tradeCode="" balance="6101.027475778" levelOfDetail="Currency" transactionID="2235239151"/>
<StatementOfFundsLine accountId="U11213636" acctAlias="" model="" currency="EUR" fxRateToBase="1" assetCategory="OPT" symbol="P ODAX 20231103 14600 W" description="DAX 03NOV23 14600 P" conid="656318775" securityID="" securityIDType="" cusip="" isin="" listingExchange="EUREX" underlyingConid="825711" underlyingSymbol="DAX" underlyingSecurityID="" underlyingListingExchange="" issuer="" multiplier="5" strike="14600" expiry="2023-11-03" putCall="P" principalAdjustFactor="" reportDate="2023-11-02" date="2023-11-02" settleDate="2023-11-03" activityCode="SELL" activityDescription="Sell -1 DAX 03NOV23 14600 P " tradeID="" orderID="" buySell="SELL" tradeQuantity="-1" tradePrice="0.2" tradeGross="1" tradeCommission="-1.7" tradeTax="0" debit="-0.7" credit="" amount="-0.7" tradeCode="" balance="6100.327475778" levelOfDetail="Currency" transactionID="2247751188"/>
<StatementOfFundsLine accountId="U11213636" acctAlias="" model="" currency="EUR" fxRateToBase="1" assetCategory="OPT" symbol="P ODXS 20230915 15700 M" description="DAX 15SEP23 15700 P" conid="611912335" securityID="" securityIDType="" cusip="" isin="" listingExchange="EUREX" underlyingConid="825711" underlyingSymbol="DAX" underlyingSecurityID="" underlyingListingExchange="" issuer="" multiplier="1" strike="15700" expiry="2023-09-15" putCall="P" principalAdjustFactor="" reportDate="2023-09-05" date="2023-09-05" settleDate="2023-09-06" activityCode="BUY" activityDescription="Buy 5 DAX 15SEP23 15700 P " tradeID="" orderID="" buySell="BUY" tradeQuantity="5" tradePrice="97.1" tradeGross="-485.5" tradeCommission="-8.5" tradeTax="0" debit="-494" credit="" amount="-494" tradeCode="" balance="4653.762431178" levelOfDetail="Currency" transactionID="2115753022"/>
<StatementOfFundsLine accountId="U11213636" acctAlias="" model="" currency="EUR" fxRateToBase="1" assetCategory="OPT" symbol="P ODXS 20230915 15700 M" description="DAX 15SEP23 15700 P" conid="611912335" securityID="" securityIDType="" cusip="" isin="" listingExchange="EUREX" underlyingConid="825711" underlyingSymbol="DAX" underlyingSecurityID="" underlyingListingExchange="" issuer="" multiplier="1" strike="15700" expiry="2023-09-15" putCall="P" principalAdjustFactor="" reportDate="2023-09-05" date="2023-09-05" settleDate="2023-09-06" activityCode="SELL" activityDescription="Sell -2 DAX 15SEP23 15700 P " tradeID="" orderID="" buySell="SELL" tradeQuantity="-2" tradePrice="92" tradeGross="184" tradeCommission="-3.4" tradeTax="0" debit="" credit="180.6" amount="180.6" tradeCode="" balance="4834.362431178" levelOfDetail="Currency" transactionID="2115797686"/>
<StatementOfFundsLine accountId="U11213636" acctAlias="" model="" currency="EUR" fxRateToBase="1" assetCategory="OPT" symbol="P ODXS 20230915 15700 M" description="DAX 15SEP23 15700 P" conid="611912335" securityID="" securityIDType="" cusip="" isin="" listingExchange="EUREX" underlyingConid="825711" underlyingSymbol="DAX" underlyingSecurityID="" underlyingListingExchange="" issuer="" multiplier="1" strike="15700" expiry="2023-09-15" putCall="P" principalAdjustFactor="" reportDate="2023-09-08" date="2023-09-08" settleDate="2023-09-11" activityCode="SELL" activityDescription="Sell -3 DAX 15SEP23 15700 P " tradeID="" orderID="" buySell="SELL" tradeQuantity="-3" tradePrice="174" tradeGross="522" tradeCommission="-5.1" tradeTax="0" debit="" credit="516.9" amount="516.9" tradeCode="" balance="5351.262431178" levelOfDetail="Currency" transactionID="2125366488"/>
<StatementOfFundsLine accountId="U11213636" acctAlias="" model="" currency="EUR" fxRateToBase="1" assetCategory="OPT" symbol="P ODXS 20231020 15000 M" description="DAX 20OCT23 15000 P" conid="644436062" securityID="" securityIDType="" cusip="" isin="" listingExchange="EUREX" underlyingConid="825711" underlyingSymbol="DAX" underlyingSecurityID="" underlyingListingExchange="" issuer="" multiplier="1" strike="15000" expiry="2023-10-20" putCall="P" principalAdjustFactor="" reportDate="2023-09-29" date="2023-09-29" settleDate="2023-10-02" activityCode="BUY" activityDescription="Buy 5 DAX 20OCT23 15000 P " tradeID="" orderID="" buySell="BUY" tradeQuantity="5" tradePrice="73.5" tradeGross="-367.5" tradeCommission="-8.5" tradeTax="0" debit="-376" credit="" amount="-376" tradeCode="" balance="6004.627475778" levelOfDetail="Currency" transactionID="2171656166"/>
<StatementOfFundsLine accountId="U11213636" acctAlias="" model="" currency="EUR" fxRateToBase="1" assetCategory="OPT" symbol="P ODXS 20231020 15000 M" description="DAX 20OCT23 15000 P" conid="644436062" securityID="" securityIDType="" cusip="" isin="" listingExchange="EUREX" underlyingConid="825711" underlyingSymbol="DAX" underlyingSecurityID="" underlyingListingExchange="" issuer="" multiplier="1" strike="15000" expiry="2023-10-20" putCall="P" principalAdjustFactor="" reportDate="2023-10-02" date="2023-10-02" settleDate="2023-10-03" activityCode="SELL" activityDescription="Sell -2 DAX 20OCT23 15000 P " tradeID="" orderID="" buySell="SELL" tradeQuantity="-2" tradePrice="135" tradeGross="270" tradeCommission="-3.4" tradeTax="0" debit="" credit="266.6" amount="266.6" tradeCode="" balance="6271.227475778" levelOfDetail="Currency" transactionID="2175542268"/>
<StatementOfFundsLine accountId="U11213636" acctAlias="" model="" currency="EUR" fxRateToBase="1" assetCategory="OPT" symbol="P ODXS 20231020 15000 M" description="DAX 20OCT23 15000 P" conid="644436062" securityID="" securityIDType="" cusip="" isin="" listingExchange="EUREX" underlyingConid="825711" underlyingSymbol="DAX" underlyingSecurityID="" underlyingListingExchange="" issuer="" multiplier="1" strike="15000" expiry="2023-10-20" putCall="P" principalAdjustFactor="" reportDate="2023-10-04" date="2023-10-04" settleDate="2023-10-05" activityCode="SELL" activityDescription="Sell -1 DAX 20OCT23 15000 P " tradeID="" orderID="" buySell="SELL" tradeQuantity="-1" tradePrice="175" tradeGross="175" tradeCommission="-1.7" tradeTax="0" debit="" credit="173.3" amount="173.3" tradeCode="" balance="6444.527475778" levelOfDetail="Currency" transactionID="2182034036"/>
<StatementOfFundsLine accountId="U11213636" acctAlias="" model="" currency="EUR" fxRateToBase="1" assetCategory="OPT" symbol="P ODXS 20231020 15000 M" description="DAX 20OCT23 15000 P" conid="644436062" securityID="" securityIDType="" cusip="" isin="" listingExchange="EUREX" underlyingConid="825711" underlyingSymbol="DAX" underlyingSecurityID="" underlyingListingExchange="" issuer="" multiplier="1" strike="15000" expiry="2023-10-20" putCall="P" principalAdjustFactor="" reportDate="2023-10-09" date="2023-10-09" settleDate="2023-10-10" activityCode="SELL" activityDescription="Sell -1 DAX 20OCT23 15000 P " tradeID="" orderID="" buySell="SELL" tradeQuantity="-1" tradePrice="125" tradeGross="125" tradeCommission="-1.7" tradeTax="0" debit="" credit="123.3" amount="123.3" tradeCode="" balance="6567.827475778" levelOfDetail="Currency" transactionID="2191561460"/>
<StatementOfFundsLine accountId="U11213636" acctAlias="" model="" currency="EUR" fxRateToBase="1" assetCategory="OPT" symbol="P ODXS 20231020 15000 M" description="DAX 20OCT23 15000 P" conid="644436062" securityID="" securityIDType="" cusip="" isin="" listingExchange="EUREX" underlyingConid="825711" underlyingSymbol="DAX" underlyingSecurityID="" underlyingListingExchange="" issuer="" multiplier="1" strike="15000" expiry="2023-10-20" putCall="P" principalAdjustFactor="" reportDate="2023-10-10" date="2023-10-10" settleDate="2023-10-11" activityCode="SELL" activityDescription="Sell -1 DAX 20OCT23 15000 P " tradeID="" orderID="" buySell="SELL" tradeQuantity="-1" tradePrice="45" tradeGross="45" tradeCommission="-1.7" tradeTax="0" debit="" credit="43.3" amount="43.3" tradeCode="" balance="6611.127475778" levelOfDetail="Currency" transactionID="2194924326"/>
<StatementOfFundsLine accountId="U11213636" acctAlias="" model="" currency="EUR" fxRateToBase="1" assetCategory="OPT" symbol="P ODXS 20231215 14800 M" description="DAX 15DEC23 14800 P" conid="611912519" securityID="" securityIDType="" cusip="" isin="" listingExchange="EUREX" underlyingConid="825711" underlyingSymbol="DAX" underlyingSecurityID="" underlyingListingExchange="" issuer="" multiplier="1" strike="14800" expiry="2023-12-15" putCall="P" principalAdjustFactor="" reportDate="2023-11-08" date="2023-11-08" settleDate="2023-11-09" activityCode="BUY" activityDescription="Buy 4 DAX 15DEC23 14800 P " tradeID="" orderID="" buySell="BUY" tradeQuantity="4" tradePrice="102.5" tradeGross="-410" tradeCommission="-6.8" tradeTax="0" debit="-416.8" credit="" amount="-416.8" tradeCode="" balance="5683.527475778" levelOfDetail="Currency" transactionID="2261552830"/>
<StatementOfFundsLine accountId="U11213636" acctAlias="" model="" currency="EUR" fxRateToBase="1" assetCategory="STK" symbol="IUSA" description="ISHARES CORE SP 500" conid="29651319" securityID="IE0031442068" securityIDType="ISIN" cusip="" isin="IE0031442068" listingExchange="AEB" underlyingConid="" underlyingSymbol="" underlyingSecurityID="" underlyingListingExchange="" issuer="" multiplier="1" strike="" expiry="" putCall="" principalAdjustFactor=""                   reportDate="2023-08-09" date="2023-08-09" settleDate="2023-08-11" activityCode="BUY" activityDescription="Buy 18.3553 ISHARES CORE S&P 500 " tradeID="" orderID="" buySell="BUY" tradeQuantity="18.3553" tradePrice="40.859922573" tradeGross="-749.9961368" tradeCommission="-3" tradeTax="0" debit="-752.9961368" credit="" amount="-752.9961368" tradeCode="" balance="4246.312431178" levelOfDetail="Currency" transactionID="2058074598"/>
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
<OpenPosition accountId="U11213636" acctAlias="" model="" currency="USD" fxRateToBase="0.90606" assetCategory="BOND" symbol="T 2 1/4 03/31/24" description="T 2 1/4 03/31/24" conid="553749289" securityID="US91282CEG24" securityIDType="ISIN" cusip="91282CEG2" isin="US91282CEG24" listingExchange="" underlyingConid="" underlyingSymbol="" underlyingSecurityID="" underlyingListingExchange="" issuer="" multiplier="1" strike="" expiry="" putCall="" principalAdjustFactor="1" reportDate="2023-12-29" position="10000" markPrice="99.3125" positionValue="9931.25" openPrice="97.0427" costBasisPrice="97.0427" costBasisMoney="9704.27" percentOfNAV="34.69" fifoPnlUnrealized="226.98" side="Long" levelOfDetail="SUMMARY" openDateTime="" holdingPeriodDateTime="" vestingDate="" code="" originatingOrderID="" originatingTransactionID="" accruedInt="57.79" serialNumber="" deliveryType="" commodityType="" fineness="0.0" weight="0.0"/>
<OpenPosition accountId="U11213636" acctAlias="" model="" currency="EUR" fxRateToBase="1" assetCategory="OPT" symbol="P ODXS 20231215 14800 M" description="DAX 15DEC23 14800 P" conid="611912519" securityID="" securityIDType="" cusip="" isin="" listingExchange="EUREX" underlyingConid="825711" underlyingSymbol="DAX" underlyingSecurityID="" underlyingListingExchange="" issuer="" multiplier="1" strike="14800" expiry="2023-12-15" putCall="P" principalAdjustFactor="" reportDate="2023-11-09" position="4" markPrice="82.7" positionValue="330.8" openPrice="104.2" costBasisPrice="104.2" costBasisMoney="416.8" percentOfNAV="100.00" fifoPnlUnrealized="-86" side="Long" levelOfDetail="SUMMARY" openDateTime="" holdingPeriodDateTime="" vestingDate="" code="" originatingOrderID="" originatingTransactionID="" accruedInt="" serialNumber="" deliveryType="" commodityType="" fineness="0.0" weight="0.0 ()"/>
<OpenPosition accountId="U11213636" acctAlias="" model="" currency="EUR" fxRateToBase="1" assetCategory="BOND" symbol="DBR 1 08/15/24" description="DBR 1 08/15/24" conid="165809631" securityID="DE0001102366" securityIDType="ISIN" cusip="" isin="DE0001102366" listingExchange="" underlyingConid="" underlyingSymbol="" underlyingSecurityID="" underlyingListingExchange="" issuer="" multiplier="1" strike="" expiry="" putCall="" principalAdjustFactor="1" reportDate="2023-11-09" position="5000" markPrice="98.113" positionValue="4905.65" openPrice="97.75" costBasisPrice="97.75" costBasisMoney="4887.5" percentOfNAV="20.05" fifoPnlUnrealized="18.15" side="Long" levelOfDetail="SUMMARY" openDateTime="" holdingPeriodDateTime="" vestingDate="" code="" originatingOrderID="" originatingTransactionID="" accruedInt="12.3" serialNumber="" deliveryType="" commodityType="" fineness="0.0" weight="0.0 ()"/>
<OpenPosition accountId="U11213636" acctAlias="" model="" currency="EUR" fxRateToBase="1" assetCategory="BOND" symbol="DBR 1 3/4 02/15/24" description="DBR 1 3/4 02/15/24" conid="142870711" securityID="DE0001102333" securityIDType="ISIN" cusip="" isin="DE0001102333" listingExchange="" underlyingConid="" underlyingSymbol="" underlyingSecurityID="" underlyingListingExchange="" issuer="" multiplier="1" strike="" expiry="" putCall="" principalAdjustFactor="1" reportDate="2023-11-09" position="15000" markPrice="99.502" positionValue="14925.3" openPrice="99.133333333" costBasisPrice="99.133333333" costBasisMoney="14870" percentOfNAV="61.01" fifoPnlUnrealized="55.3" side="Long" levelOfDetail="SUMMARY" openDateTime="" holdingPeriodDateTime="" vestingDate="" code="" originatingOrderID="" originatingTransactionID="" accruedInt="194.9" serialNumber="" deliveryType="" commodityType="" fineness="0.0" weight="0.0 ()"/>
<OpenPosition accountId="U11213636" acctAlias="" model="" currency="USD" fxRateToBase="0.93732" assetCategory="BOND" symbol="T 2 1/4 03/31/24" description="T 2 1/4 03/31/24" conid="553749289" securityID="US91282CEG24" securityIDType="ISIN" cusip="91282CEG2" isin="US91282CEG24" listingExchange="" underlyingConid="" underlyingSymbol="" underlyingSecurityID="" underlyingListingExchange="" issuer="" multiplier="1" strike="" expiry="" putCall="" principalAdjustFactor="1" reportDate="2023-11-09" position="5000" markPrice="98.828125" positionValue="4941.41" openPrice="96.9284" costBasisPrice="96.9284" costBasisMoney="4846.42" percentOfNAV="18.93" fifoPnlUnrealized="94.99" side="Long" levelOfDetail="SUMMARY" openDateTime="" holdingPeriodDateTime="" vestingDate="" code="" originatingOrderID="" originatingTransactionID="" accruedInt="12.6" serialNumber="" deliveryType="" commodityType="" fineness="0.0" weight="0.0 ()"/>
</OpenPositions>
</FlexStatement>
"#;
        let xml = xml.replace('&', "&amp;");
        let response: FlexStatement = from_str(&xml).unwrap();
        tracing::error!("Response - {response:#?}");
        if let super::trades::TradeElements::Trade(trade) =
            &response.trades.as_ref().unwrap().items[1]
        {
            assert_eq!(trade.contract.symbol, "VWRL");
        } else {
            panic!("");
        }
    }

    #[tokio::test]
    async fn flex_deserialize_statement_full() {
        let response: FlexStatement = flex_statement_from_file(std::path::PathBuf::from(
            r"/home/czichy/Dokumente/finanzen/ledger/import/ib/U11213636/1-in/2023/U11213636_2023.xml",
        ))
        .await
        .unwrap();
        tracing::error!("Response - {response:#?}");
        if let super::trades::TradeElements::Trade(trade) =
            &response.trades.as_ref().unwrap().items[1]
        {
            assert_eq!(trade.contract.symbol, "VWRL");
        } else {
            panic!("");
        }
    }
}
