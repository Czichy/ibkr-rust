use bytes::Bytes;

use crate::{
    enums::Outgoing,
    frame::Frame,
    order::ExecutionFilter,
    utils::ib_message::{Encodable, IBMessage},
    RequestId,
};

const VERSION: i32 = 3;
#[derive(Debug)]
pub struct RequestExecutions {
    req_id: RequestId,
    filter: Option<ExecutionFilter>,
}

impl RequestExecutions {
    /// Create a new `Set` command which sets `key` to `value`.
    ///
    /// If `expire` is `Some`, the value should expire after the specified
    /// duration.
    pub const fn new(req_id: RequestId, filter: Option<ExecutionFilter>) -> RequestExecutions {
        RequestExecutions { req_id, filter }
    }

    /// Converts the command into an equivalent `Frame`.
    ///
    /// This is called by the client when encoding a `RequestMarketData` command
    /// to send to the server.
    pub(crate) fn into_frame(self) -> Frame {
        let mut msg = Outgoing::ReqExecutions.encode();
        msg.push_str(&VERSION.encode());
        msg.push_str(&self.req_id.encode());
        if let Some(filter) = &self.filter {
            msg.push_str(&filter.client_id.encode());
            msg.push_str(&filter.account_code.encode());
            msg.push_str(&filter.time.encode());
            msg.push_str(&filter.symbol.encode());
            msg.push_str(&filter.sec_type.encode());
            msg.push_str(&filter.exchange.encode());
            msg.push_str(&filter.side.encode());
        } else {
            for _ in 0..7 {
                msg.push('\0');
            }
        }
        let msg = msg.as_str().to_ib_message().unwrap();
        tracing::debug!("req exec: {:?}", &msg);
        Frame::Bulk(Bytes::from(msg))
    }
}
