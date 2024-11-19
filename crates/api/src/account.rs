use std::{fmt, str::Split};

use rust_decimal::prelude::*;
use tokio::sync::watch;

use crate::{account_summary_tags::AccountValueKey,
            contract::Contract,
            ib_frame::{ParseError, ParseIbkrFrame, ParseResult},
            prelude::{ib_message::{decode, Decodable},
                      Incoming,
                      ParseEnumError},
            AccountCode,
            RequestId,
            ServerVersion,
            TimeStamp};
#[derive(Debug, Clone)]
pub struct AccountData {
    pub req_id:   Option<RequestId>,
    pub account:  AccountCode,
    pub key:      AccountValueKey,
    pub value:    String,
    pub currency: String,
}

impl ParseIbkrFrame for AccountData {
    fn try_parse_frame(
        msg_id: Incoming,
        server_version: Option<ServerVersion>,
        it: &mut Split<&str>,
    ) -> ParseResult<Self>
    where
        Self: Sized,
    {
        match msg_id {
            Incoming::AccountSummary => {
                it.next(); // skip version
                tracing::debug!("getting account values");
                let req_id = decode(it)?;
                let account = decode(it)?.unwrap();
                let key = &decode::<String>(it)?.unwrap();
                let key = AccountValueKey::from_str(key)
                    .unwrap_or_else(|_| AccountValueKey::Unknown(key.to_string()));
                Ok(Self {
                    req_id,
                    account,
                    key,
                    value: decode(it)?.unwrap(),
                    currency: decode(it)?.unwrap_or_default(),
                })
            },
            Incoming::AcctValue => {
                it.next(); // skip version
                tracing::debug!("getting account values");
                let key = &decode::<String>(it)?.unwrap();
                let key = AccountValueKey::from_str(key)
                    .unwrap_or_else(|_| AccountValueKey::Unknown(key.to_string()));
                Ok(Self {
                    req_id: None,
                    key,
                    value: decode(it)?.unwrap(),
                    currency: decode(it)?.unwrap_or_default(),
                    account: decode(it)?.unwrap(),
                })
            },
            _ => Err(ParseError::Incomplete),
        }
    }
}

pub type AccountLastUpdate = TimeStamp;

#[derive(Debug, Clone)]
pub enum AccountValue {
    Property { value: String },
    Balance { value: Decimal },
}
impl fmt::Display for AccountValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AccountValue::Property { value } => {
                write!(f, "{value}")
            },
            AccountValue::Balance { value } => {
                write!(f, "{value}")
            },
        }
    }
}
#[derive(Debug, Hash, Clone, Eq, PartialEq)]
pub struct AccountPropertyKey {
    pub key:      AccountValueKey,
    pub currency: String,
    // pub value:String,
}
//#[derive(Debug, Hash)]
// pub struct AccountBalance {
//    pub key:      AccountValueKey,
//    pub currency: Currency,
//    //pub value: Decimal,
//}

// pub struct InteractivebrokersAccountData {
//    /// The raw IB account properties
//    pub account_properties: HashMap<AccountProperty, String>,
//
//    /// The account cash balances indexed by currency
//    pub cash_balances: HashMap<AccountProperty, Decimal>,
//}
// pub struct InteractivebrokersAccount {
//    pub summaries: HashMap<AccountCode, InteractivebrokersAccountData>,
//}

type Updating<T> = watch::Receiver<Option<T>>;
type Sender<T> = watch::Sender<Option<T>>;
#[derive(Debug)]
pub struct Position {
    pub contract:       Contract,
    pub position:       Option<Decimal>,
    pub market_price:   Option<Decimal>,
    pub market_value:   Option<Decimal>,
    pub average_cost:   Option<Decimal>,
    pub unrealized_pnl: Option<Decimal>,
    pub realized_pnl:   Option<Decimal>,
}

