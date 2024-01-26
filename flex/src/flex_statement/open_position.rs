    pub struct OpenPosition
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

        pub UnderlyingConid:Option<i32>,

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

        pub Position:Option<i32>,

        pub MarkPrice:Option<Decimal>,

        pub PositionValue:Option<Decimal>,

        pub OpenPrice:Option<Decimal>,

        pub CostBasisPrice:Option<Decimal>,

        pub CostBasisMoney:Option<Decimal>,

        pub PercentOfNAV:Option<Decimal>,

        pub FifoPnlUnrealized:Option<Decimal>,

        pub Side:Option<LongShort>,

        pub LevelOfDetail:string,

        [Format(Constants.DateTimeFormat)]
        pub OpenDateTime:Option<NaiveDateTime>,

        [Format(Constants.DateTimeFormat)]
        pub HoldingPeriodDateTime:Option<NaiveDateTime>,

        pub Code:string,

        pub OriginatingOrderID:Option<i64>,

        pub OriginatingTransactionID:Option<i64>,

        pub AccruedInt:Option<Decimal>,
    }