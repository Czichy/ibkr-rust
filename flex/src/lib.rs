use std::{error, fmt, path::PathBuf};

pub use flex_statement::*;
use quick_xml::de::from_str;
use reqwest::header::USER_AGENT;
use serde::de::DeserializeOwned;
use tokio::{fs::File, io::AsyncWriteExt};

pub mod config;
pub mod enums;
pub mod flex_statement;
pub mod utils;

const MAX_RETRIES: u32 = 10;
const FIRST_RETRY_DELAY: u64 = 1;

/// Error returned by most functions.
///
/// When writing a real application, one might want to consider a specialized
/// error handling crate or defining an error type as an `enum` of causes.
/// However, for our example, using a boxed `std::error::Error` is sufficient.
///
/// For performance reasons, boxing is avoided in any hot path. For example, in
/// `parse`, a custom error `enum` is defined. This is because the error is hit
/// and handled during normal execution when a partial frame is received on a
/// socket. `std::error::Error` is implemented for `parse::Error` which allows
/// it to be converted to `Box<dyn std::error::Error>`.
pub type Error = Box<dyn std::error::Error + Send + Sync>;

/// A specialized `Result` type for mini-redis operations.
///
/// This is defined as a convenience.
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum FlexError {
    Reqwest(reqwest::Error),
    StatusCode(reqwest::StatusCode),
    Deserialization(quick_xml::DeError),
    SetFailed(String),
    Save(std::io::Error),
}
impl fmt::Display for FlexError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Reqwest(err) => err.fmt(f),
            Self::Deserialization(err) => err.fmt(f),
            Self::StatusCode(status_code) => {
                write!(
                    f,
                    "{} {}",
                    status_code.as_str(),
                    status_code.canonical_reason().unwrap_or_default(),
                )
            },
            Self::SetFailed(code) => write!(f, "{}", code),
            Self::Save(io) => write!(f, "{}", io),
        }
    }
}

impl error::Error for FlexError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Self::Reqwest(err) => Some(err),
            Self::Deserialization(err) => Some(err),
            Self::Save(err) => Some(err),
            _ => None,
        }
    }
}

impl From<reqwest::Error> for FlexError {
    fn from(err: reqwest::Error) -> Self { Self::Reqwest(err) }
}

impl From<quick_xml::DeError> for FlexError {
    fn from(err: quick_xml::DeError) -> Self { Self::Deserialization(err) }
}

impl From<std::io::Error> for FlexError {
    fn from(err: std::io::Error) -> Self { Self::Save(err) }
}
#[derive(Default)]
pub struct FlexReader {
    pub write_to_path:      Option<PathBuf>,
    pub override_file_name: Option<String>,
}

