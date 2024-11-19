    pub struct TransactionTax
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

        [Format(Constants.DateFormat)]
        pub Date:Option<NaiveDateTime>,

        pub TaxDescription:string,

        pub Quantity:Option<Decimal>,

        //Note: The reportDate XML attribute may contain either a date or aString, i.e. reportDate="MULTI"
        pub ReportDate:string,

        pub TaxAmount:Option<Decimal>,

        pub TradeID:Option<i64>,

        pub TradePrice:Option<Decimal>,

        pub Source:string,

        pub Code:string,

        pub LevelOfDetail:string,
    }