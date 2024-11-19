    pub struct EquitySummaryByReportDateInBase
    {
        pub AccountId:string,

        pub AcctAlias:string,

        pub Model:string,

        //Note: The reportDate XML attribute may contain either a date or aString, i.e. reportDate="MULTI"
        pub ReportDate:string,

        pub Cash:Option<Decimal>,

        pub CashLong:Option<Decimal>,

        pub CashShort:Option<Decimal>,

        pub SlbCashCollateral:Option<Decimal>,

        pub SlbCashCollateralLong:Option<Decimal>,

        pub SlbCashCollateralShort:Option<Decimal>,

        pub Stock:Option<Decimal>,

        pub StockLong:Option<Decimal>,

        pub StockShort:Option<Decimal>,

        pub SlbDirectSecuritiesBorrowed:Option<Decimal>,

        pub SlbDirectSecuritiesBorrowedLong:Option<Decimal>,

        pub SlbDirectSecuritiesBorrowedShort:Option<Decimal>,

        pub SlbDirectSecuritiesLent:Option<Decimal>,

        pub SlbDirectSecuritiesLentLong:Option<Decimal>,

        pub SlbDirectSecuritiesLentShort:Option<Decimal>,

        pub Options:Option<Decimal>,

        pub OptionsLong:Option<Decimal>,

        pub OptionsShort:Option<Decimal>,

        pub Commodities:Option<Decimal>,

        pub CommoditiesLong:Option<Decimal>,

        pub CommoditiesShort:Option<Decimal>,

        pub Bonds:Option<Decimal>,

        pub BondsLong:Option<Decimal>,

        pub BondsShort:Option<Decimal>,

        pub Notes:Option<Decimal>,

        pub NotesLong:Option<Decimal>,

        pub NotesShort:Option<Decimal>,

        pub Funds:Option<Decimal>,

        pub FundsLong:Option<Decimal>,

        pub FundsShort:Option<Decimal>,

        pub InterestAccruals:Option<Decimal>,

        pub InterestAccrualsLong:Option<Decimal>,

        pub InterestAccrualsShort:Option<Decimal>,

        pub SoftDollars:Option<Decimal>,

        pub SoftDollarsLong:Option<Decimal>,

        pub SoftDollarsShort:Option<Decimal>,

        pub ForexCfdUnrealizedPl:Option<Decimal>,

        pub ForexCfdUnrealizedPlLong:Option<Decimal>,

        pub ForexCfdUnrealizedPlShort:Option<Decimal>,

        pub CfdUnrealizedPl:Option<Decimal>,

        pub CfdUnrealizedPlLong:Option<Decimal>,

        pub CfdUnrealizedPlShort:Option<Decimal>,

        pub DividendAccruals:Option<Decimal>,

        pub DividendAccrualsLong:Option<Decimal>,

        pub DividendAccrualsShort:Option<Decimal>,

        pub FdicInsuredBankSweepAccountCashComponent:Option<Decimal>,

        pub FdicInsuredBankSweepAccountCashComponentLong:Option<Decimal>,

        pub FdicInsuredBankSweepAccountCashComponentShort:Option<Decimal>,

        pub FdicInsuredAccountInterestAccrualsComponent:Option<Decimal>,

        pub FdicInsuredAccountInterestAccrualsComponentLong:Option<Decimal>,

        pub FdicInsuredAccountInterestAccrualsComponentShort:Option<Decimal>,

        pub Total:Option<Decimal>,

        pub TotalLong:Option<Decimal>,

        pub TotalShort:Option<Decimal>,
    }