#[derive(Debug, Clone)]
pub struct AccountReceiver {
    pub update_time_rx:            Updating<String>,
    pub account_code_rx:           Updating<String>,
    pub account_type_rx:           Updating<String>,
    pub cash_balance_rx:           Updating<Decimal>,
    pub equity_with_loan_value_rx: Updating<Decimal>,
    pub excess_liquidity_rx:       Updating<Decimal>,
    pub net_liquidation_rx:        Updating<Decimal>,
    pub realized_pnl_rx:           Updating<Decimal>,
    pub unrealized_pnl_rx:         Updating<Decimal>,
    pub total_cash_balance_rx:     Updating<Decimal>,
    pub portfolio_rx:              Updating<Vec<Position>>,
}
#[derive(Debug)]
pub struct AccountSender {
    pub update_time:            Sender<String>,
    pub account_code:           Sender<String>,
    pub account_type:           Sender<String>,
    pub cash_balance:           Sender<Decimal>,
    pub equity_with_loan_value: Sender<Decimal>,
    pub excess_liquidity:       Sender<Decimal>,
    pub net_liquidation:        Sender<Decimal>,
    pub realized_pnl:           Sender<Decimal>,
    pub unrealized_pnl:         Sender<Decimal>,
    pub total_cash_balance:     Sender<Decimal>,
    pub portfolio:              Sender<Vec<Position>>,
}
pub fn init_account_channel() -> (AccountSender, AccountReceiver) {
    let (update_time_t, update_time_r) = watch::channel(None);
    let (account_code_t, account_code_r) = watch::channel(None);
    let (account_type_t, account_type_r) = watch::channel(None);
    let (cash_balance_t, cash_balance_r) = watch::channel(None);
    let (equity_with_loan_value_t, equity_with_loan_value_r) = watch::channel(None);
    let (excess_liquidity_t, excess_liquidity_r) = watch::channel(None);
    let (net_liquidation_t, net_liquidation_r) = watch::channel(None);
    let (realized_pnl_t, realized_pnl_r) = watch::channel(None);
    let (unrealized_pnl_t, unrealized_pnl_r) = watch::channel(None);
    let (total_cash_balance_t, total_cash_balance_r) = watch::channel(None);
    let (portfolio_t, portfolio_r) = watch::channel(None);
    let sender = AccountSender {
        update_time:            update_time_t,
        account_code:           account_code_t,
        account_type:           account_type_t,
        cash_balance:           cash_balance_t,
        equity_with_loan_value: equity_with_loan_value_t,
        excess_liquidity:       excess_liquidity_t,
        net_liquidation:        net_liquidation_t,
        realized_pnl:           realized_pnl_t,
        unrealized_pnl:         unrealized_pnl_t,
        total_cash_balance:     total_cash_balance_t,
        portfolio:              portfolio_t,
    };
    let receiver = AccountReceiver {
        update_time_rx:            update_time_r,
        account_code_rx:           account_code_r,
        account_type_rx:           account_type_r,
        cash_balance_rx:           cash_balance_r,
        equity_with_loan_value_rx: equity_with_loan_value_r,
        excess_liquidity_rx:       excess_liquidity_r,
        net_liquidation_rx:        net_liquidation_r,
        realized_pnl_rx:           realized_pnl_r,
        unrealized_pnl_rx:         unrealized_pnl_r,
        total_cash_balance_rx:     total_cash_balance_r,
        portfolio_rx:              portfolio_r,
    };
    (sender, receiver)
}
impl AccountReceiver {
    pub fn update_time(&self) -> Option<String> {
        (*self.update_time_rx.borrow()).as_ref().cloned()
    }

    pub fn unrealized_pnl(&self) -> Option<Decimal> {
        (*self.unrealized_pnl_rx.borrow()).as_ref().cloned()
    }
}
#[derive(Debug, Clone, Copy)]
pub enum IBAccountField {
    AccountType,
    NetLiquidation,
    TotalCashValue,
    SettledCash,
    AccruedCash,
    BuyingPower,
    EquityWithLoanValue,
    PreviousEquityWithLoanValue,
    GrossPositionValue,
    ReqTEquity,
    ReqTMargin,
    SMA,
    InitMarginReq,
    MaintMarginReq,
    AvailableFunds,
    ExcessLiquidity,
    Cushion,
    FullInitMarginReq,
    FullMaintMarginReq,
    FullAvailableFunds,
    FullExcessLiquidity,
    LookAheadNextChange,
    LookAheadInitMarginReq,
    LookAheadMaintMarginReq,
    LookAheadAvailableFunds,
    LookAheadExcessLiquidity,
    HighestSeverity,
    DayTradesRemaining,
    Leverage,
}

impl FromStr for IBAccountField {
    type Err = ParseEnumError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "AccountType" => Ok(IBAccountField::AccountType),
            "NetLiquidation" => Ok(IBAccountField::NetLiquidation),
            "TotalCashValue" => Ok(IBAccountField::TotalCashValue),
            "SettledCash" => Ok(IBAccountField::SettledCash),
            "AccruedCash" => Ok(IBAccountField::AccruedCash),
            "BuyingPower" => Ok(IBAccountField::BuyingPower),
            "EquityWithLoanValue" => Ok(IBAccountField::EquityWithLoanValue),
            "PreviousEquityWithLoanValue" => Ok(IBAccountField::PreviousEquityWithLoanValue),
            "GrossPositionValue" => Ok(IBAccountField::GrossPositionValue),
            "ReqTEquity" => Ok(IBAccountField::ReqTEquity),
            "ReqTMargin" => Ok(IBAccountField::ReqTMargin),
            "SMA" => Ok(IBAccountField::SMA),
            "InitMarginReq" => Ok(IBAccountField::InitMarginReq),
            "MaintMarginReq" => Ok(IBAccountField::MaintMarginReq),
            "AvailableFunds" => Ok(IBAccountField::AvailableFunds),
            "ExcessLiquidity" => Ok(IBAccountField::ExcessLiquidity),
            "Cushion" => Ok(IBAccountField::Cushion),
            "FullInitMarginReq" => Ok(IBAccountField::FullInitMarginReq),
            "FullMaintMarginReq" => Ok(IBAccountField::FullMaintMarginReq),
            "FullAvailableFunds" => Ok(IBAccountField::FullAvailableFunds),
            "FullExcessLiquidity" => Ok(IBAccountField::FullExcessLiquidity),
            "LookAheadNextChange" => Ok(IBAccountField::LookAheadNextChange),
            "LookAheadInitMarginReq" => Ok(IBAccountField::LookAheadInitMarginReq),
            "LookAheadMaintMarginReq" => Ok(IBAccountField::LookAheadMaintMarginReq),
            "LookAheadAvailableFunds" => Ok(IBAccountField::LookAheadAvailableFunds),
            "LookAheadExcessLiquidity" => Ok(IBAccountField::LookAheadExcessLiquidity),
            "HighestSeverity" => Ok(IBAccountField::HighestSeverity),
            "DayTradesRemaining" => Ok(IBAccountField::DayTradesRemaining),
            "Leverage" => Ok(IBAccountField::Leverage),
            &_ => Err(ParseEnumError),
        }
    }
}

impl Decodable for IBAccountField {}
