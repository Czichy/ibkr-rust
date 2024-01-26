use bytes::Bytes;

use crate::{
    enums::Outgoing,
    frame::Frame,
    utils::ib_message::{Encodable, IBMessage},
    RequestId,
};

const VERSION: i32 = 1;

///         * @brief Requests a specific account's summary.\n
///         * This method will subscribe to the account summary as presented in
///           the TWS' Account Summary tab. The data is returned at
///           EWrapper::accountSummary\n
///         * https://www.interactivebrokers.com/en/software/tws/accountwindowtop.htm
///         * @param reqId the unique request identifier.\n
///         * @param group set to "All" to return account summary data for all
///           accounts, or set to a specific Advisor Account Group name that has
///           already been created in TWS Global Configuration.\n
///         * @param tags a comma separated list with the desired tags:
///         * - AccountType — Identifies the IB account structure
///         * - NetLiquidation — The basis for determining the price of the
///           assets in your account. Total cash value + stock value + options
///           value + bond value
///         * - TotalCashValue — Total cash balance recognized at the time of
///           trade + futures PNL
///         * - SettledCash — Cash recognized at the time of settlement -
///           purchases at the time of trade - commissions - taxes - fees
///         * - AccruedCash — Total accrued cash value of stock, commodities and
///           securities
///         * - BuyingPower — Buying power serves as a measurement of the dollar
///           value of securities that one may purchase in a securities account
///           without depositing additional funds
///         * - EquityWithLoanValue — Forms the basis for determining whether a
///           client has the necessary assets to either initiate or maintain
///           security positions. Cash + stocks + bonds + mutual funds
///         * - PreviousEquityWithLoanValue — Marginable Equity with Loan value
///           as of 16:00 ET the previous day
///         * - GrossPositionValue — The sum of the absolute value of all stock
///           and equity option positions
///         * - RegTEquity — Regulation T equity for universal account
///         * - RegTMargin — Regulation T margin for universal account
///         * - SMA — Special Memorandum Account: Line of credit created when
///           the market value of securities in a Regulation T account increase
///           in value
///         * - InitMarginReq — Initial Margin requirement of whole portfolio
///         * - MaintMarginReq — Maintenance Margin requirement of whole
///           portfolio
///         * - AvailableFunds — This value tells what you have available for
///           trading
///         * - ExcessLiquidity — This value shows your margin cushion, before
///           liquidation
///         * - Cushion — Excess liquidity as a percentage of net liquidation
///           value
///         * - FullInitMarginReq — Initial Margin of whole portfolio with no
///           discounts or intraday credits
///         * - FullMaintMarginReq — Maintenance Margin of whole portfolio with
///           no discounts or intraday credits
///         * - FullAvailableFunds — Available funds of whole portfolio with no
///           discounts or intraday credits
///         * - FullExcessLiquidity — Excess liquidity of whole portfolio with
///           no discounts or intraday credits
///         * - LookAheadNextChange — Time when look-ahead values take effect
///         * - LookAheadInitMarginReq — Initial Margin requirement of whole
///           portfolio as of next period's margin change
///         * - LookAheadMaintMarginReq — Maintenance Margin requirement of
///           whole portfolio as of next period's margin change
///         * - LookAheadAvailableFunds — This value reflects your available
///           funds at the next margin change
///         * - LookAheadExcessLiquidity — This value reflects your excess
///           liquidity at the next margin change
///         * - HighestSeverity — A measure of how close the account is to
///           liquidation
///         * - DayTradesRemaining — The Number of Open/Close trades a user
///           could put on before Pattern Day Trading is detected. A value of
///           "-1" means that the user can put on unlimited day trades.
///         * - Leverage — GrossPositionValue / NetLiquidation
///         * - $LEDGER — Single flag to relay all cash balance tags*, only in
///           base currency.
///         * - $LEDGER:CURRENCY — Single flag to relay all cash balance tags*,
///           only in the specified currency.
///         * - $LEDGER:ALL — Single flag to relay all cash balance tags* in all
///           currencies.
///         */
#[derive(Debug)]
pub struct RequestAccountSummary {
    req_id: RequestId,
    group_name: String,
    tags: Vec<String>,
}

impl RequestAccountSummary {
    /// Create a new `Set` command which sets `key` to `value`.
    ///
    /// If `expire` is `Some`, the value should expire after the specified
    /// duration.
    pub fn new(req_id: RequestId, group_name: String, tags: Vec<String>) -> RequestAccountSummary {
        RequestAccountSummary {
            req_id,
            group_name,
            tags,
        }
    }

    /// Converts the command into an equivalent `Frame`.
    ///
    /// This is called by the client when encoding a `RequestMarketData` command
    /// to send to the server.
    pub(crate) fn into_frame(self) -> Frame {
        let mut msg = Outgoing::ReqAccountSummary.encode();
        msg.push_str(&VERSION.encode());
        msg.push_str(&self.req_id.encode());
        msg.push_str(&self.group_name.encode());
        msg.push_str(&self.tags.join(",").encode());
        let msg = msg.as_str().to_ib_message().unwrap();
        Frame::Bulk(Bytes::from(msg))
    }
}
