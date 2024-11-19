use std::{fmt::{Display, Formatter},
          str::FromStr};

use serde::{Deserialize, Deserializer};

#[derive(Debug)]
pub struct ParseEnumError;
impl Display for ParseEnumError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "Unknown Enum value")
    }
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub enum BuySell {
    #[serde(rename = "BUY")]
    Buy,
    #[serde(rename = "SELL")]
    Sell,
    #[serde(rename = "SELL (Ca.)")]
    SellCa,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub enum LongShort {
    #[serde(rename = "Long")]
    Long,
    #[serde(rename = "Short")]
    Short,
}

#[derive(Debug, Deserialize, Clone, Eq, PartialEq)]
pub enum PutCall {
    P,
    C,
}
#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub enum OpenClose {
    O,
    C,
}
pub(crate) fn open_close_deserialize<'de, D>(
    deserializer: D,
) -> core::result::Result<Option<OpenClose>, D::Error>
where
    D: Deserializer<'de>,
{
    let str_sequence = String::deserialize(deserializer)?;
    Ok(match str_sequence.as_ref() {
        "O" => Some(OpenClose::O),
        "C" => Some(OpenClose::C),
        _ => None,
    })
}

#[derive(Debug, Deserialize, Clone, Eq, PartialEq)]
pub enum AssetCategory {
    STK,
    OPT,
    FOP,
    CFD,
    FUT,
    CASH,
    FXCFD,
    BOND,
}
impl Display for AssetCategory {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result { write!(f, "{:?}", self) }
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
pub enum SecIdType {
    Isin,
    Cusip,
}
impl FromStr for SecIdType {
    type Err = ParseEnumError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ISIN" => Ok(Self::Isin),
            "CUSIP" => Ok(Self::Cusip),
            &_ => Err(ParseEnumError),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Notes {
    ADRFeeAccrual,
    AdjustedLossWashSale,
    Adjustment,
    Allocation,
    ArrangedByIB,
    ArrangedByIntroducingBroker,
    Assigned,
    AutoBuyIn,
    AutomaticalExercise,
    AwayTrade,
    CashDelivery,
    ClosingTrade,
    ComplexePosition,
    CorrectedTrade,
    CrossTrade,
    Deleted,
    Delisted,
    DirectLending,
    DirectLoan,
    DividendReinvestment,
    EtfCreation,
    ExecutedAgainstCompany,
    Exercised,
    Expired,
    GuaranteedAccountSegment,
    IBPrincipalForFractional,
    IBPrincipalForFractionalAgentWhole,
    IBRisklessPrincipalForFractional,
    IBRisklessPrincipalForFractionalAgentWhole,
    IPO,
    InterestDividendAccrualPosting,
    InternalTransfer,
    InvestmentTransferFromInvestor,
    InvestmentTransferToHedgeFund,
    LiFo,
    LongTermPL,
    MLG,
    MLL,
    MSG,
    MSL,
    ManualExercise,
    ManualThroughIB,
    MarginViolation,
    MaxTaxBase,
    MaximizeLoss,
    OpeningTrade,
    PI,
    PartialExecution,
    PrincipalTrade,
    RedemptionForHedgeFund,
    RedemptionToInvestor,
    Refund,
    SL,
    ShortTermPL,
    ShortendedExecution,
    StockYieldEligible,
    Transfer,
    TransformationInterestDividend,
    UnvestedSharesFromStockGrant,
}

impl FromStr for Notes {
    type Err = ParseEnumError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let res = match s {
            "A" => Notes::Assigned,
            "AEx" => Notes::AutomaticalExercise,
            "Adj" => Notes::Adjustment,
            "Al" => Notes::Allocation,
            "Aw" => Notes::AwayTrade,
            "B" => Notes::AutoBuyIn,
            "Bo" => Notes::DirectLending,
            "C" => Notes::ClosingTrade,
            "CD" => Notes::CashDelivery,
            "CP" => Notes::ComplexePosition,
            "Ca" => Notes::Deleted,
            "Co" => Notes::CorrectedTrade,
            "Cx" => Notes::CrossTrade,
            "ETF" => Notes::EtfCreation,
            "Ep" => Notes::Expired,
            "Ex" => Notes::Exercised,
            "G" => Notes::GuaranteedAccountSegment,
            "HC" => Notes::MaxTaxBase,
            "HFI" => Notes::InvestmentTransferToHedgeFund,
            "HFR" => Notes::RedemptionForHedgeFund,
            "I" => Notes::InternalTransfer,
            "IA" => Notes::ExecutedAgainstCompany,
            "INV" => Notes::InvestmentTransferFromInvestor,
            "IPO" => Notes::IPO,
            "L" => Notes::MarginViolation,
            "LD" => Notes::AdjustedLossWashSale,
            "LI" => Notes::LiFo,
            "LT" => Notes::LongTermPL,
            "Lo" => Notes::DirectLoan,
            "M" => Notes::ManualThroughIB,
            "MEx" => Notes::ManualExercise,
            "ML" => Notes::MaximizeLoss,
            "MLG" => Notes::MLG,
            "MLL" => Notes::MLL,
            "MSG" => Notes::MSG,
            "MSL" => Notes::MSL,
            "O" => Notes::OpeningTrade,
            "P" => Notes::PartialExecution,
            "PI" => Notes::PI,
            "Po" => Notes::InterestDividendAccrualPosting,
            "Pr" => Notes::PrincipalTrade,
            "R" => Notes::DividendReinvestment,
            "RED" => Notes::RedemptionToInvestor,
            "Re" => Notes::TransformationInterestDividend,
            "Ri" => Notes::Refund,
            "SI" => Notes::ArrangedByIB,
            "SL" => Notes::SL,
            "SO" => Notes::ArrangedByIntroducingBroker,
            "SS" => Notes::ShortendedExecution,
            "ST" => Notes::ShortTermPL,
            "SY" => Notes::StockYieldEligible,
            "T" => Notes::Transfer,
            "ADR" => Notes::ADRFeeAccrual,
            "FP" => Notes::IBPrincipalForFractional,
            "FPA" => Notes::IBPrincipalForFractionalAgentWhole,
            "RP" => Notes::IBRisklessPrincipalForFractional,
            "RPA" => Notes::IBRisklessPrincipalForFractionalAgentWhole,
            "U" => Notes::UnvestedSharesFromStockGrant,
            "D" => Notes::Delisted,
            &_ => return Err(ParseEnumError),
        };
        Ok(res)
    }
}
pub(crate) fn notes_deserialize<'de, D>(
    deserializer: D,
) -> core::result::Result<Vec<Notes>, D::Error>
where
    D: Deserializer<'de>,
{
    let str_sequence = String::deserialize(deserializer)?;
    Ok(str_sequence
        .split(';')
        .map(Notes::from_str)
        .filter_map(|f| f.ok())
        //.flatten()
        .collect())
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize)]
pub enum OrderType {
    NoOrderType, // only legit for deltaNeutralOrderType
    Limit,
    Market,
    MarketIfTouched,
    MarketOnClose,
    MarketOnOpen,
    PeggedToMarket,
    PeggedToStock,
    PeggedToPrimary,
    BoxTop,
    LimitIfTouched,
    LimitOnClose,
    PassiveRelative,
    PeggedToMidpoint,
    MarketToLimit,
    MarketWithProtection,
    MidPrice,
    Stop,
    StopLimit,
    StopWithProtection,
    TrailingStop,
    TrailingStopLimit,
    RelativeLimit,
    RelativeMarket,
    Volatility,
    PeggedToBenchmark,
}

