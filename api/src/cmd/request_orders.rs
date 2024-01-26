use bytes::Bytes;

use crate::{
    enums::Outgoing,
    frame::Frame,
    utils::ib_message::{Encodable, IBMessage},
};

const VERSION: i32 = 1;
#[derive(Debug, Copy, Clone)]
#[allow(dead_code)]
pub enum RequestOrders {
    AllOpen,
    AutoOpen { auto_bind: bool },
    Open,
    Completed { api_only: bool },
    NextOrderId,
}

impl RequestOrders {
    /// Converts the command into an equivalent `Frame`.
    ///
    /// This is called by the client when encoding a `RequestMarketData` command
    /// to send to the server.
    pub(crate) fn into_frame(self) -> Frame {
        match self {
            RequestOrders::AllOpen => {
                let mut msg = Outgoing::ReqAllOpenOrders.encode();
                msg.push_str(&VERSION.encode()); // version
                let msg = msg.as_str().to_ib_message().unwrap();
                Frame::Bulk(Bytes::from(msg))
            },
            RequestOrders::AutoOpen { auto_bind } => {
                let mut msg = Outgoing::ReqAutoOpenOrders.encode();
                msg.push_str(&VERSION.encode()); // version
                msg.push_str(&auto_bind.encode()); // version
                let msg = msg.as_str().to_ib_message().unwrap();
                Frame::Bulk(Bytes::from(msg))
            },
            RequestOrders::Completed { api_only } => {
                let mut msg = Outgoing::ReqCompletedOrders.encode();
                // msg.push_str(&VERSION.encode()); // version
                msg.push_str(&api_only.encode()); // version
                let msg = msg.as_str().to_ib_message().unwrap();
                Frame::Bulk(Bytes::from(msg))
            },
            RequestOrders::Open => {
                let mut msg = Outgoing::ReqOpenOrders.encode();
                msg.push_str(&VERSION.encode()); // version
                let msg = msg.as_str().to_ib_message().unwrap();
                Frame::Bulk(Bytes::from(msg))
            },
            RequestOrders::NextOrderId => {
                let mut msg = Outgoing::ReqIds.encode();
                msg.push_str(&VERSION.encode());
                // deprecated num_ids
                msg.push_str(&0i32.encode());
                let msg = msg.as_str().to_ib_message().unwrap();
                Frame::Bulk(Bytes::from(msg))
            },
        }
    }
}
