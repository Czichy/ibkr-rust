use iso_currency::Currency;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::{flex_statement::contract::Contract, utils::de::*};

#[derive(Debug, Deserialize, Clone)]
pub struct OpenPositions {
    #[serde(rename = "OpenPosition", default)]
    pub positions: Vec<OpenPosition>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OpenPosition {
    #[serde(rename = "accountId")]
    pub account_id: String,

    #[serde(deserialize_with = "deserialize_option_from_str")]
    pub acct_alias: Option<String>,

    #[serde(deserialize_with = "deserialize_option_from_str")]
    pub model: Option<String>,

    pub fx_rate_to_base: Option<Decimal>,

    #[serde(flatten)]
    pub contract: Option<Contract>,
    //     [Format(Constants.DateFormat)]
    //     pub Expiry:Option<NaiveDateTime>,

    //     pub PutCall:Option<PutCall>,

    //     pub PrincipalAdjustFactor:string,

    //     //Note: The reportDate XML attribute may contain either a date or aString, i.e.
    // reportDate="MULTI"     pub ReportDate:string,

    //     pub Position:Option<i32>,

    //     pub MarkPrice:Option<Decimal>,

    //     pub PositionValue:Option<Decimal>,

    //     pub OpenPrice:Option<Decimal>,

    //     pub CostBasisPrice:Option<Decimal>,

    //     pub CostBasisMoney:Option<Decimal>,

    //     pub PercentOfNAV:Option<Decimal>,

    //     pub FifoPnlUnrealized:Option<Decimal>,

    //     pub Side:Option<LongShort>,

    //     pub LevelOfDetail:string,

    //     [Format(Constants.DateTimeFormat)]
    //     pub OpenDateTime:Option<NaiveDateTime>,

    //     [Format(Constants.DateTimeFormat)]
    //     pub HoldingPeriodDateTime:Option<NaiveDateTime>,

    //     pub Code:string,

    //     pub OriginatingOrderID:Option<i64>,

    //     pub OriginatingTransactionID:Option<i64>,

    //     pub AccruedInt:Option<Decimal>,
}
