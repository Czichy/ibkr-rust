use chrono::NaiveDateTime;
use iso_currency::Currency;
use serde::Deserialize;

use crate::utils::de::some_naive_date_time_from_str;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AccountInformation {
    #[serde(rename = "@accountId")]
    pub account_id: String,

    pub acct_alias: String,

    pub model: String,

    pub currency: Option<Currency>,

    pub name: String,

    pub account_type: String,

    pub customer_type: String,

    pub account_capabilities: String,

    pub trading_permissions: String,

    pub registered_rep_name: String,

    pub registered_rep_phone: String,

    #[serde(deserialize_with = "some_naive_date_time_from_str")]
    pub date_opened: Option<NaiveDateTime>,

    #[serde(deserialize_with = "some_naive_date_time_from_str")]
    pub date_funded: Option<NaiveDateTime>,

    #[serde(deserialize_with = "some_naive_date_time_from_str")]
    pub date_closed: Option<NaiveDateTime>,

    pub street: String,

    pub street2: String,

    pub city: String,

    pub state: String,

    pub country: String,

    pub postal_code: String,

    pub street_residential_address: String,

    pub street2_residential_address: String,

    pub city_residential_address: String,

    pub state_residential_address: String,

    pub country_residential_address: String,

    pub postal_code_residential_address: String,

    pub master_name: String,

    pub ib_entity: String,

    pub primary_email: String,
}
