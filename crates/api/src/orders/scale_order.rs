use std::str::Split;

use rust_decimal::prelude::*;

use crate::{enums::*,
            ib_frame::{ParseError, ParseIbkrFrame, ParseResult},
            utils::ib_message::decode,
            ServerVersion};

#[derive(Debug, Clone, Default)]
pub struct ScaleOrderParameter {
    pub scale_init_level_size:       Option<i32>,
    pub scale_subs_level_size:       Option<i32>,
    pub scale_price_increment:       Option<Decimal>,
    pub scale_price_adjust_value:    Option<Decimal>,
    pub scale_price_adjust_interval: Option<i32>,
    pub scale_profit_offset:         Option<Decimal>,
    pub scale_auto_reset:            bool,
    pub scale_init_position:         Option<i32>,
    pub scale_init_fill_qty:         Option<i32>,
    pub scale_random_percent:        bool,
    pub scale_table:                 Option<String>,
}

// TODO: Check None
impl ParseIbkrFrame for ScaleOrderParameter {
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
        let mut result = Self {
            scale_init_level_size: decode(it)?,
            scale_subs_level_size: decode(it)?,
            scale_price_increment: decode(it)?,
            ..Default::default()
        };
        if let Some(incr) = result.scale_price_increment {
            if incr > rust_decimal_macros::dec!(0.0) {
                result.scale_price_adjust_value = decode(it)?;
                result.scale_price_adjust_interval = decode(it)?;
                result.scale_profit_offset = decode(it)?;
                result.scale_auto_reset = decode(it)?.unwrap();
                result.scale_init_position = decode(it)?;
                result.scale_init_fill_qty = decode(it)?;
                result.scale_random_percent = decode(it)?.unwrap();
            }
        }
        Ok(result)
    }
}
impl crate::utils::ib_message::Encodable for ScaleOrderParameter {
    fn encode(&self) -> String {
        let mut code = String::new();

        code.push_str(&self.scale_init_level_size.encode());
        code.push_str(&self.scale_subs_level_size.encode());
        code.push_str(&self.scale_price_increment.encode());
        if let Some(inc) = self.scale_price_increment {
            if inc > rust_decimal_macros::dec!(0.0) {
                code.push_str(&self.scale_price_adjust_value.encode());
                code.push_str(&self.scale_price_adjust_interval.encode());
                code.push_str(&self.scale_profit_offset.encode());
                code.push_str(&self.scale_auto_reset.encode());
                code.push_str(&self.scale_init_position.encode());
                code.push_str(&self.scale_init_fill_qty.encode());
                code.push_str(&self.scale_random_percent.encode());
            }
        }
        code
    }
}
