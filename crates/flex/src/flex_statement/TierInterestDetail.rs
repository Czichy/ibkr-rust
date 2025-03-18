    pub struct TierInterestDetail
    {
        pub AccountId:string,

        pub AcctAlias:string,

        pub Model:string,

        pub Currency:Option<Currency>,

        pub FxRateToBase:Option<D256>,

        pub Code:string,

        pub ToAcct:string,

        pub FromAcct:string,

        pub TotalInterest:Option<D256>,

        pub IbuklInterest:Option<D256>,

        pub CommoditiesInterest:Option<D256>,

        pub SecuritiesInterest:Option<D256>,

        pub Rate:Option<D256>,

        pub TotalPrincipal:Option<D256>,

        pub IbuklPrincipal:Option<D256>,

        pub CommoditiesPrincipal:Option<D256>,

        pub SecuritiesPrincipal:Option<D256>,

        pub BalanceThreshold:Option<D256>,

        pub TierBreak:string,

        [Format(Constants.DateFormat)]
        pub ValueDate:Option<NaiveDateTime>,

        pub InterestType:string,
    }