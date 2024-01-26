use bytes::Bytes;

use crate::{
    enums::Outgoing,
    frame::Frame,
    utils::ib_message::{Encodable, IBMessage},
    AccountCode,
};
const VERSION: i32 = 2;

#[derive(Debug)]
pub struct RequestAccountUpdates {
    subscribe: bool,
    account_code: AccountCode,
}
/// Subscribes to a specific account's information and
/// portfolio.
///
/// Through this method, a single account's subscription can be
/// started/stopped. As a result from the subscription, the account's
/// information, portfolio and last update time will be received at
/// EWrapper::updateAccountValue, EWrapper::updateAccountPortfolio,
/// EWrapper::updateAccountTime respectively. All account values and
/// positions will be returned initially, and then there will only be
/// updates when there is a change in a position, or to an account
/// value every 3 minutes if it has changed.
///
/// Only one account can be subscribed at a time. A second
/// subscription request for another account when the previous one is
/// still active will cause the first one to be canceled in favour of
/// the second one. Consider user reqPositions if you want to retrieve
/// all your accounts' portfolios directly.
///
/// # Arguments
/// * `subscribe` - set to true to start the subscription and to false to stop
///   it.
/// * `acctCode` - the account id (i.e. U123456) for which the
/// information is requested.
impl RequestAccountUpdates {
    /// Create a new `Set` command which sets `key` to `value`.
    ///
    /// If `expire` is `Some`, the value should expire after the specified
    /// duration.
    pub const fn new(subscribe: bool, account_code: AccountCode) -> RequestAccountUpdates {
        RequestAccountUpdates {
            subscribe,
            account_code,
        }
    }

    /// Converts the command into an equivalent `Frame`.
    ///
    /// This is called by the client when encoding a `RequestMarketData` command
    /// to send to the server.
    pub(crate) fn into_frame(self) -> Frame {
        let mut msg = Outgoing::ReqAcctData.encode();
        msg.push_str(&VERSION.encode());
        msg.push_str(&self.subscribe.encode());
        msg.push_str(&self.account_code.encode());
        let msg = msg.as_str().to_ib_message().unwrap();
        Frame::Bulk(Bytes::from(msg))
    }
}
