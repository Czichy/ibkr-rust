    pub struct ChangeInDividendAccrual
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

        pub ExDate:string,

        pub PayDate:string,

        pub Quantity:Option<Decimal>,

        pub Tax:Option<Decimal>,

        pub Fee:Option<Decimal>,

        pub GrossRate:Option<Decimal>,

        pub GrossAmount:Option<Decimal>,

        pub NetAmount:Option<Decimal>,

        pub Code:string,

        pub FromAcct:string,

        pub ToAcct:string,
    }