impl FromStr for OrderType {
    type Err = ParseEnumError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "" => Ok(OrderType::NoOrderType),
            "None" => Ok(OrderType::NoOrderType),
            "LMT" => Ok(OrderType::Limit),
            "MKT" => Ok(OrderType::Market),
            "MIT" => Ok(OrderType::MarketIfTouched),
            "MOC" => Ok(OrderType::MarketOnClose),
            "MOO" => Ok(OrderType::MarketOnOpen),
            "PEGMKT" => Ok(OrderType::PeggedToMarket),
            "PEGSTK" => Ok(OrderType::PeggedToStock),
            "REL" => Ok(OrderType::PeggedToPrimary),
            "BOXTOP" => Ok(OrderType::BoxTop),
            "LIT" => Ok(OrderType::LimitIfTouched),
            "LOC" => Ok(OrderType::LimitOnClose),
            "PASSVREL" => Ok(OrderType::PassiveRelative),
            "PEGMID" => Ok(OrderType::PeggedToMidpoint),
            "MTL" => Ok(OrderType::MarketToLimit),
            "MKTPRT" => Ok(OrderType::MarketWithProtection),
            "MIDPRICE" => Ok(OrderType::MidPrice),
            "MIDPX" => Ok(OrderType::MidPrice),
            "STP" => Ok(OrderType::Stop),
            "STPLMT" => Ok(OrderType::StopLimit),
            "STPPRT" => Ok(OrderType::StopWithProtection),
            "TRAIL" => Ok(OrderType::TrailingStop),
            "TRAILLIMIT" => Ok(OrderType::TrailingStopLimit),
            "REL + LMT" => Ok(OrderType::RelativeLimit),
            "REL + MKT" => Ok(OrderType::RelativeMarket),
            "VOL" => Ok(OrderType::Volatility),
            "PEG BENCH" => Ok(OrderType::PeggedToBenchmark),
            &_ => Err(ParseEnumError),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize)]
