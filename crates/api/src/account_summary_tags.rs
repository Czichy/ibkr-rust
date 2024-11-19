//! Account summary tags
use strum_macros::{Display, EnumString};
//==================================================================================================
/// AccountType — Identifies the IB account structure
/// NetLiquidation — The basis for determining the price of the assets in your
/// account. Total cash value + stock value + options value + bond value
/// TotalCashValue — Total cash balance recognized at the time of trade +
/// futures PNL SettledCash — Cash recognized at the time of settlement -
/// purchases at the time of trade - commissions - taxes - fees AccruedCash —
/// Total accrued cash value of stock, commodities and securities BuyingPower —
/// Buying power serves as a measurement of the dollar value of securities that
/// one may purchase in a securities account without depositing additional funds
/// EquityWithLoanValue — Forms the basis for determining whether a client has
/// the necessary assets to either initiate or maintain security positions. Cash
/// + stocks + bonds + mutual funds PreviousEquityWithLoanValue — Marginable
/// Equity with Loan value as of 16:00 ET the previous day GrossPositionValue —
/// The sum of the absolute value of all stock and equity option positions
/// RegTEquity — Regulation T equity for universal account
/// RegTMargin — Regulation T margin for universal account
/// SMA — Special Memorandum Account: Line of credit created when the market
/// value of securities in a Regulation T account increase in value
/// InitMarginReq — Initial Margin requirement of whole portfolio
/// MaintMarginReq — Maintenance Margin requirement of whole portfolio
/// AvailableFunds — This value tells what you have available for trading
/// ExcessLiquidity — This value shows your margin cushion, before liquidation
/// Cushion — Excess liquidity as a percentage of net liquidation value
/// FullInitMarginReq — Initial Margin of whole portfolio with no discounts or
/// intraday credits FullMaintMarginReq — Maintenance Margin of whole portfolio
/// with no discounts or intraday credits FullAvailableFunds — Available funds
/// of whole portfolio with no discounts or intraday credits FullExcessLiquidity
/// — Excess liquidity of whole portfolio with no discounts or intraday credits
/// LookAheadNextChange — Time when look-ahead values take effect
/// LookAheadInitMarginReq — Initial Margin requirement of whole portfolio as of
/// next period's margin change LookAheadMaintMarginReq — Maintenance Margin
/// requirement of whole portfolio as of next period's margin change
/// LookAheadAvailableFunds — This value reflects your available funds at the
/// next margin change LookAheadExcessLiquidity — This value reflects your
/// excess liquidity at the next margin change HighestSeverity — A measure of
/// how close the account is to liquidation DayTradesRemaining — The Number of
/// Open/Close trades a user could put on before Pattern Day Trading is
/// detected. A value of "-1" means that the user can put on unlimited day
/// trades. Leverage — GrossPositionValue / NetLiquidation
/// $LEDGER — Single flag to relay all cash balance tags*, only in base
/// currency. $LEDGER:CURRENCY — Single flag to relay all cash balance tags*,
/// only in the specified currency. $LEDGER:ALL — Single flag to relay all cash
/// balance tags* in all currencies.
#[derive(Display, Debug, PartialEq, Eq, EnumString, Clone, Copy)]
pub enum AccountSummaryTags {
    AccountType,
    NetLiquidation,
    TotalCashValue,
    SettledCash,
    AccruedCash,
    BuyingPower,
    EquityWithLoanValue,
    PreviousEquityWithLoanValue,
    GrossPositionValue,
    ReqTEquity,
    ReqTMargin,
    SMA,
    InitMarginReq,
    MaintMarginReq,
    AvailableFunds,
    ExcessLiquidity,
    Cushion,
    FullInitMarginReq,
    FullMaintMarginReq,
    FullAvailableFunds,
    FullExcessLiquidity,
    LookAheadNextChange,
    LookAheadInitMarginReq,
    LookAheadMaintMarginReq,
    LookAheadAvailableFunds,
    LookAheadExcessLiquidity,
    HighestSeverity,
    DayTradesRemaining,
    Leverage,
    Ledger,
    LedgerCurrency,
    LedgerAll,
    AllTags,
}
#[derive(Display, Debug, PartialEq, EnumString, Clone, Hash, Eq)]
pub enum AccountValueKey {
    /// The account ID number
    AccountCode,
    /// All to return account summary data for all accounts, or set to a
    /// specific Advisor Account Group name that has already been created in TWS
    /// Global Configuration
    AccountOrGroup,
    /// For internal use only
    AccountReady,
    /// Identifies the IB account structure
    AccountType,
    /// Total accrued cash value of stock, commodities and securities
    AccruedCash,
    /// Reflects the current's month accrued debit and credit interest to date,
    /// updated daily in commodity segment
    #[strum(serialize = "AccruedCash-C")]
    AccruedCashC,
    /// Reflects the current's month accrued debit and credit interest to date,
    /// updated daily in security segment
    #[strum(serialize = "AccruedCash-S")]
    AccruedCashS,
    /// Total portfolio value of dividends accrued
    AccruedDividend,
    /// Dividends accrued but not paid in commodity segment
    #[strum(serialize = "AccruedDividend-C")]
    AccruedDividendC,
    /// Dividends accrued but not paid in security segment
    #[strum(serialize = "AccruedDividend-S")]
    AccruedDividendS,
    /// This value tells what you have available for trading
    AvailableFunds,
    /// Net Liquidation Value - Initial Margin
    #[strum(serialize = "AvailableFunds-C")]
    AvailableFundsC,
    /// Equity with Loan Value - Initial Margin
    #[strum(serialize = "AvailableFunds-S")]
    AvailableFundsS,
    /// Total portfolio value of treasury bills
    Billable,
    /// Value of treasury bills in commodity segment
    #[strum(serialize = "Billable-C")]
    BillableC,
    /// Value of treasury bills in security segment
    #[strum(serialize = "Billable-S")]
    BillableS,
    /// Cash Account: Minimum (Equity with Loan Value, Previous Day Equity with
    /// Loan Value)-Initial Margin, Standard Margin Account: Minimum (Equity
    /// with Loan Value, Previous Day Equity with Loan Value) - Initial Margin
    /// *4
    BuyingPower,
    /// Cash recognized at the time of trade + futures PNL
    CashBalance,
    ///
    #[strum(serialize = "ColumnPrio-S")]
    ColumnPrioS,
    /// Value of non-Government bonds such as corporate bonds and municipal
    /// bonds
    CorporateBondValue,
    /// Open positions are grouped by currency
    Currency,
    /// Excess liquidity as a percentage of net liquidation value
    Cushion,
    /// Number of Open/Close trades one could do before Pattern Day Trading is
    /// detected
    DayTradesRemaining,
    /// Number of Open/Close trades one could do tomorrow before Pattern Day
    /// Trading is detected
    #[strum(serialize = "DayTradesRemainingT+1")]
    DayTradesRemainingT1,
    /// Number of Open/Close trades one could do two days from today before
    /// Pattern Day Trading is detected
    #[strum(serialize = "DayTradesRemainingT+2")]
    DayTradesRemainingT2,
    /// Number of Open/Close trades one could do three days from today before
    /// Pattern Day Trading is detected
    #[strum(serialize = "DayTradesRemainingT+3")]
    DayTradesRemainingT3,
    /// Number of Open/Close trades one could do four days from today before
    /// Pattern Day Trading is detected
    #[strum(serialize = "DayTradesRemainingT+4")]
    DayTradesRemainingT4,
    /// Forms the basis for determining whether a client has the necessary
    /// assets to either initiate or maintain security positions
    EquityWithLoanValue,
    /// Cash account: Total cash value + commodities option value - futures
    /// maintenance margin requirement + minimum (0, futures PNL) Margin
    /// account: Total cash value + commodities option value - futures
    /// maintenance margin requirement
    #[strum(serialize = "EquityWithLoanValue-C")]
    EquityWithLoanValueC,
    /// Cash account: Settled Cash Margin Account: Total cash value + stock
    /// value + bond value + (non-U.S. & Canada securities options value)
    #[strum(serialize = "EquityWithLoanValue-S")]
    EquityWithLoanValueS,
    /// This value shows your margin cushion, before liquidation
    ExcessLiquidity,
    /// Equity with Loan Value - Maintenance Margin
    #[strum(serialize = "ExcessLiquidity-C")]
    ExcessLiquidityC,
    /// Net Liquidation Value - Maintenance Margin
    #[strum(serialize = "ExcessLiquidity-S")]
    ExcessLiquidityS,
    /// The exchange rate of the currency to your base currency
    ExchangeRate,
    /// Available funds of whole portfolio with no discounts or intraday credits
    FullAvailableFunds,
    /// Net Liquidation Value - Full Initial Margin
    #[strum(serialize = "FullAvailableFunds-C")]
    FullAvailableFundsC,
    /// Equity with Loan Value - Full Initial Margin
    #[strum(serialize = "FullAvailableFunds-S")]
    FullAvailableFundsS,
    /// Excess liquidity of whole portfolio with no discounts or intraday
    /// credits
    FullExcessLiquidity,
    /// Net Liquidation Value - Full Maintenance Margin
    #[strum(serialize = "FullExcessLiquidity-C")]
    FullExcessLiquidityC,
    /// Equity with Loan Value - Full Maintenance Margin
    #[strum(serialize = "FullExcessLiquidity-S")]
    FullExcessLiquidityS,
    /// Initial Margin of whole portfolio with no discounts or intraday credits
    FullInitMarginReq,
    /// Initial Margin of commodity segment's portfolio with no discounts or
    /// intraday credits
    #[strum(serialize = "FullInitMarginReq-C")]
    FullInitMarginReqC,
    /// Initial Margin of security segment's portfolio with no discounts or
    /// intraday credits
    #[strum(serialize = "FullInitMarginReq-S")]
    FullInitMarginReqS,
    /// Maintenance Margin of whole portfolio with no discounts or intraday
    /// credits
    FullMaintMarginReq,
    /// Maintenance Margin of commodity segment's portfolio with no discounts or
    /// intraday credits
    #[strum(serialize = "FullMaintMarginReq-C")]
    FullMaintMarginReqC,
    /// Maintenance Margin of security segment's portfolio with no discounts or
    /// intraday credits
    #[strum(serialize = "FullMaintMarginReq-S")]
    FullMaintMarginReqS,
    /// Value of funds value (money market funds + mutual funds)
    FundValue,
    /// Real-time market-to-market value of futures options
    FutureOptionValue,
    /// Real-time changes in futures value since last settlement
    FuturesPNL,
    /// Cash balance in related IB-UKL account
    FxCashBalance,
    /// Gross Position Value in securities segment
    GrossPositionValue,
    /// Long Stock Value + Short Stock Value + Long Option Value + Short Option
    /// Value
    #[strum(serialize = "GrossPositionValue-S")]
    GrossPositionValueS,
    ///
    Guarantee,
    /// Margin rule for IB-IN accounts
    IndianStockHaircut,
    /// Initial Margin requirement of whole portfolio
    InitMarginReq,
    /// Initial Margin of the commodity segment in base currency
    #[strum(serialize = "InitMarginReq-C")]
    InitMarginReqC,
    /// Initial Margin of the security segment in base currency
    #[strum(serialize = "InitMarginReq-S")]
    InitMarginReqS,
    /// Real-time mark-to-market value of Issued Option
    IssuerOptionValue,
    /// GrossPositionValue / NetLiquidation in security segment
    #[strum(serialize = "Leverage-S")]
    LeverageS,
    /// Time when look-ahead values take effect
    LookAheadNextChange,
    /// This value reflects your available funds at the next margin change
    LookAheadAvailableFunds,
    /// Net Liquidation Value - look ahead Initial Margin
    #[strum(serialize = "LookAheadAvailableFunds-C")]
    LookAheadAvailableFundsC,
    /// Equity with Loan Value - look ahead Initial Margin
    #[strum(serialize = "LookAheadAvailableFunds-S")]
    LookAheadAvailableFundsS,
    /// This value reflects your excess liquidity at the next margin change
    LookAheadExcessLiquidity,
    /// Net Liquidation Value - look ahead Maintenance Margin
    #[strum(serialize = "LookAheadExcessLiquidity-C")]
    LookAheadExcessLiquidityC,
    /// Equity with Loan Value - look ahead Maintenance Margin
    #[strum(serialize = "LookAheadExcessLiquidity-S")]
    LookAheadExcessLiquidityS,
    /// Initial margin requirement of whole portfolio as of next period's margin
    /// change
    LookAheadInitMarginReq,
    /// Initial margin requirement as of next period's margin change in the base
    /// currency of the account
    #[strum(serialize = "LookAheadInitMarginReq-C")]
    LookAheadInitMarginReqC,
    /// Initial margin requirement as of next period's margin change in the base
    /// currency of the account
    #[strum(serialize = "LookAheadInitMarginReq-S")]
    LookAheadInitMarginReqS,
    /// Maintenance margin requirement of whole portfolio as of next period's
    /// margin change
    LookAheadMaintMarginReq,
    /// Maintenance margin requirement as of next period's margin change in the
    /// base currency of the account
    #[strum(serialize = "LookAheadMaintMarginReq-C")]
    LookAheadMaintMarginReqC,
    /// Maintenance margin requirement as of next period's margin change in the
    /// base currency of the account
    #[strum(serialize = "LookAheadMaintMarginReq-S")]
    LookAheadMaintMarginReqS,
    /// Maintenance Margin requirement of whole portfolio
    MaintMarginReq,
    /// Maintenance Margin for the commodity segment
    #[strum(serialize = "MaintMarginReq-C")]
    MaintMarginReqC,
    /// Maintenance Margin for the security segment
    #[strum(serialize = "MaintMarginReq-S")]
    MaintMarginReqS,
    /// Market value of money market funds excluding mutual funds
    MoneyMarketFundValue,
    /// Market value of mutual funds excluding money market funds
    MutualFundValue,
    /// The sum of the Dividend Payable/Receivable Values for the securities and
    /// commodities segments of the account
    NetDividend,
    /// The basis for determining the price of the assets in your account
    NetLiquidation,
    /// Total cash value + futures PNL + commodities options value
    #[strum(serialize = "NetLiquidation-C")]
    NetLiquidationC,
    /// Total cash value + stock value + securities options value + bond value
    #[strum(serialize = "NetLiquidation-S")]
    NetLiquidationS,
    /// Net liquidation for individual currencies
    NetLiquidationByCurrency,
    ///
    NLVAndMarginInReview,
    /// Real-time mark-to-market value of options
    OptionMarketValue,
    /// Personal Account shares value of whole portfolio
    PASharesValue,
    /// Personal Account shares value in commodity segment
    #[strum(serialize = "PASharesValue-C")]
    PASharesValueC,
    /// Personal Account shares value in security segment
    #[strum(serialize = "PASharesValue-S")]
    PASharesValueS,
    /// Total projected at expiration excess liquidity
    PostExpirationExcess,
    /// Provides a projected at expiration excess liquidity based on the soon-to
    /// expire contracts in your portfolio in commodity segment
    #[strum(serialize = "PostExpirationExcess-C")]
    PostExpirationExcessC,
    /// Provides a projected at expiration excess liquidity based on the soon-to
    /// expire contracts in your portfolio in security segment
    #[strum(serialize = "PostExpirationExcess-S")]
    PostExpirationExcessS,
    /// Total projected at expiration margin
    PostExpirationMargin,
    /// Provides a projected at expiration margin value based on the soon-to
    /// expire contracts in your portfolio in commodity segment
    #[strum(serialize = "PostExpirationMargin-C")]
    PostExpirationMarginC,
    /// Provides a projected at expiration margin value based on the soon-to
    /// expire contracts in your portfolio in security segment
    #[strum(serialize = "PostExpirationMargin-S")]
    PostExpirationMarginS,
    /// Marginable Equity with Loan value as of 16:00 ET the previous day in
    /// securities segment
    PreviousDayEquityWithLoanValue,
    /// IMarginable Equity with Loan value as of 16:00 ET the previous day
    #[strum(serialize = "PreviousDayEquityWithLoanValue-S")]
    PreviousDayEquityWithLoanValueS,
    /// Open positions are grouped by currency
    RealCurrency,
    /// Shows your profit on closed positions, which is the difference between
    /// your entry execution cost and exit execution costs, or (execution price
    /// + commissions to open the positions) - (execution price + commissions to
    /// close the position)
    RealizedPnL,
    /// Regulation T equity for universal account
    RegTEquity,
    /// Regulation T equity for security segment
    #[strum(serialize = "RegTEquity-S")]
    RegTEquityS,
    /// Regulation T margin for universal account
    RegTMargin,
    /// Regulation T margin for security segment
    #[strum(serialize = "RegTMargin-S")]
    RegTMarginS,
    /// Line of credit created when the market value of securities in a
    /// Regulation T account increase in value
    SMA,
    /// Regulation T Special Memorandum Account balance for security segment
    #[strum(serialize = "SMA-S")]
    SMAS,
    /// Account segment name
    SegmentTitle,
    /// Real-time mark-to-market value of stock
    StockMarketValue,
    /// Value of treasury bonds
    TBondValue,
    /// Value of treasury bills
    TBillValue,
    /// Total Cash Balance including Future PNL
    TotalCashBalance,
    /// Total cash value of stock, commodities and securities
    TotalCashValue,
    /// CashBalance in commodity segment
    #[strum(serialize = "TotalCashValue-C")]
    TotalCashValueC,
    /// CashBalance in security segment
    #[strum(serialize = "TotalCashValue-S")]
    TotalCashValueS,
    /// Account Type
    #[strum(serialize = "TradingType-S")]
    TradingTypeS,
    /// The difference between the current market value of your open positions
    /// and the average cost, or Value - Average Cost
    UnrealizedPnL,
    /// Value of warrants
    WarrantValue,
    /// To check projected margin requirements under Portfolio Margin model
    WhatIfPMEnabled,
    ///
    Unknown(String),
}
