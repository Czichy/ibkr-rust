    pub struct ComplexPosition
    {
        pub AccountId:string,

        pub AcctAlias:string,

        pub LevelOfDetail:string,

        pub Quantity:Option<Decimal>,

        pub PrincipalAdjustFactor:string,

        pub PutCall:Option<PutCall>,

        pub Issuer:string,

        pub Multiplier:Option<i32>,

        pub Strike:Option<Decimal>,

        [Format(Constants.DateFormat)]
        pub Expiry:Option<NaiveDateTime>,

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

        pub MtmPnl:Option<Decimal>,

        pub Value:Option<Decimal>,

        pub ClosePrice:Option<Decimal>,
    }