impl FlexReader {
    async fn retried_request<T: DeserializeOwned>(&self, url: &str) -> Result<T> {
        let mut retries = MAX_RETRIES;
        let mut wait = FIRST_RETRY_DELAY;
        loop {
            tracing::info!("retries: {} fetching report: {}", retries, &url);
            let client = reqwest::Client::new();
            // let response = reqwest::get(url).await?;
            let response = client
                .get(url)
                .header(
                    USER_AGENT,
                    "Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:87.0) Gecko/20100101 Firefox/87.0",
                )
                .send()
                .await?;
            tracing::debug!("{:#?}", response);
            let bytes = response.text().await?;
            let bytes = bytes.replace('&', "&amp;");
            tracing::error!(
                "\n{} \n...\n {}",
                &bytes[0..175],
                &bytes[&bytes.len() - 175..bytes.len()]
            );
            // tracing::error!("{}",);
            if let Ok(already_available) = from_str(&bytes) {
                tracing::error!("path:{:?}", &self.write_to_path);
                tracing::error!("name:{:?}", &self.override_file_name);
                if let Some(path) = &self.write_to_path {
                    if let Ok(response) = from_str::<FlexQueryResponse>(&bytes) {
                        let mut file_name: PathBuf = PathBuf::from(&path);
                        if let Some(file_name_override) = self.override_file_name.clone() {
                            file_name.push(&file_name_override);
                        } else {
                            file_name.push(format!(
                                "{}_{}_{}",
                                response.query_name,
                                response.flex_statements.statements[0].account_id,
                                response.flex_statements.statements[0].to_date
                            ));
                        }
                        file_name.set_extension("xml");

                        tracing::error!("{:?}", &file_name);
                        let mut f = File::create(file_name).await?;
                        f.write_all(bytes.as_bytes()).await?;
                    }
                }
                return Ok(already_available);
            }
            let response: core::result::Result<FlexQueryResponse, quick_xml::DeError> =
                from_str(&bytes);
            tracing::error!("{:#?}", response);
            if response.is_ok() {
                // match response.unwrap().status.unwrap_or_default().as_str() {
                // "Success" => {
                tracing::error!("path:{:?}", &self.write_to_path);
                tracing::error!("name:{:?}", &self.override_file_name);
                if let Some(path) = &self.write_to_path {
                    if let Ok(response) = from_str::<FlexQueryResponse>(&bytes) {
                        let mut file_name: PathBuf = PathBuf::from(&path);
                        if let Some(file_name_override) = self.override_file_name.clone() {
                            file_name.push(&file_name_override);
                        } else {
                            file_name.push(format!(
                                "{}_{}_{}",
                                response.query_name,
                                response.flex_statements.statements[0].account_id,
                                response.flex_statements.statements[0].to_date
                            ));
                        }
                        file_name.set_extension("xml");
                        let mut f = File::create(file_name).await?;
                        f.write_all(bytes.as_bytes()).await?;
                    }
                }
                return Ok(from_str(&bytes)?);
                // },
                // "Warn" => {},
                // _ =>
                // else {
                // return Err(Box::new(FlexError::SetFailed(
                // "error in
                // query"
                // .into(),
                // )))
                // },
                // }
            }
            if retries > 0 {
                tracing::info!("Flex not ready yet. Waiting {} sec...", wait);
                retries -= 1;
                tokio::time::sleep(std::time::Duration::from_secs(wait)).await;
                wait *= 2;
            } else {
                panic!(
                    "Still couldn't request flex statement after {} retries",
                    MAX_RETRIES
                );
            }
        }
    }

    pub async fn fetch_flex_statement(
        &self,
        token: String,
        query_id: String,
    ) -> Result<FlexStatement> {
        let reference_code = self
            .enqueue_flex_statement_request(token.clone(), query_id)
            .await?;
        tracing::error!("got reference code: {}", &reference_code);
        self.get_flex_statement(&reference_code, &token).await
        // let response: FlexStatementGetResponse = from_str(&flex_response)?;

        // Ok(response
        //    .flex_statements
        //    .statements
        //    .into_iter()
        //    .next()
        //    .unwrap())
    }

    pub async fn download_flex_statement(
        &self,
        _token: String,
        _query_id: String,
        write_to_path: PathBuf,
    ) -> Result<PathBuf> {
        Ok(write_to_path)
    }

    /// Returns statement reference code
    /// Cache this for a day so we avoid re-queueing flex statement requests
    async fn enqueue_flex_statement_request(
        &self,
        token: String,
        query_id: String,
    ) -> Result<String> {
        let url = format!("https://ndcdyn.interactivebrokers.com/AccountManagement/FlexWebService/SendRequest?t={}&q={}&v=3", token, query_id);
        let response: FlexStatementRequestResponse = self.retried_request(&url).await?;
        Ok(response.reference_code)
    }

    async fn get_flex_statement(&self, reference_code: &str, token: &str) -> Result<FlexStatement> {
        let url = format!("https://gdcdyn.interactivebrokers.com/AccountManagement/FlexWebService/GetStatement?q={}&t={}&v=3", reference_code, token);
        tracing::error!("Getting fetch statement using {}", &url);
        let response: FlexQueryResponse = self.retried_request(&url).await?;
        Ok(response
            .flex_statements
            .statements
            .into_iter()
            .next()
            .unwrap())
    }
}
pub async fn flex_statement_from_file(path: PathBuf) -> Result<FlexStatement> {
    if !path.is_file() {
        return Err(Box::new(FlexError::SetFailed(format!(
            "File {} does not exist",
            path.to_string_lossy()
        ))));
    }
    let contents = tokio::fs::read_to_string(path.as_path()).await?;

    let response: FlexQueryResponse = from_str(&contents)?;
    Ok(response
        .flex_statements
        .statements
        .into_iter()
        .next()
        .unwrap())
}
#[cfg(test)]
mod tests {
    use std::{env, path::PathBuf, str::FromStr};

