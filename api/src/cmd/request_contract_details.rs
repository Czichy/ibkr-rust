use bytes::Bytes;

use crate::{
    contract,
    enums::Outgoing,
    frame::Frame,
    utils::ib_message::{Encodable, IBMessage},
    RequestId,
};
const VERSION: i32 = 8;

/// Call this function to download all details for a particular
/// underlying. The contract details will be received via the contractDetails()
/// function on the EWrapper.
///
///    
/// # Arguments
/// * req_id - The ID of the data request. Ensures that responses are matched to
///   requests if several requests are in process.
/// * contract - The summary description of the contract being looked up.
#[derive(Debug)]
pub struct RequestContractDetails {
    req_id: RequestId,
    contract: contract::Contract,
}

impl RequestContractDetails {
    /// Create a new `Set` command which sets `key` to `value`.
    ///
    /// If `expire` is `Some`, the value should expire after the specified
    /// duration.
    pub const fn new(req_id: RequestId, contract: contract::Contract) -> RequestContractDetails {
        RequestContractDetails { req_id, contract }
    }

    /// Converts the command into an equivalent `Frame`.
    ///
    /// This is called by the client when encoding a `RequestMarketData` command
    /// to send to the server.
    pub(crate) fn into_frame(self) -> Frame {
        let mut msg = Outgoing::ReqContractData.encode();
        msg.push_str(&VERSION.encode());
        msg.push_str(&self.req_id.encode());
        msg.push_str(&self.contract.encode());
        let msg = msg.as_str().to_ib_message().unwrap();
        Frame::Bulk(Bytes::from(msg))
    }
}
