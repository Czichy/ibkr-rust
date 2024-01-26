    pub struct SLBActivity
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

        pub Exchange:string,

        pub Type:string,

        [Format(Constants.DateFormat)]
        pub Date:Option<NaiveDateTime>,

        pub MarkCurrentPrice:Option<Decimal>,

        pub MarkPriorPrice:Option<Decimal>,

        pub MarkQuantity:Option<Decimal>,

        pub CollateralAmount:Option<Decimal>,

        pub ActivityDescription:string,

        pub Quantity:Option<Decimal>,

        pub SlbTransactionId:string,
    }