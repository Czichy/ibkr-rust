pub mod dateparser;

pub mod ib_message {
    use std::{convert::TryInto, str};

    use chrono::{Local, NaiveDate, Utc};
    use rust_decimal::prelude::*;
    use tracing::log::error;

    use crate::{contract::OptionRight, TimeStamp};
    pub trait IBMessage {
        fn to_ib_message(&self) -> Result<Vec<u8>, std::num::TryFromIntError>;
    }

    impl IBMessage for &str {
        fn to_ib_message(&self) -> Result<Vec<u8>, std::num::TryFromIntError> {
            let msg_len: u32 = match self.len().try_into() {
                Ok(val) => val,
                Err(err) => return Err(err),
            };
            let len_bytes = msg_len.to_be_bytes();
            let mut res = Vec::with_capacity(self.len() + 4);
            res.extend_from_slice(&len_bytes);
            res.extend_from_slice(self.as_bytes());
            Ok(res)
        }
    }

    pub type Result<T, E = IbDecodeError> = std::result::Result<T, E>;

    #[derive(Debug, thiserror::Error)]
    pub enum IbDecodeError {
        #[error("Unknown decimal value: '{1}'")]
        UnknownDecimal(#[source] rust_decimal::Error, Box<str>),

        #[error("Unknown datetime value: '{1}'")]
        UnknownDateTime(#[source] anyhow::Error, Box<str>),

        #[error("Unknown date value: '{1}'")]
        UnknownDate(#[source] chrono::ParseError, Box<str>),

        #[error("Unknown epoch value: '{1}'")]
        UnknownUnixTimestamp(#[source] std::num::ParseIntError, Box<str>),

        #[error("Unknown string: '{}'", _0)]
        UnknownString(Box<str>),

        #[error("Unknown bool value: '{}'", _0)]
        UnknownBool(Box<str>),

        #[error("Unknown instrument type: '{}'", _0)]
        Mapping(Box<str>),

        #[error("Unknown option right: '{}'", _0)]
        UnknownRight(Box<str>),
    }

    pub trait Decodable
    where
        Self: FromStr + Sized,
    {
        fn decode_str(val: &str) -> Result<Self, IbDecodeError> {
            Self::from_str(val).map_or_else(|_| Err(IbDecodeError::UnknownString(val.into())), Ok)
        }
    }

    impl Decodable for i32 {}
    impl Decodable for u32 {}
    impl Decodable for usize {}
    impl Decodable for isize {}
    impl Decodable for f64 {}
    impl Decodable for Decimal {
        fn decode_str(val: &str) -> Result<Self, IbDecodeError> {
            match Decimal::from_str(val) {
                Ok(decimal) => Ok(decimal),
                Err(_) => {
                    Ok(Decimal::from_scientific(val)
                        .map_err(|e| IbDecodeError::UnknownDecimal(e, val.into()))?)
                },
            }
        }
    }

    impl Decodable for String {}
    impl Decodable for i64 {}

    impl Decodable for bool {
        fn decode_str(val: &str) -> Result<Self, IbDecodeError> {
            match val {
                "" | "0" => Ok(false),
                "1" => Ok(true),
                &_ => Err(IbDecodeError::UnknownBool(val.into())),
            }
        }
    }
    impl Decodable for TimeStamp {
        fn decode_str(val: &str) -> Result<Self, IbDecodeError> {
            super::dateparser::Parse::new(&Local, Utc::now().time())
                .parse(val)
                .map_err(|err| IbDecodeError::UnknownDateTime(err, val.into()))
        }
    }

    impl Decodable for NaiveDate {
        fn decode_str(val: &str) -> Result<Self, IbDecodeError> {
            let val = &val.replace([' ', '-'], "");
            match NaiveDate::parse_from_str(val, "%Y%m%d") {
                Ok(dt) => Ok(dt),
                Err(e) => Err(IbDecodeError::UnknownDate(e, val.as_str().into())),
            }
        }
    }

    impl Decodable for OptionRight {
        fn decode_str(val: &str) -> Result<Self, IbDecodeError> {
            tracing::debug!("decode value {} for OptionRight", val,);
            OptionRight::from_str(val)
                .map_or_else(|_| Err(IbDecodeError::UnknownRight(val.into())), Ok)
        }
    }

    pub fn decode<T>(stream: &mut std::str::Split<'_, &str>) -> Result<Option<T>, IbDecodeError>
    where
        T: FromStr + Sized + Decodable,
        <T as FromStr>::Err: std::fmt::Debug,
    {
        let str_val = stream.next().unwrap();
        tracing::debug!(
            "decode value {} for type {}",
            str_val,
            std::any::type_name::<T>()
        );
        match str_val {
            "" | "1.7976931348623157E308" | "2147483647" => Ok(None),
            _ => {
                match T::decode_str(str_val) {
                    Ok(val) => Ok(Some(val)),
                    Err(e) => {
                        Err(e)
                        // IbDecodeError{"{} could not be decoded into type:
                        // {:?}", str_val,type_name::<T>()}
                    },
                }
            },
        }
    }

    pub trait Encodable {
        fn encode(&self) -> String;
    }

    impl Encodable for f64 {
        fn encode(&self) -> String { self.to_string() + "\0" }
    }
    impl Encodable for i32 {
        fn encode(&self) -> String { self.to_string() + "\0" }
    }
    impl Encodable for i64 {
        fn encode(&self) -> String { self.to_string() + "\0" }
    }
    impl Encodable for Decimal {
        fn encode(&self) -> String { self.to_string() + "\0" }
    }
    impl Encodable for usize {
        fn encode(&self) -> String { self.to_string() + "\0" }
    }
    impl Encodable for String {
        fn encode(&self) -> String { self.to_string() + "\0" }
    }
    impl Encodable for &str {
        fn encode(&self) -> String { self.to_string() + "\0" }
    }

    impl Encodable for bool {
        fn encode(&self) -> String {
            if *self {
                "1\0".to_string()
            } else {
                "0\0".to_string()
            }
        }
    }

    impl<T: Encodable> Encodable for Option<T> {
        fn encode(&self) -> String {
            #[allow(clippy::option_if_let_else)]
            match self {
                Some(val) => val.encode(),
                None => "\0".to_string(),
            }
        }
    }

    impl Encodable for Vec<(String, String)> {
        fn encode(&self) -> String {
            let mut code = String::new();
            for tv in self {
                code.push_str(&tv.0);
                code.push('=');
                code.push_str(&tv.1);
                code.push(';');
            }
            code.push('\0');
            code
        }
    }

    // pub fn push_enc<T: Encodable>(str: &mut String, val: T) {
    // str.push_str(&val.encode()); }
}
#[cfg(test)]
#[allow(deprecated)]
mod tests {

    use chrono::{Local, NaiveDate, TimeZone, Utc};
    use pretty_assertions::assert_eq;
    use rust_decimal::prelude::*;
    use rust_decimal_macros::dec;

    use crate::{utils::ib_message::*, TimeStamp};
    #[test]
    fn decode_timestamp() {
        let dt = "20210921  20:58:03".to_string();
        let parse = match Local.datetime_from_str(&dt, "%Y%m%d  %H:%M:%S") {
            Ok(dt) => {
                let utc: TimeStamp = chrono::DateTime::from(dt);
                Ok(utc)
            },
            Err(e) => Err(e),
        };
        tracing::debug!("test {:?}", parse);
    }
    #[test]
    fn decode_timestamp_schedule() {
        let dt = "20211229-15:33:42 MEZ";
        let timestamp: TimeStamp = TimeStamp::decode_str(dt).unwrap();
        assert_eq!(
            Utc.with_ymd_and_hms(2021, 12, 29, 13, 33, 42).unwrap(),
            timestamp,
            "Time conversion"
        );
        let dt = "20210729-15:33:42 MEZ";
        let timestamp: TimeStamp = TimeStamp::decode_str(dt).unwrap();
        assert_eq!(
            Utc.with_ymd_and_hms(2021, 7, 29, 13, 33, 42).unwrap(),
            timestamp,
            "Time conversion"
        );
        let dt = "20210729-15:33:42 MESZ";
        let timestamp: TimeStamp = TimeStamp::decode_str(dt).unwrap();
        assert_eq!(
            Utc.with_ymd_and_hms(2021, 7, 29, 14, 33, 42).unwrap(),
            timestamp,
            "Time conversion"
        );

        tracing::debug!("test {:?}", timestamp);
    }
    #[test]
    #[allow(deprecated)]
    fn decode_timestamp_timezone() {
        let dt = "20211229 15:33:42 Europe/Berlin";
        let timestamp: TimeStamp = TimeStamp::decode_str(dt).unwrap();
        assert_eq!(
            Utc.with_ymd_and_hms(2021, 12, 29, 14, 33, 42).unwrap(),
            timestamp,
            "Time conversion"
        );
        tracing::debug!("test {:?}", timestamp);

        let dt = "15:33:42 Europe/Berlin";
        let timestamp: TimeStamp = TimeStamp::decode_str(dt).unwrap();
        assert_eq!(
            Utc::today().and_hms_opt(13, 33, 42).unwrap(),
            timestamp,
            "Time conversion"
        );
        tracing::debug!("test {:?}", timestamp);
    }

    #[test]
    fn decode_timestamp_mez() {
        let dt = "20220628-09:30:00";
        let timestamp: TimeStamp = TimeStamp::decode_str(dt).unwrap();
        assert_eq!(
            Utc.with_ymd_and_hms(2022, 6, 28, 7, 30, 0).unwrap(),
            timestamp,
            "Time conversion"
        );
        tracing::debug!("test {:?}", timestamp);
    }
    #[test]
    fn decode_timestamp_date() {
        let dt = "20220630";
        let timestamp: NaiveDate = NaiveDate::decode_str(dt).unwrap();
        assert_eq!(
            NaiveDate::from_ymd(2022, 6, 30),
            timestamp,
            "Time conversion"
        );
        tracing::debug!("test {:?}", timestamp);
    }
    #[test]
    fn decode_decimal() {
        let val = "1234.3";
        let decimal: Decimal = Decimal::decode_str(val).unwrap();
        assert_eq!(dec!(1234.3), decimal, "Decimal conversion");
        let val = "7.55E-4";
        let decimal: Decimal = Decimal::decode_str(val).unwrap();
        assert_eq!(dec!(0.000755), decimal, "Decimal conversion");
    }
}
