mod api;
pub use api::Api;

mod request_account_summary;
pub use request_account_summary::RequestAccountSummary;
mod request_account_updates;
pub use request_account_updates::RequestAccountUpdates;

pub(crate) mod request_market_data;
pub use request_market_data::*;
mod request_contract_details;
pub use request_contract_details::RequestContractDetails;
mod place_order;
pub use place_order::PlaceOrder;
pub mod request_executions;
pub use request_executions::RequestExecutions;
mod request_orders;
pub use request_orders::RequestOrders;

pub(crate) trait IntoIbkrFrame {
    /// Converts the command into an equivalent `Frame`.
    ///
    /// This is called by the client when encoding a command
    /// to send to the server.

    #[allow(clippy::wrong_self_convention)]
    fn into_frame(&self) -> crate::frame::Frame;
}
