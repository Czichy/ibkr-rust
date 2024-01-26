    pub struct TierInterestDetail
    {
        pub AccountId:string,

        pub AcctAlias:string,

        pub Model:string,

        pub Currency:Option<Currency>,

        pub FxRateToBase:Option<Decimal>,

        pub Code:string,

        pub ToAcct:string,

        pub FromAcct:string,

        pub TotalInterest:Option<Decimal>,

        pub IbuklInterest:Option<Decimal>,

        pub CommoditiesInterest:Option<Decimal>,

        pub SecuritiesInterest:Option<Decimal>,

        pub Rate:Option<Decimal>,

        pub TotalPrincipal:Option<Decimal>,

        pub IbuklPrincipal:Option<Decimal>,

        pub CommoditiesPrincipal:Option<Decimal>,

        pub SecuritiesPrincipal:Option<Decimal>,

        pub BalanceThreshold:Option<Decimal>,

        pub TierBreak:string,

        [Format(Constants.DateFormat)]
        pub ValueDate:Option<NaiveDateTime>,

        pub InterestType:string,
    }