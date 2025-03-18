    pub struct SLBFee
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

        pub Exchange:string,

        pub Quantity:Option<D256>,

        pub Code:string,

        pub ToAcct:string,

        pub FromAcct:string,

        pub Type:string,

        [Format(Constants.DateFormat)]
        pub ValueDate:Option<NaiveDateTime>,

        pub CollateralAmount:string,

        pub UniqueID:string,

        pub NetLendFee:Option<D256>,

        pub NetLendFeeRate:Option<D256>,

        pub GrossLendFee:Option<D256>,

        pub MarketFeeRate:Option<D256>,

        pub TotalCharges:Option<D256>,

        pub TicketCharge:Option<D256>,

        pub CarryCharge:Option<D256>,

        pub Fee:Option<D256>,

        pub FeeRate:Option<D256>,

        [Format(Constants.DateFormat)]
        pub StartDate:Option<NaiveDateTime>,
    }