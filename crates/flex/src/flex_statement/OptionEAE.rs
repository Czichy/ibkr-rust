    pub struct OptionEAE
    {
        pub AccountId:string,

        pub AcctAlias:string,

        pub Model:string,

        pub Currency:Option<Currency>,

        pub FxRateToBase:Option<D256>,

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

        pub Strike:Option<D256>,

        [Format(Constants.DateFormat)]
        pub Expiry:Option<NaiveDateTime>,

        pub PutCall:Option<PutCall>,

        pub PrincipalAdjustFactor:string,

        [Format(Constants.DateFormat)]
        pub Date:Option<NaiveDateTime>,

        pub TransactionType:string,

        pub Quantity:Option<D256>,

        pub TradePrice:Option<D256>,

        pub MarkPrice:Option<D256>,

        pub Proceeds:Option<D256>,

        pub CommisionsAndTax:Option<D256>,

        pub CostBasis:Option<D256>,

        pub RealizedPnl:Option<D256>,

        pub FxPnl:Option<D256>,

        pub MtmPnl:Option<D256>,

        pub TradeID:string,
    }