use bytes::Bytes;

use crate::{
    enums::{Outgoing, ServerLogLevel},
    frame::Frame,
    utils::ib_message::{Encodable, IBMessage},
    ClientId,
};

const VERSION: i32 = 1;

#[derive(Debug)]
pub enum Api {
    /// Start the TWS/IB Gateway API.
    ///
    /// # Options
    ///
    /// Currently, the following options are supported:
    ///
    /// * EX `seconds` -- Set the specified expire time, in seconds.
    /// * PX `milliseconds` -- Set the specified expire time, in milliseconds.
    Init {
        min_client_version: i32,
        max_client_version: i32,
    },
    Start {
        client_id: ClientId,
        optional_capabilities: Option<String>,
    },
    SetServerLoglevel {
        log_level: ServerLogLevel,
    },
    RequestCurrentTime,
}
impl Api {
    pub(crate) fn into_frame(self) -> Frame {
        match self {
            Api::Init {
                min_client_version,
                max_client_version,
            } => {
                let mut frame = Frame::array();
                // frame.push_bulk(Bytes::from("API\0".as_bytes()));
                let mut valid_versions = min_client_version.to_string();
                valid_versions.push_str("..");
                valid_versions.push_str(&max_client_version.to_string());
                valid_versions.push_str(" +PACEAPI");
                let msg = valid_versions.as_str().to_ib_message().unwrap();
                frame.push_bulk(Bytes::from(msg));
                frame
            },
            Api::Start {
                client_id,
                optional_capabilities,
            } => {
                let mut msg = Outgoing::StartApi.encode();
                // start API
                msg.push_str(&VERSION.encode());
                msg.push_str(&client_id.encode());
                msg.push_str(&optional_capabilities.unwrap_or_else(|| "".into()).encode());
                let msg = msg.as_str().to_ib_message().unwrap();
                Frame::Bulk(Bytes::from(msg))
            },
            Api::SetServerLoglevel { log_level } => {
                let mut msg = Outgoing::SetServerLoglevel.encode();
                // start API
                msg.push_str(&VERSION.encode());
                msg.push_str(&log_level.encode());
                let msg = msg.as_str().to_ib_message().unwrap();
                Frame::Bulk(Bytes::from(msg))
            },
            Api::RequestCurrentTime => {
                let version = 2;
                let mut msg = Outgoing::ReqCurrentTime.encode();
                // start API
                msg.push_str(&version.encode());
                let msg = msg.as_str().to_ib_message().unwrap();
                Frame::Bulk(Bytes::from(msg))
            },
        }
    }
}
