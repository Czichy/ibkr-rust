#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
   
#[yaserde()]
pub struct Order {
    #[yaserde(attribute, rename = "tax")]
    pub tax: String,

    #[yaserde(attribute, rename = "code")]
    pub code: String,

    #[yaserde(attribute, rename = "isin")]
    pub isin: String,

    #[yaserde(attribute, rename = "conid")]
    pub conid: String,

    #[yaserde(attribute, rename = "cusip")]
    pub cusip: String,

    #[yaserde(attribute, rename = "model")]
    pub model: String,

    #[yaserde(attribute, rename = "price")]
    pub price: String,

    #[yaserde(attribute, rename = "rfqID")]
    pub rfq_id: String,

    #[yaserde(attribute, rename = "amount")]
    pub amount: String,

    #[yaserde(attribute, rename = "execID")]
    pub exec_id: String,

    #[yaserde(attribute, rename = "expiry")]
    pub expiry: String,

    #[yaserde(attribute, rename = "issuer")]
    pub issuer: String,

    #[yaserde(attribute, rename = "strike")]
    pub strike: String,

    #[yaserde(attribute, rename = "symbol")]
    pub symbol: String,

    #[yaserde(attribute, rename = "weight")]
    pub weight: String,

    #[yaserde(attribute, rename = "buySell")]
    pub buy_sell: String,

    #[yaserde(attribute, rename = "orderID")]
    pub order_id: String,

    #[yaserde(attribute, rename = "putCall")]
    pub put_call: String,

    #[yaserde(attribute, rename = "tradeID")]
    pub trade_id: String,

    #[yaserde(attribute, rename = "currency")]
    pub currency: String,

    #[yaserde(attribute, rename = "dateTime")]
    pub date_time: String,

    #[yaserde(attribute, rename = "exchange")]
    pub exchange: String,

    #[yaserde(attribute, rename = "fineness")]
    pub fineness: String,

    #[yaserde(attribute, rename = "proceeds")]
    pub proceeds: String,

    #[yaserde(attribute, rename = "quantity")]
    pub quantity: String,

    #[yaserde(attribute, rename = "traderID")]
    pub trader_id: String,

    #[yaserde(attribute, rename = "accountId")]
    pub account_id: String,

    #[yaserde(attribute, rename = "acctAlias")]
    pub acct_alias: String,

    #[yaserde(attribute, rename = "orderTime")]
    pub order_time: String,

    #[yaserde(attribute, rename = "orderType")]
    pub order_type: String,

    #[yaserde(attribute, rename = "tradeDate")]
    pub trade_date: String,

    #[yaserde(attribute, rename = "accruedInt")]
    pub accrued_int: String,

    #[yaserde(attribute, rename = "commission")]
    pub commission: String,

    #[yaserde(attribute, rename = "isAPIOrder")]
    pub is_api_order: String,

    #[yaserde(attribute, rename = "multiplier")]
    pub multiplier: String,

    #[yaserde(attribute, rename = "reportDate")]
    pub report_date: String,

    #[yaserde(attribute, rename = "securityID")]
    pub security_id: String,

    #[yaserde(attribute, rename = "settleDate")]
    pub settle_date: String,

    #[yaserde(attribute, rename = "allocatedTo")]
    pub allocated_to: String,

    #[yaserde(attribute, rename = "description")]
    pub description: String,

    #[yaserde(attribute, rename = "origTradeID")]
    pub orig_trade_id: String,

    #[yaserde(attribute, rename = "deliveryType")]
    pub delivery_type: String,

    #[yaserde(attribute, rename = "serialNumber")]
    pub serial_number: String,

    #[yaserde(attribute, rename = "assetCategory")]
    pub asset_category: String,

    #[yaserde(attribute, rename = "commodityType")]
    pub commodity_type: String,

    #[yaserde(attribute, rename = "levelOfDetail")]
    pub level_of_detail: String,

    #[yaserde(attribute, rename = "origTradeDate")]
    pub orig_trade_date: String,

    #[yaserde(attribute, rename = "clearingFirmID")]
    pub clearing_firm_id: String,

    #[yaserde(attribute, rename = "orderReference")]
    pub order_reference: String,

    #[yaserde(attribute, rename = "origTradePrice")]
    pub orig_trade_price: String,

    #[yaserde(attribute, rename = "securityIDType")]
    pub security_id_type: String,

    #[yaserde(attribute, rename = "listingExchange")]
    pub listing_exchange: String,

    #[yaserde(attribute, rename = "otherCommission")]
    pub other_commission: String,

    #[yaserde(attribute, rename = "transactionType")]
    pub transaction_type: String,

    #[yaserde(attribute, rename = "underlyingConid")]
    pub underlying_conid: String,

    #[yaserde(attribute, rename = "brokerageOrderID")]
    pub brokerage_order_id: String,

    #[yaserde(attribute, rename = "underlyingSymbol")]
    pub underlying_symbol: String,

    #[yaserde(attribute, rename = "commissionCurrency")]
    pub commission_currency: String,

    #[yaserde(attribute, rename = "volatilityOrderLink")]
    pub volatility_order_link: String,

    #[yaserde(attribute, rename = "underlyingSecurityID")]
    pub underlying_security_id: String,

    #[yaserde(attribute, rename = "principalAdjustFactor")]
    pub principal_adjust_factor: String,

    #[yaserde(attribute, rename = "brokerClearingCommission")]
    pub broker_clearing_commission: String,

    #[yaserde(attribute, rename = "brokerExecutionCommission")]
    pub broker_execution_commission: String,

    #[yaserde(attribute, rename = "underlyingListingExchange")]
    pub underlying_listing_exchange: String,

    #[yaserde(attribute, rename = "thirdPartyClearingCommission")]
    pub third_party_clearing_commission: String,

    #[yaserde(attribute, rename = "thirdPartyExecutionCommission")]
    pub third_party_execution_commission: String,

    #[yaserde(attribute, rename = "thirdPartyRegulatoryCommission")]
    pub third_party_regulatory_commission: String,
}

impl Validate for Order {}

