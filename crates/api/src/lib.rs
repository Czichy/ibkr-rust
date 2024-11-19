#![warn(
    // missing_docs,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    unused_import_braces,
    unused_qualifications
)]
#![deny(clippy::all)]
// #![deny(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::cast_lossless)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::cast_precision_loss)]
#![allow(renamed_and_removed_lints)] // workaround clippy unknown lints when rust stable 1.50. May be removed in the future
#![allow(clippy::unknown_clippy_lints)] // workaround clippy unknown lints when rust stable 1.50. May be removed in the future
#![allow(unknown_lints)] // workaround clippy unknown lints when rust stable 1.50. May be removed in the future
#![allow(clippy::upper_case_acronyms)]
#![deny(clippy::nursery)]
#![allow(clippy::use_self)]

use chrono::{DateTime, Utc};

pub mod account;
pub mod account_summary_tags;
pub mod api_message;
pub mod bars;
pub mod client;
pub mod cmd;
pub mod contract;
pub mod enums;
mod frame;
mod ib_frame;
pub mod order;
mod reader;
mod shutdown;
pub mod ticker;
mod utils;
mod writer;

pub type ClientId = i32;
pub type ServerVersion = i32;
pub type OrderId = i32;
pub type RequestId = usize;
pub type AccountCode = String;
pub type TimeStamp = DateTime<Utc>;

pub mod prelude {
    #[doc(hidden)]
    pub use crate::{
        account::*, account_summary_tags::*, api_message::*, bars::*, client::*, cmd::*,
        contract::*, enums::*, order::*, ticker::*, utils::*, AccountCode, ClientId, Error,
        OrderId, RequestId, Result, TimeStamp,
    };
}
// use parse::{Parse, ParseError};

/// Error returned by most functions.
///
/// When writing a real application, one might want to consider a specialized
/// error handling crate or defining an error type as an `enum` of causes.
/// However, for our example, using a boxed `std::error::Error` is sufficient.
///
/// For performance reasons, boxing is avoided in any hot path. For example, in
/// `parse`, a custom error `enum` is defined. This is because the error is hit
/// and handled during normal execution when a partial frame is received on a
/// socket. `std::error::Error` is implemented for `parse::Error` which allows
/// it to be converted to `Box<dyn std::error::Error>`.
pub type Error = Box<dyn std::error::Error + Send + Sync>;

/// A specialized `Result` type for mini-redis operations.
///
/// This is defined as a convenience.
pub type Result<T> = std::result::Result<T, Error>;

/// Main value type for market data
///
/// Default is `f64`
///
/// If you want use `f32` which is (may be) faster, you can use `cargo build
/// --features value_type_f32`
///
/// Or in your `cargo.toml`:
///
/// ```toml
/// [dependencies]
/// ibkr_rust_api = { features = [ "value_type_f32" ] }
/// ```
///
/// Read more at [Features section](https://doc.rust-lang.org/cargo/reference/features.html#the-features-section)
///
/// # See also
///
/// [`PeriodType`]
#[cfg(not(feature = "market_data_value_type_f32"))]
pub type MarketDataValueType = f64;
#[cfg(feature = "market_data_value_type_f32")]
#[allow(missing_docs)]
pub type MarketDataValueType = f32;

/// `PeriodType` is a type for using on methods and indicators params.
///
/// For default it is `u8` (from `0` to `255`). That means you can use up to
/// `SMA::new(254)`, `WMA::new(254)`, etc... That's right, there are not `255`,
/// but `254` (`u8::MAX` - 1)
///
/// If you want use larger periods, you can switch it by using crate features:
/// `period_type_u16`, `period_type_u32`, `period_type_u64`.
///
/// F.e. `cargo build --features period_type_u16`
///
/// or in your `cargo.toml`:
///
/// ```toml
/// [dependencies]
/// yata = { features = ["period_type_u16"] }
/// ```
///
/// Read more at [Features section](https://doc.rust-lang.org/cargo/reference/features.html#the-features-section)
///
/// # See also
///
/// [`ValueType`]
#[cfg(not(any(
    feature = "period_type_u16",
    feature = "period_type_u32",
    feature = "period_type_u64"
)))]
pub type PeriodType = u8;
#[cfg(all(
    feature = "period_type_u16",
    not(any(feature = "period_type_u32", feature = "period_type_u64"))
))]
#[allow(missing_docs)]
pub type PeriodType = u16;
#[cfg(all(feature = "period_type_u32", not(feature = "period_type_u64")))]
#[allow(missing_docs)]
pub type PeriodType = u32;
#[cfg(feature = "period_type_u64")]
#[allow(missing_docs)]
pub type PeriodType = u64;
