use std::str::Split;

use fastnum::D256;

use crate::{enums::*,
            ib_frame::{ParseError, ParseIbkrFrame, ParseResult},
            utils::ib_message::decode,
            ServerVersion};

#[derive(Debug, Clone, Default)]
pub struct VolatilityOrderParameter {
    /// The option price in volatility
    pub volatility:                        Option<D256>,
    ///     Values include:
    /// 1 – Daily Volatility
    /// 2 – Annual Volatility.
    pub volatility_type:                   VolatilityType,
    /// Enter an order type to instruct TWS to submit a delta neutral trade on
    /// full or partial execution of the VOL order. VOL orders only. For no
    /// hedge delta order to be sent, specify NONE.
    pub delta_neutral_order_type:          OrderType,
    /// Use this field to enter a value if the value in the
    /// deltaNeutralOrderType field is an order type that requires an Aux price,
    /// such as a REL order. VOL orders only.
    pub delta_neutral_aux_price:           Option<D256>,
    /// The unique contract identifier specifying the security in Delta Neutral
    /// order.
    pub delta_neutral_con_id:              usize,
    /// Indicates the firm which will settle the Delta Neutral trade.
    /// Institutions only.
    pub delta_neutral_settling_firm:       Option<String>,
    /// Specifies the beneficiary of the Delta Neutral order.
    pub delta_neutral_clearing_account:    Option<String>,
    /// Specifies where the clients want their shares to be cleared at. Must be
    /// specified by execution-only clients. Valid values are:
    /// IB, Away, and PTA (post trade allocation).
    pub delta_neutral_clearing_intent:     Option<String>,
    /// Specifies whether the order is an Open or a Close order and is used when
    /// the hedge involves a CFD and and the order is clearing away.
    pub delta_neutral_open_close:          Option<String>,
    /// Used when the hedge involves a stock and indicates whether or not it is
    /// sold short.
    pub delta_neutral_short_sale:          bool,
    /// Indicates a short sale Delta Neutral order. Has a value of 1 (the
    /// clearing broker holds shares) or 2 (delivered from a third party). If
    /// you use 2, then you must specify a deltaNeutralDesignatedLocation.
    pub delta_neutral_short_sale_slot:     bool,
    /// Identifies third party order origin. Used only when
    /// deltaNeutralShortSaleSlot = 2.
    pub delta_neutral_designated_location: Option<String>,
    /// Specifies whether TWS will automatically update the limit price of the
    /// order as the underlying price moves. VOL orders only.
    pub continuous_update:                 bool,
    /// Specifies how you want TWS to calculate the limit price for options, and
    /// for stock range price monitoring. VOL orders only.
    /// Valid values include:
    /// 1 - Average of NBBO
    /// 2 - NBB or the NBO depending on the action and right.
    pub reference_price_type:              ReferencePriceType,
}

// TODO: Check None
impl ParseIbkrFrame for VolatilityOrderParameter {
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
        let _server_version = server_version.ok_or(ParseError::MissingServerVersion)?;
        let mut result = Self {
            volatility: decode(it)?,
            volatility_type: decode(it)?.unwrap(),
            delta_neutral_order_type: decode(it)?.unwrap(),
            delta_neutral_aux_price: decode(it)?,
            ..Default::default()
        };
        // if result.delta_neutral_order_type.is_some() {
        result.delta_neutral_con_id = decode(it)?.unwrap();
        if matches!(msg_id, Incoming::OpenOrder) {
            result.delta_neutral_settling_firm = decode(it)?;
            result.delta_neutral_clearing_account = decode(it)?;
            result.delta_neutral_clearing_intent = decode(it)?;
            result.delta_neutral_open_close = decode(it)?;
        }
        result.delta_neutral_short_sale = decode(it)?.unwrap();
        result.delta_neutral_short_sale_slot = decode(it)?.unwrap();
        result.delta_neutral_designated_location = decode(it)?;
        // }
        result.continuous_update = decode(it)?.unwrap();
        result.reference_price_type = decode(it)?.unwrap();
        Ok(result)
    }
}
impl crate::utils::ib_message::Encodable for VolatilityOrderParameter {
    fn encode(&self) -> String {
        let mut code = String::new();

        code.push_str(&self.volatility.encode());
        code.push_str(&self.volatility_type.encode());
        code.push_str(&self.delta_neutral_order_type.encode());
        code.push_str(&self.delta_neutral_aux_price.encode());
        // if self.delta_neutral_order_type.is_some() {
        code.push_str(&self.delta_neutral_con_id.encode());
        code.push_str(&self.delta_neutral_settling_firm.encode());
        code.push_str(&self.delta_neutral_clearing_account.encode());
        code.push_str(&self.delta_neutral_clearing_intent.encode());
        code.push_str(&self.delta_neutral_open_close.encode());
        code.push_str(&self.delta_neutral_short_sale.encode());
        code.push_str(&self.delta_neutral_designated_location.encode());
        // }
        code.push_str(&self.continuous_update.encode());
        code.push_str(&self.reference_price_type.encode());
        code
    }
}
