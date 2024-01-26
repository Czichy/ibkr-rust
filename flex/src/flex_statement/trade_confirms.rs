use crate::domain::*;
use chrono::NaiveDateTime;
use rust_decimal::Decimal;
use serde::Deserialize;
#[derive(Debug, Deserialize)]
pub struct TradeConfirms {
    pub trades: Vec<TradeConfirm>,
    //pub orders: Vec<Order>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TradeConfirm {
    pub AccountId: String,

    pub AcctAlias: String,

    pub Model: String,

    pub Currency: Option<Currency>,

    pub AssetCategory: Option<AssetCategory>,

    pub Symbol: String,

    pub Description: String,

    pub Conid: Option<i64>,

    pub SecurityID: String,

    pub SecurityIDType: String,

    pub Cusip: String,

    pub Isin: String,

    pub ListingExchange: String,

    pub UnderlyingConid: Option<i64>,

    pub UnderlyingSymbol: String,

    pub UnderlyingSecurityID: String,

    pub UnderlyingListingExchange: String,

    pub Issuer: String,

    pub Multiplier: Option<i32>,

    pub Strike: Option<Decimal>,

    pub Expiry: Option<NaiveDateTime>,

    pub TradeID: Option<i64>,

    pub PutCall: Option<PutCall>,

    pub ReportDate: String,

    pub SettleDate: Option<NaiveDateTime>,

    //Note: The tradeDate XML attribute may contain either a date or aString, i.e. tradeDate="MULTI"
    pub TradeDate: String,

    pub PrincipalAdjustFactor: String,

    pub DateTime: Option<NaiveDateTime>,

    pub TransactionType: String,

    pub Exchange: String,

    pub Quantity: Option<Decimal>,

    pub Proceeds: Option<Decimal>,

    pub Tax: Option<Decimal>,

    pub Commission: Option<Decimal>,

    pub CommissionCurrency: Option<Currency>,

    pub Price: Option<Decimal>,

    pub Amount: Option<Decimal>,

    pub OrigTradePrice: Option<Decimal>,

    pub OrigTradeDate: Option<NaiveDateTime>,

    pub OrigTradeID: Option<i64>,

    pub ClearingFirmID: String,

    pub BuySell: Option<BuySell>,

    pub OrderID: Option<i64>,

    pub ExecID: String,

    pub BrokerageOrderID: String,

    pub OrderReference: String,

    pub VolatilityOrderLink: String,

    pub OrderTime: Option<NaiveDateTime>,

    pub LevelOfDetail: String,

    pub OrderType: String,

    pub TraderID: String,

    pub IsAPIOrder: String,

    //pub Code: Option<Notes>,
    pub BrokerExecutionCommission: Option<Decimal>,

    pub BrokerClearingCommission: Option<Decimal>,

    pub ThirdPartyExecutionCommission: Option<Decimal>,

    pub ThirdPartyClearingCommission: Option<Decimal>,

    pub ThirdPartyRegulatoryCommission: Option<Decimal>,

    pub OtherCommission: Option<Decimal>,

    pub AllocatedTo: String,
}
