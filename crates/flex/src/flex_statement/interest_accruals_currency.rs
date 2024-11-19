    pub struct InterestAccrualsCurrency
    {
        [Format(Constants.DateFormat)]
        pub ToDate:Option<NaiveDateTime>,

        [Format(Constants.DateFormat)]
        pub FromDate:Option<NaiveDateTime>,

        pub AccountId:string,

        pub AcctAlias:string,

        pub Model:string,

        //Note: IB does not use a standard currency code here.  It is a value like BASE_SUMMARY.
        pub Currency:string,

        pub EndingAccrualBalance:Option<Decimal>,

        pub FxTranslation:Option<Decimal>,

        pub AccrualReversal:Option<Decimal>,

        pub InterestAccrued:Option<Decimal>,

        pub StartingAccrualBalance:Option<Decimal>,
    }