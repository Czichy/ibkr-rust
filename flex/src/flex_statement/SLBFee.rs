    pub struct SLBFee
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

        pub Quantity:Option<Decimal>,

        pub Code:string,

        pub ToAcct:string,

        pub FromAcct:string,

        pub Type:string,

        [Format(Constants.DateFormat)]
        pub ValueDate:Option<NaiveDateTime>,

        pub CollateralAmount:string,

        pub UniqueID:string,

        pub NetLendFee:Option<Decimal>,

        pub NetLendFeeRate:Option<Decimal>,

        pub GrossLendFee:Option<Decimal>,

        pub MarketFeeRate:Option<Decimal>,

        pub TotalCharges:Option<Decimal>,

        pub TicketCharge:Option<Decimal>,

        pub CarryCharge:Option<Decimal>,

        pub Fee:Option<Decimal>,

        pub FeeRate:Option<Decimal>,

        [Format(Constants.DateFormat)]
        pub StartDate:Option<NaiveDateTime>,
    }