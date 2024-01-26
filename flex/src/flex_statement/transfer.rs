    pub struct Transfer
    {
        pub AccountId:string,

        pub AcctAlias:string,

        pub Model:string,

        pub Currency:Option<Currency>,

        pub FxRateToBase:Option<Decimal>,

        pub AssetCategory:Option<AssetCategory>,

        pub Symbol:string,

        pub Description:string,

        pub Conid:Option<i64>,

        pub SecurityID:string,

        pub SecurityIDType:string,

        pub Cusip:string,

        pub Isin:string,

        pub ListingExchange:string,

        pub UnderlyingConid:Option<i64>,

        pub UnderlyingSymbol:string,

        pub UnderlyingSecurityID:string,

        pub UnderlyingListingExchange:string,

        pub Issuer:string,

        pub Multiplier:Option<i32>,

        pub Strike:Option<Decimal>,

        [Format(Constants.DateFormat)]
        pub Expiry:Option<NaiveDateTime>,

        pub PutCall:Option<PutCall>,

        pub PrincipalAdjustFactor:string,

        //Note: The reportDate XML attribute may contain either a date or aString, i.e. reportDate="MULTI"
        pub ReportDate:string,

        [Format(Constants.DateFormat)]
        pub Date:Option<NaiveDateTime>,

        [Format(Constants.DateTimeFormat, 0)]
        // alternative format
        [Format(Constants.DateFormat, 1)]
        pub TradeDateTime:Option<NaiveDateTime>,

        pub Type:string,

        pub Direction:string,

        pub Company:string,

        pub Account:string,

        pub AccountName:string,

        pub Quantity:Option<Decimal>,

        pub TransferPrice:Option<Decimal>,

        pub PositionAmount:Option<Decimal>,

        pub PositionAmountInBase:Option<Decimal>,

        pub PnlAmount:Option<Decimal>,

        pub PnlAmountInBase:Option<Decimal>,

        pub FxPnl:Option<Decimal>,

        pub CashTransfer:Option<Decimal>,

        pub Code:string,

        pub ClientReference:string,

        pub TransactionID:Option<i64>,
    }