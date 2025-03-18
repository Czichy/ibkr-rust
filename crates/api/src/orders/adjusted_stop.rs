use std::str::Split;

use fastnum::D256;

use crate::{enums::*,
            ib_frame::{ParseError, ParseIbkrFrame, ParseResult},
            utils::ib_message::{decode, Encodable},
            ServerVersion};

#[derive(Debug, Clone, Default)]
pub struct AdjustedOrder {
    pub adjusted_order_type:       Option<String>,
    pub trail_stop_price:          Option<D256>,
    pub trigger_price:             Option<D256>,
    pub adjusted_stop_price:       Option<D256>,
    pub adjusted_stop_limit_price: Option<D256>,
    pub adjusted_trailing_amount:  Option<D256>,
    pub adjustable_trailing_unit:  i32,
    pub lmt_price_offset:          Option<D256>,
}

// // TODO: Check None
impl ParseIbkrFrame for AdjustedOrder {
    #[allow(clippy::cognitive_complexity)]
    fn try_parse_frame(
        msg_id: Incoming,
        server_version: Option<ServerVersion>,
        it: &mut Split<&str>,
    ) -> ParseResult<Self>
    where
        Self: Sized,
    {
        if !matches!(msg_id, Incoming::OpenOrder | Incoming::CompletedOrder) {
            return Err(ParseError::UnexpectedMessage);
        }
        let server_version = server_version.ok_or(ParseError::MissingServerVersion)?;
        let completed = matches!(msg_id, Incoming::CompletedOrder);
        let mut result = Self::default();
        if !completed {
            result.adjusted_order_type = decode(it)?;
            result.trigger_price = decode(it)?;
        }
        result.trail_stop_price = decode(it)?;
        result.lmt_price_offset = decode(it)?;
        if !completed {
            result.adjusted_stop_price = decode(it)?;
            result.adjusted_stop_limit_price = decode(it)?;
            result.adjusted_trailing_amount = decode(it)?;
            result.adjustable_trailing_unit = decode(it)?.unwrap();
        }

        Ok(result)
    }
}
impl Encodable for AdjustedOrder {
    fn encode(&self) -> String {
        let mut code = String::new();

        code.push_str(&self.adjusted_order_type.encode());
        code.push_str(&self.trigger_price.encode());
        code.push_str(&self.lmt_price_offset.encode());
        code.push_str(&self.adjusted_stop_price.encode());
        code.push_str(&self.adjusted_stop_limit_price.encode());
        code.push_str(&self.adjusted_trailing_amount.encode());
        code.push_str(&self.adjustable_trailing_unit.encode());
        code
    }
}
