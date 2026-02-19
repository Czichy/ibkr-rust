use bytes::Bytes;

use crate::{enums::Outgoing,
            frame::Frame,
            orders::Order,
            utils::ib_message::{Encodable, IBMessage}};

/// Encodes a PlaceOrder request for the TWS API protocol.
///
/// TWS API v100+ PlaceOrder frame format:
///   PLACE_ORDER(3) | order_id | contract fields | order fields ...
///
/// The order_id MUST be sent before the contract/order payload,
/// otherwise TWS misinterprets the con_id field as order_id and
/// the symbol string as con_id, causing Error 320.
#[derive(Debug)]
pub struct PlaceOrder(Order);

impl PlaceOrder {
    pub const fn new(order: Order) -> PlaceOrder { PlaceOrder(order) }

    pub(crate) fn into_frame(self) -> Frame {
        let mut msg = Outgoing::PlaceOrder.encode();
        // TWS API requires order_id before the contract+order payload
        msg.push_str(&self.0.order_id.encode());
        msg.push_str(&self.0.encode());
        let msg = msg.as_str().to_ib_message().unwrap();
        Frame::Bulk(Bytes::from(msg))
    }
}