pub enum ActionType {
    AssetPurchase,
    BondConversion,
    BondMaturity,
    CashDividend,
    ChoiceDividendDelivery,
    ChoiceDividendIssue,
    ContactConsolidation,
    ContractSoulte,
    ContractSpinOff,
    ContractSplit,
    ConvertibleIssue,
    CouponPayment,
    DelistWorthless,
    DividendRightsIssue,
    ExpireDividendRight,
    FeeAllocation,
    ForwardSplit,
    GenericVoluntary,
    IssueChange,
    IssueForwardSplit,
    Merger,
    PartialCallIssue,
    ReverseSplit,
    SharePurchaseIssue,
    SpinOff,
    StockDividend,
    SubscribableRightsIssue,
    SubscribeRights,
    TbillMaturity,
    TenderIssue,
    UnknownEvent,
    VoluntaryConversion,
}

impl FromStr for ActionType {
    type Err = ParseEnumError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "OR" => Ok(ActionType::AssetPurchase),
            "BC" => Ok(ActionType::BondConversion),
            "BM" => Ok(ActionType::BondMaturity),
            "CD" => Ok(ActionType::CashDividend),
            "HD" => Ok(ActionType::ChoiceDividendDelivery),
            "HI" => Ok(ActionType::ChoiceDividendIssue),
            "CC" => Ok(ActionType::ContactConsolidation),
            "CA" => Ok(ActionType::ContractSoulte),
            "CO" => Ok(ActionType::ContractSpinOff),
            "CS" => Ok(ActionType::ContractSplit),
            "CI" => Ok(ActionType::ConvertibleIssue),
            "CP" => Ok(ActionType::CouponPayment),
            "DW" => Ok(ActionType::DelistWorthless),
            "DI" => Ok(ActionType::DividendRightsIssue),
            "ED" => Ok(ActionType::ExpireDividendRight),
            "FA" => Ok(ActionType::FeeAllocation),
            "FS" => Ok(ActionType::ForwardSplit),
            "GV" => Ok(ActionType::GenericVoluntary),
            "IC" => Ok(ActionType::IssueChange),
            "FI" => Ok(ActionType::IssueForwardSplit),
            "TC" => Ok(ActionType::Merger),
            "PC" => Ok(ActionType::PartialCallIssue),
            "RS" => Ok(ActionType::ReverseSplit),
            "PI" => Ok(ActionType::SharePurchaseIssue),
            "SO" => Ok(ActionType::SpinOff),
            "SD" => Ok(ActionType::StockDividend),
            "RI" => Ok(ActionType::SubscribableRightsIssue),
            "SR" => Ok(ActionType::SubscribeRights),
            "TM" => Ok(ActionType::TbillMaturity),
            "TI" => Ok(ActionType::TenderIssue),
            "UE" => Ok(ActionType::UnknownEvent),
            "TO" => Ok(ActionType::VoluntaryConversion),
            &_ => Err(ParseEnumError),
        }
    }
}
