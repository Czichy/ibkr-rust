use std::str::Split;

use crate::{enums::*,
            ib_frame::{ParseError, ParseIbkrFrame, ParseResult},
            utils::ib_message::decode,
            ServerVersion};

#[derive(Debug, Clone, Default)]
pub struct FinancialAdvisor {
    pub fa_group:      Option<String>,
    pub fa_profile:    Option<String>,
    pub fa_method:     Option<String>,
    pub fa_percentage: Option<String>,
}

// TODO: Check None
impl ParseIbkrFrame for FinancialAdvisor {
    #[allow(clippy::cognitive_complexity)]
    fn try_parse_frame(
        msg_id: Incoming,
        _server_version: Option<ServerVersion>,
        it: &mut Split<&str>,
    ) -> ParseResult<Self>
    where
        Self: Sized,
    {
        if !matches!(msg_id, Incoming::OpenOrder | Incoming::CompletedOrder) {
            return Err(ParseError::UnexpectedMessage);
        }
        // let server_version = server_version.ok_or(ParseError::MissingServerVersion)?;
        let result = Self {
            fa_group:      {
                // skip deprecated sharesAllocation field
                if msg_id == Incoming::OpenOrder {
                    it.next();
                }
                decode(it)?
            },
            fa_method:     decode(it)?,
            fa_percentage: decode(it)?,
            fa_profile:    decode(it)?,
        };

        Ok(result)
    }
}
impl crate::utils::ib_message::Encodable for FinancialAdvisor {
    fn encode(&self) -> String {
        let mut code = String::new();

        code.push_str(&self.fa_group.encode());
        code.push_str(&self.fa_method.encode());
        code.push_str(&self.fa_percentage.encode());
        // fa_profile deprecated since MIN_SERVER_VER_FA_PROFILE_DESUPPORT (177)
        // Modern TWS (>= 177) does NOT expect this field
        code
    }
}
