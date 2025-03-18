    pub struct EquitySummaryByReportDateInBase
    {
        pub AccountId:string,

        pub AcctAlias:string,

        pub Model:string,

        //Note: The reportDate XML attribute may contain either a date or aString, i.e. reportDate="MULTI"
        pub ReportDate:string,

        pub Cash:Option<D256>,

        pub CashLong:Option<D256>,

        pub CashShort:Option<D256>,

        pub SlbCashCollateral:Option<D256>,

        pub SlbCashCollateralLong:Option<D256>,

        pub SlbCashCollateralShort:Option<D256>,

        pub Stock:Option<D256>,

        pub StockLong:Option<D256>,

        pub StockShort:Option<D256>,

        pub SlbDirectSecuritiesBorrowed:Option<D256>,

        pub SlbDirectSecuritiesBorrowedLong:Option<D256>,

        pub SlbDirectSecuritiesBorrowedShort:Option<D256>,

        pub SlbDirectSecuritiesLent:Option<D256>,

        pub SlbDirectSecuritiesLentLong:Option<D256>,

        pub SlbDirectSecuritiesLentShort:Option<D256>,

        pub Options:Option<D256>,

        pub OptionsLong:Option<D256>,

        pub OptionsShort:Option<D256>,

        pub Commodities:Option<D256>,

        pub CommoditiesLong:Option<D256>,

        pub CommoditiesShort:Option<D256>,

        pub Bonds:Option<D256>,

        pub BondsLong:Option<D256>,

        pub BondsShort:Option<D256>,

        pub Notes:Option<D256>,

        pub NotesLong:Option<D256>,

        pub NotesShort:Option<D256>,

        pub Funds:Option<D256>,

        pub FundsLong:Option<D256>,

        pub FundsShort:Option<D256>,

        pub InterestAccruals:Option<D256>,

        pub InterestAccrualsLong:Option<D256>,

        pub InterestAccrualsShort:Option<D256>,

        pub SoftDollars:Option<D256>,

        pub SoftDollarsLong:Option<D256>,

        pub SoftDollarsShort:Option<D256>,

        pub ForexCfdUnrealizedPl:Option<D256>,

        pub ForexCfdUnrealizedPlLong:Option<D256>,

        pub ForexCfdUnrealizedPlShort:Option<D256>,

        pub CfdUnrealizedPl:Option<D256>,

        pub CfdUnrealizedPlLong:Option<D256>,

        pub CfdUnrealizedPlShort:Option<D256>,

        pub DividendAccruals:Option<D256>,

        pub DividendAccrualsLong:Option<D256>,

        pub DividendAccrualsShort:Option<D256>,

        pub FdicInsuredBankSweepAccountCashComponent:Option<D256>,

        pub FdicInsuredBankSweepAccountCashComponentLong:Option<D256>,

        pub FdicInsuredBankSweepAccountCashComponentShort:Option<D256>,

        pub FdicInsuredAccountInterestAccrualsComponent:Option<D256>,

        pub FdicInsuredAccountInterestAccrualsComponentLong:Option<D256>,

        pub FdicInsuredAccountInterestAccrualsComponentShort:Option<D256>,

        pub Total:Option<D256>,

        pub TotalLong:Option<D256>,

        pub TotalShort:Option<D256>,
    }