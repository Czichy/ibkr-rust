use std::str::Split;

use rust_decimal::prelude::*;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{contract::Contract,
            enums::*,
            ib_frame::{ParseError, ParseIbkrFrame, ParseResult},
            prelude::ib_message::{decode, Decodable},
            utils::ib_message::Encodable,
            AccountCode,
            ClientId,
            OrderId,
            ServerVersion,
            TimeStamp};

/// describing the liquidity type of an execution.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Liquidity {
    Unknown,
    AddedLiquidity,
    RemovedLiquidity,
    LiquidityRoutedOut,
}
impl Encodable for Liquidity {
    fn encode(&self) -> String {
        match self {
            Liquidity::Unknown => "0\0",
            Liquidity::AddedLiquidity => "1\0",
            Liquidity::RemovedLiquidity => "2\0",
            Liquidity::LiquidityRoutedOut => "3\0",
        }
        .to_string()
    }
}

impl FromStr for Liquidity {
    type Err = ParseEnumError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(Self::Unknown),
            "1" => Ok(Self::AddedLiquidity),
            "2" => Ok(Self::RemovedLiquidity),
            "3" => Ok(Self::LiquidityRoutedOut),
            &_ => Err(ParseEnumError),
        }
    }
}

impl Decodable for Liquidity {}

#[derive(Debug, Clone)]
pub struct Execution {
    /// The execution's identifier. Each partial fill has a separate ExecId.
    /// A correction is indicated by an ExecId which differs from a previous
    /// ExecId in only the digits after the final period,
    /// e.g. an ExecId ending in ".02" would be a correction of a previous
    /// execution with an ExecId ending in ".01"
    pub exec_id:        String,
    /// The execution's server time.
    pub time:           TimeStamp,
    /// The account to which the order was allocated.
    pub acct_number:    String,
    pub exchange:       Option<String>,
    pub side:           Side,
    pub shares:         Decimal,
    pub price:          Decimal,
    pub perm_id:        i32,
    /// The API client identifier which placed the order which originated this
    /// execution.
    pub client_id:      ClientId,
    /// The API client's order Id. May not be unique to an account.
    pub order_id:       OrderId,
    pub contract:       Contract,
    pub liquidation:    i32,
    pub cum_qty:        Decimal,
    pub avg_price:      Decimal,
    pub order_ref:      Option<String>,
    pub ev_rule:        Option<String>,
    pub ev_multiplier:  Option<Decimal>,
    pub model_code:     Option<String>,
    /// The liquidity type of the execution. Requires TWS 968+ and API v973.05+.
    /// Python API specifically requires API v973.06+.
    pub last_liquidity: Liquidity,

    /// pending price revision
    pub pending_price_revision: bool,

    /// Submitter
    pub submitter: String,
}

impl ParseIbkrFrame for Execution {
    fn try_parse_frame(
        msg_id: Incoming,
        server_version: Option<ServerVersion>,
        it: &mut Split<&str>,
    ) -> ParseResult<Self>
    where
        Self: Sized,
    {
        if !matches!(msg_id, Incoming::ExecutionData) {
            return Err(ParseError::UnexpectedMessage);
        }
        // let _req_id: i32 = decode(it)?.unwrap();
        let order_id: i32 = decode(it)?.unwrap();
        let contract = Contract::try_parse_frame(msg_id, server_version, it)?;
        let exec_id = decode(it)?.unwrap();
        tracing::debug!("parse execution id - {exec_id}!");
        Ok(Self {
            order_id,
            contract,
            exec_id,
            // exec_id: decode(it)?.unwrap(),
            time: decode(it)?.unwrap(),
            acct_number: decode(it)?.unwrap(),
            exchange: decode(it)?,
            side: decode(it)?.unwrap(),
            shares: decode(it)?.unwrap(),
            price: decode(it)?.unwrap(),

            perm_id: decode(it)?.unwrap(),
            client_id: decode(it)?.unwrap(),
            liquidation: decode(it)?.unwrap(),
            cum_qty: decode(it)?.unwrap(),
            avg_price: decode(it)?.unwrap(),
            order_ref: decode(it)?,
            ev_rule: decode(it)?,
            ev_multiplier: decode(it)?,
            model_code: decode(it)?,
            last_liquidity: decode(it)?.unwrap(),
            pending_price_revision: decode(it)?.unwrap(),
            submitter: decode(it)?.unwrap(),
        })
    }
}

#[derive(Clone, Debug, Default)]
pub struct ExecutionFilter {
    pub client_id:    Option<ClientId>,
    pub account_code: AccountCode,
    pub time:         String,
    pub symbol:       String,
    pub sec_type:     String,
    pub exchange:     String,
    pub side:         Option<Side>,
}

impl ExecutionFilter {
    pub const fn new(
        client_id: ClientId,
        acct_code: AccountCode,
        time: String,
        symbol: String,
        sec_type: String,
        exchange: String,
        side: Option<Side>,
    ) -> Self {
        ExecutionFilter {
            client_id: Some(client_id),
            account_code: acct_code,
            time,
            symbol,
            sec_type,
            exchange,
            side,
        }
    }
}
