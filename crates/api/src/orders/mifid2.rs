use crate::utils::ib_message::Encodable;

#[derive(Debug, Clone, Default)]
pub struct Mifid2 {
    pub decision_maker:   Option<String>,
    pub decision_algo:    Option<String>,
    pub execution_trader: Option<String>,
    pub execution_algo:   Option<String>,
}

// // TODO: Check None
// impl ParseIbkrFrame for Mifid2 {
//     #[allow(clippy::cognitive_complexity)]
//     fn try_parse_frame(
//         msg_id: Incoming,
//         server_version: Option<ServerVersion>,
//         it: &mut Split<&str>,
//     ) -> ParseResult<Self>
//     where
//         Self: Sized,
//     {
//         if !matches!(msg_id, Incoming::OpenOrder | Incoming::CompletedOrder)
// {             return Err(ParseError::UnexpectedMessage);
//         }
//         let server_version =
// server_version.ok_or(ParseError::MissingServerVersion)?;         let mut
// result = Self {             fa_group:      {
//                 // skip deprecated sharesAllocation field
//                 if msg_id == Incoming::OpenOrder {
//                     it.next();
//                 }
//                 decode(it)?
//             },
//             fa_method:     decode(it)?,
//             fa_percentage: decode(it)?,
//             fa_profile:    decode(it)?,
//         };

//         Ok(result)
//     }
// }
impl Encodable for Mifid2 {
    fn encode(&self) -> String {
        let mut code = String::new();

        code.push_str(&self.decision_maker.encode());
        code.push_str(&self.decision_algo.encode());
        code.push_str(&self.execution_trader.encode());
        code.push_str(&self.execution_algo.encode());
        code
    }
}
