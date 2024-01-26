    pub struct ConversionRate
    {
        //Note: The reportDate XML attribute may contain either a date or aString, i.e. reportDate="MULTI"
        pub ReportDate:string,

        pub FromCurrency:Currency,

        pub ToCurrency:Currency,

        pub Rate:double,
    }