    use chrono::{NaiveDate, NaiveDateTime};
    use iso_currency::Currency;
    // use ibkr_rust_api::{
    //    ib_enums::{self, SecIdType},
    //    prelude::Contract,
    //};
    use pretty_assertions::assert_eq;
    use quick_xml::de::from_str;
    use rust_decimal::Decimal;
    use rust_decimal_macros::dec;
    use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
    use tracing_log::LogTracer;
    use tracing_subscriber::{layer::SubscriberExt, registry::Registry, EnvFilter};

    use crate::{enums::*,
                flex_statement::{contract::Contract,
                                 trades::{Trade, TradeElements},
                                 FlexStatementRequestResponse},
                FlexQueryResponse,
                FlexReader};

    #[ctor::ctor]
    fn init() {
        LogTracer::init().expect("Unable to setup log tracer!");
        let env_filter =
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("debug"));
        let app_name = concat!(env!("CARGO_PKG_NAME"), "-", env!("CARGO_PKG_VERSION")).to_string();
        ////let (non_blocking_writer, _guard) =
        ////let tracing_appender::non_blocking(std::io::stdout());
        // let bunyan_formatting_layer = BunyanFormattingLayer::new(app_name,
        // std::io::stdout);//non_blocking_writer);
        let formatting_layer = BunyanFormattingLayer::new(app_name, std::io::stdout);
        let subscriber = Registry::default()
            .with(env_filter)
            .with(JsonStorageLayer)
            .with(formatting_layer);
        tracing::subscriber::set_global_default(subscriber).unwrap();
    }
    #[test]
    fn deserialize_flex_response() {
        let xml = r#"<FlexStatementResponse timestamp='26 August, 2023 12:36 PM EDT'>
<Status>Success</Status>
<ReferenceCode>9324443625</ReferenceCode>
<Url>https://gdcdyn.interactivebrokers.com/AccountManagement/FlexWebService/GetStatement</Url>
</FlexStatementResponse>
"#;
        let response: FlexStatementRequestResponse = from_str(xml).unwrap();
        assert_eq!(response.reference_code, "9324443625");
    }
    #[test]
    fn flex_deserialize_flex_query() {
        let xml = r#"
<FlexQueryResponse queryName="Trading EOD" type="AF">
<FlexStatements count="1">
<FlexStatement accountId="U7502027" fromDate="2022-02-15" toDate="2022-02-15" period="LastBusinessDay" whenGenerated="2022-02-1613:44:35">
<StmtFunds>
<StatementOfFundsLine accountId="U7502027" acctAlias="" model="" currency="EUR" fxRateToBase="1" assetCategory="" symbol="" description="" conid="" securityID="" securityIDType="" cusip="" isin="" listingExchange="" underlyingConid="" underlyingSymbol="" underlyingSecurityID="" underlyingListingExchange="" issuer="" multiplier="" strike="" expiry="" putCall="" principalAdjustFactor="" reportDate="2022-02-15" date="2022-02-15" settleDate="" activityCode="" activityDescription="Starting Balance" tradeID="" orderID="" buySell="" tradeQuantity="0" tradePrice="0" tradeGross="0" tradeCommission="0" tradeTax="0" debit="" credit="" amount="0" tradeCode="" balance="6655.368396105" levelOfDetail="BaseCurrency" transactionID="" serialNumber="" deliveryType="" commodityType="" fineness="" weight="" />
</StmtFunds>
<CommissionCredits>
</CommissionCredits>
<Trades>
<Trade accountId="U7502027" acctAlias="" model="" currency="USD" fxRateToBase="0.88043" assetCategory="STK" symbol="AMD" description="ADVANCED MICRO DEVICES" conid="4391" securityID="US0079031078" securityIDType="ISIN" cusip="007903107" isin="US0079031078" listingExchange="NASDAQ" underlyingConid="" underlyingSymbol="" underlyingSecurityID="" underlyingListingExchange="" issuer="" multiplier="1" strike="" expiry="" tradeID="337365179" putCall="" reportDate="2022-02-15" principalAdjustFactor="" dateTime="2022-02-1510:20:26" tradeDate="2022-02-15" settleDateTarget="2022-02-17" transactionType="ExchTrade" exchange="ISLAND" quantity="-1" tradePrice="115.73" tradeMoney="-115.73" proceeds="115.73" taxes="0" ibCommission="-1.000720223" ibCommissionCurrency="USD" netCash="114.729279777" closePrice="121.47" openCloseIndicator="O" notes="P" cost="-114.729279777" fifoPnlRealized="0" fxPnl="0" mtmPnl="-5.74" origTradePrice="0" origTradeDate="" origTradeID="" origOrderID="0" clearingFirmID="" transactionID="932769321" buySell="SELL" ibOrderID="300028496" ibExecID="0000febb.620b9507.01.01" brokerageOrderID="0004f96a.00014c43.620b64e8.0001" orderReference="ChartTrader428018967" volatilityOrderLink="1413113351.0" exchOrderId="N/A" extExecID="0322420512" orderTime="2022-02-1510:20:24" openDateTime="" holdingPeriodDateTime="" whenRealized="" whenReopened="" levelOfDetail="EXECUTION" changeInPrice="0" changeInQuantity="0" orderType="LMT" traderID="" isAPIOrder="N" accruedInt="0" serialNumber="" deliveryType="" commodityType="" fineness="0.0" weight="0.0 ()" />
</Trades><TransactionTaxes>
</TransactionTaxes><RoutingCommissions>
</RoutingCommissions><UnbundledCommissionDetails>
<UnbundledCommissionDetail accountId="U7502027" acctAlias="" model="" currency="USD" fxRateToBase="0.88043" assetCategory="STK" symbol="AMD" description="ADVANCED MICRO DEVICES" conid="4391" securityID="US0079031078" securityIDType="ISIN" cusip="007903107" isin="US0079031078" listingExchange="NASDAQ" underlyingConid="" underlyingSymbol="" underlyingSecurityID="" underlyingListingExchange="" issuer="" multiplier="1" strike="" expiry="" putCall="" principalAdjustFactor="" dateTime="2022-02-1510:20:26" exchange="ISLAND" buySell="SELL" quantity="-1" price="115.73" tradeID="337365179" orderReference="ChartTrader428018967" totalCommission="-1.000720223" brokerExecutionCharge="-1" brokerClearingCharge="0" thirdPartyExecutionCharge="0" thirdPartyClearingCharge="0" thirdPartyRegulatoryCharge="-0.00072" regFINRATradingActivityFee="-0.00013" regSection31TransactionFee="-0.00059" regOther="0" other="0" serialNumber="" deliveryType="" commodityType="" fineness="0.0" weight="0.0 ()" />
</UnbundledCommissionDetails>
<InterestAccruals><InterestAccrualsCurrency accountId="U7502027" acctAlias="" model="" currency="BASE_SUMMARY" fromDate="2023-08-29" toDate="2023-08-29" startingAccrualBalance="46.41" interestAccrued="0.31" accrualReversal="0" fxTranslation="0" endingAccrualBalance="46.72"/>
</InterestAccruals>
<HardToBorrowDetails>
</HardToBorrowDetails><SLBFees>
</SLBFees>
</FlexStatement>
</FlexStatements>
</FlexQueryResponse>
"#;
        let response: FlexQueryResponse = from_str(xml).unwrap();
        assert_eq!(response.query_name, "Trading EOD");
        assert_eq!(response.query_type, "AF");
        let statement = &response.flex_statements.statements[0];
        assert_eq!(statement.account_id, "U7502027");
        assert_eq!(
            statement.from_date,
            NaiveDate::from_str("2022-02-15").unwrap()
        );
        assert_eq!(
            statement.to_date,
            NaiveDate::from_str("2022-02-15").unwrap()
        );
        if let TradeElements::Trade(trade_1) = &statement.trades.as_ref().unwrap().items[0] {
            assert_eq!(
                &Trade {
                    account_id:               "U7502027".to_string(),
                    acct_alias:               None,
                    model:                    None,
                    contract:                 Contract {
                        asset_category:              AssetCategory::STK,
                        symbol:                      "AMD".to_string(),
                        description:                 "ADVANCED MICRO DEVICES".to_string(),
                        con_id:                      4391,
                        security_id:                 Some("US0079031078".to_string()),
                        security_id_type:            Some(SecIdType::Isin),
                        cusip:                       Some("007903107".to_string()),
                        isin:                        Some("US0079031078".to_string()),
                        listing_exchange:            "NASDAQ".to_string(),
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
                    currency:                 Currency::USD,
                    fx_rate_to_base:          Some(dec!(0.88043)),
                    transaction_type:         "ExchTrade".to_string(),
                    trade_id:                 Some(337365179),
                    ib_order_id:              Some(300028496),
                    ib_exec_id:               Some("0000febb.620b9507.01.01".to_string()),
                    brokerage_order_id:       Some("0004f96a.00014c43.620b64e8.0001".to_string()),
                    order_reference:          Some("ChartTrader428018967".to_string()),
                    volatility_order_link:    Some("1413113351.0".to_string()),
                    clearing_firm_id:         None,
                    orig_trade_price:         Some(dec!(0)),
                    orig_trade_id:            None,
                    order_time:               Some(
                        NaiveDateTime::from_str("2022-02-15T10:20:24").unwrap()
                    ),
                    open_date_time:           None,
                    trade_date_time:          NaiveDateTime::from_str("2022-02-15T10:20:26")
                        .unwrap(),
                    report_date:              NaiveDate::from_str("2022-02-15").ok(),
                    settle_date_target:       NaiveDate::from_str("2022-02-17").ok(),
                    trade_date:               NaiveDate::from_str("2022-02-15").unwrap(),
                    exchange:                 "ISLAND".to_string(),
                    transaction_id:           "932769321".to_string(),
                    buy_sell:                 crate::enums::BuySell::Sell,
                    quantity:                 dec!(-1),
                    trade_price:              dec!(115.73),
                    trade_money:              dec!(-115.73),
                    proceeds:                 dec!(115.73),
                    ib_commission:            dec!(-1.000720223),
                    exch_order_id:            Some("N/A".to_string()),
                    ext_exec_id:              Some("0322420512".to_string()),
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
                    ib_commission_currency:   Some(Currency::USD),
                    net_cash:                 dec!(114.729279777),
                    close_price:              dec!(121.47),
                    open_close_indicator:     Some(OpenClose::O),
                    notes:                    vec![Notes::PartialExecution],
                    cost:                     dec!(-114.729279777),
                    fifo_pnl_realized:        dec!(0),
                    fx_pnl:                   Some(dec!(0)),
                    mtm_pnl:                  Some(dec!(-5.74)),
                    orig_order_id:            Some(0),
                },
                trade_1
            );
        }
        // let funds = StatementOfFundsLine{account_id:"U7502027".to_string()}
    }
    #[test]
    fn flex_deserialize_empty_response_ok() {
        let xml = r#"
<FlexQueryResponse queryName="Trading EOD" type="AF">
<FlexStatements count="1">
<FlexStatement accountId="U7502027" fromDate="2022-02-21" toDate="2022-02-21" period="LastBusinessDay" whenGenerated="2022-02-2204:07:52">
<StmtFunds>
</StmtFunds>
<CommissionCredits> </CommissionCredits>
<Trades> </Trades>
<TransactionTaxes> </TransactionTaxes>
<RoutingCommissions> </RoutingCommissions>
<UnbundledCommissionDetails> </UnbundledCommissionDetails>
<InterestAccruals> </InterestAccruals>
<HardToBorrowDetails> </HardToBorrowDetails>
<SLBFees> </SLBFees>
</FlexStatement>
</FlexStatements>
</FlexQueryResponse>
"#;
        let response: FlexQueryResponse = from_str(xml).unwrap();
        assert_eq!(response.query_name, "Trading EOD");
        assert_eq!(response.query_type, "AF");
        let statement = &response.flex_statements.statements[0];
        assert_eq!(statement.account_id, "U7502027");
        assert_eq!(
            statement.from_date,
            NaiveDate::from_str("2022-02-21").unwrap()
        );
        assert_eq!(
            statement.to_date,
            NaiveDate::from_str("2022-02-21").unwrap()
        );
    }

    #[test]
    fn flex_deserialize_response_ok() {
        let xml = r#"
<FlexQueryResponse queryName="Trading EOD" type="AF">
<FlexStatements count="1">
<FlexStatement accountId="U7502027" fromDate="2023-07-25" toDate="2023-07-25" period="LastBusinessDay" whenGenerated="2023-07-2618:00:42">
<StmtFunds>
<StatementOfFundsLine accountId="U7502027" acctAlias="" model="" currency="USD" fxRateToBase="1" assetCategory="" symbol="" description="" conid="" securityID="" securityIDType="" cusip="" isin="" listingExchange="" underlyingConid="" underlyingSymbol="" underlyingSecurityID="" underlyingListingExchange="" issuer="" multiplier="" strike="" expiry="" putCall="" principalAdjustFactor="" reportDate="2023-07-25" date="2023-07-25" settleDate="" activityCode="" activityDescription="Starting Balance" tradeID="" orderID="" buySell="" tradeQuantity="0" tradePrice="0" tradeGross="0" tradeCommission="0" tradeTax="0" debit="" credit="" amount="0" tradeCode="" balance="21.483433036" levelOfDetail="BaseCurrency" transactionID="" serialNumber="" deliveryType="" commodityType="" fineness="" weight="" />
<StatementOfFundsLine accountId="U7502027" acctAlias="" model="" currency="USD" fxRateToBase="1" assetCategory="" symbol="" description="" conid="" securityID="" securityIDType="" cusip="" isin="" listingExchange="" underlyingConid="" underlyingSymbol="" underlyingSecurityID="" underlyingListingExchange="" issuer="" multiplier="" strike="" expiry="" putCall="" principalAdjustFactor="" reportDate="2023-07-25" date="2023-07-25" settleDate="" activityCode="ADJ" activityDescription="FX Translations P&amp;L" tradeID="" orderID="" buySell="" tradeQuantity="0" tradePrice="0" tradeGross="0" tradeCommission="0" tradeTax="0" debit="-0.064064" credit="" amount="-0.064064" tradeCode="" balance="21.419369036" levelOfDetail="BaseCurrency" transactionID="" serialNumber="" deliveryType="" commodityType="" fineness="" weight="" />
<StatementOfFundsLine accountId="U7502027" acctAlias="" model="" currency="USD" fxRateToBase="1" assetCategory="" symbol="" description="" conid="" securityID="" securityIDType="" cusip="" isin="" listingExchange="" underlyingConid="" underlyingSymbol="" underlyingSecurityID="" underlyingListingExchange="" issuer="" multiplier="" strike="" expiry="" putCall="" principalAdjustFactor="" reportDate="2023-07-25" date="2023-07-25" settleDate="" activityCode="" activityDescription="Ending Balance" tradeID="" orderID="" buySell="" tradeQuantity="0" tradePrice="0" tradeGross="0" tradeCommission="0" tradeTax="0" debit="" credit="" amount="0" tradeCode="" balance="21.419369036" levelOfDetail="BaseCurrency" transactionID="" serialNumber="" deliveryType="" commodityType="" fineness="" weight="" 
/<StatementOfFundsLine accountId="U7502027" acctAlias="" model="" currency="USD" fxRateToBase="1" assetCategory="STK" symbol="TSLA" description="TESLA INC" conid="76792991" securityID="US88160R1014" securityIDType="ISIN" cusip="88160R101" isin="US88160R1014" listingExchange="NASDAQ" underlyingConid="" underlyingSymbol="" underlyingSecurityID="" underlyingListingExchange="" issuer="" multiplier="1" strike="" expiry="" putCall="" principalAdjustFactor="" reportDate="2023-08-29" date="2023-08-29" settleDate="2023-08-31" activityCode="BUY" activityDescription="Buy 22 TESLA INC " tradeID="596697748" orderID="507018808" buySell="BUY" tradeQuantity="22" tradePrice="240.4" tradeGross="-5288.8" tradeCommission="-0.33705725" tradeTax="0" debit="-5289.13705725" credit="" amount="-5289.13705725" tradeCode="P" balance="-5563.302634411" levelOfDetail="BaseCurrency" transactionID="2101185924" serialNumber="" deliveryType="" commodityType="" fineness="0.0" weight="0.0 ()"/>>
</StmtFunds>
<CommissionCredits>
</CommissionCredits>
<Trades>
<Trade accountId="U7502027" acctAlias="" model="" currency="USD" fxRateToBase="1" assetCategory="STK" symbol="GOOGL" description="ALPHABET INC-CL A" conid="208813719" securityID="US02079K3059" securityIDType="ISIN" cusip="02079K305" isin="US02079K3059" listingExchange="NASDAQ" underlyingConid="" underlyingSymbol="" underlyingSecurityID="" underlyingListingExchange="" issuer="" multiplier="1" strike="" expiry="" tradeID="596715684" putCall="" reportDate="2023-08-29" principalAdjustFactor="" dateTime="2023-08-2909:45:54" tradeDate="2023-08-29" settleDateTarget="2023-08-31" transactionType="ExchTrade" exchange="DRCTEDGE" quantity="60" tradePrice="132.75" tradeMoney="7965" proceeds="-7965" taxes="0" ibCommission="-0.31425725" ibCommissionCurrency="USD" netCash="-7965.31425725" closePrice="134.57" openCloseIndicator="O" notes="" cost="7965.31425725" fifoPnlRealized="0" fxPnl="0" mtmPnl="109.2" origTradePrice="0" origTradeDate="" origTradeID="" origOrderID="0" clearingFirmID="" transactionID="2101225667" buySell="BUY" ibOrderID="507031561" ibExecID="0000f62c.64ed9adf.01.01" brokerageOrderID="0004f96a.00014c43.64ed853a.0001" orderReference="" volatilityOrderLink="2083705967.0" exchOrderId="N/A" extExecID="940370002911B" orderTime="2023-08-2909:45:54" openDateTime="" holdingPeriodDateTime="" whenRealized="" whenReopened="" levelOfDetail="EXECUTION" changeInPrice="0" changeInQuantity="0" orderType="MIDPX" traderID="" isAPIOrder="N" accruedInt="0" serialNumber="" deliveryType="" commodityType="" fineness="0.0" weight="0.0 ()"/>
</Trades><TransactionTaxes>
</TransactionTaxes><RoutingCommissions>
</RoutingCommissions><UnbundledCommissionDetails>
</UnbundledCommissionDetails><InterestAccruals>
<InterestAccrualsCurrency accountId="U7502027" acctAlias="" model="" currency="BASE_SUMMARY" fromDate="2023-07-25" toDate="2023-07-25" startingAccrualBalance="35.66" interestAccrued="0.3" accrualReversal="0" fxTranslation="0" endingAccrualBalance="35.96" />
</InterestAccruals>
<HardToBorrowDetails>
</HardToBorrowDetails><SLBFees>
</SLBFees>
</FlexStatement>
</FlexStatements>
</FlexQueryResponse>
        "#;
        let response: FlexQueryResponse = from_str(xml).unwrap();
        assert_eq!(response.query_name, "Trading EOD");
        assert_eq!(response.query_type, "AF");
        let statement = &response.flex_statements.statements[0];
        assert_eq!(statement.account_id, "U7502027");
        assert_eq!(
            statement.from_date,
            NaiveDate::from_str("2023-07-25").unwrap()
        );
        assert_eq!(
            statement.to_date,
            NaiveDate::from_str("2023-07-25").unwrap()
        );
    }
    #[tokio::test]
    async fn flex_download_query() {
        let token = "119588012295865351751026".to_string();
        let query_id = "639991".to_string();

        // let client = reqwest::Client::new();
        // let url = "https://ndcdyn.interactivebrokers.com/AccountManagement/FlexWebService/SendRequest?t=119588012295865351751026&q=639991&v=3";
        // let url = "https://www.google.com";
        // let response = reqwest::get(url).await;
        // let response = client
        //     .get(url)
        //     .header(
        //         USER_AGENT,
        //         "Mozilla/5.0 (Windows NT 6.1; WOW64) AppleWebKit/537.36 (KHTML, like
        // Gecko) \          Chrome/56.0.2924.76 Safari/537.36",
        //     )
        //     .send()
        //     .await;
        // tracing::error!(
        //     "Body: {:?}",
        //     response.unwrap().text().await.unwrap().to_string()
        // );

        let reader = FlexReader {
            write_to_path:      Some(PathBuf::from(r"/home/czichy/tmp/")),
            override_file_name: None,
        };
        let _statement = reader.fetch_flex_statement(token, query_id).await;
        tracing::error!("Statement: {:#?}", _statement);
    }
}
