use flat::de;
use strum_macros::EnumString;

/// All the possible builtin functions in Untyped Plutus Core.
#[repr(u8)]
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, EnumString, PartialEq, Copy)]
#[strum(serialize_all = "camelCase")]
pub enum DefaultFunction {
    // Integer functions
    AddInteger = 0,
    SubtractInteger = 1,
    MultiplyInteger = 2,
    DivideInteger = 3,
    QuotientInteger = 4,
    RemainderInteger = 5,
    ModInteger = 6,
    EqualsInteger = 7,
    LessThanInteger = 8,
    LessThanEqualsInteger = 9,
    // ByteString functions
    AppendByteString = 10,
    ConsByteString = 11,
    SliceByteString = 12,
    LengthOfByteString = 13,
    IndexByteString = 14,
    EqualsByteString = 15,
    LessThanByteString = 16,
    LessThanEqualsByteString = 17,
    // Cryptography and hash functions
    #[strum(serialize = "sha2_256")]
    Sha2_256 = 18,
    Sha3_256 = 19,
    Blake2b_256 = 20,
    VerifySignature = 21,
    VerifyEcdsaSecp256k1Signature = 52,
    VerifySchnorrSecp256k1Signature = 53,
    // String functions
    AppendString = 22,
    EqualsString = 23,
    EncodeUtf8 = 24,
    DecodeUtf8 = 25,
    // Bool function
    IfThenElse = 26,
    // Unit function
    ChooseUnit = 27,
    // Tracing function
    Trace = 28,
    // Pairs functions
    FstPair = 29,
    SndPair = 30,
    // List functions
    ChooseList = 31,
    MkCons = 32,
    HeadList = 33,
    TailList = 34,
    NullList = 35,
    // Data functions
    // It is convenient to have a "choosing" function for a data type that has more than two
    // constructors to get pattern matching over it and we may end up having multiple such data
    // types, hence we include the name of the data type as a suffix.
    ChooseData = 36,
    ConstrData = 37,
    MapData = 38,
    ListData = 39,
    IData = 40,
    BData = 41,
    UnConstrData = 42,
    UnMapData = 43,
    UnListData = 44,
    UnIData = 45,
    UnBData = 46,
    EqualsData = 47,
    SerialiseData = 51,
    // Misc constructors
    // Constructors that we need for constructing e.g. Data. Polymorphic builtin
    // constructors are often problematic (See note [Representable built-in
    // functions over polymorphic built-in types])
    MkPairData = 48,
    MkNilData = 49,
    MkNilPairData = 50,
}

impl TryFrom<u8> for DefaultFunction {
    type Error = de::Error;

    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v {
            v if v == DefaultFunction::AddInteger as u8 => Ok(DefaultFunction::AddInteger),
            v if v == DefaultFunction::SubtractInteger as u8 => {
                Ok(DefaultFunction::SubtractInteger)
            }
            v if v == DefaultFunction::MultiplyInteger as u8 => {
                Ok(DefaultFunction::MultiplyInteger)
            }
            v if v == DefaultFunction::DivideInteger as u8 => Ok(DefaultFunction::DivideInteger),
            v if v == DefaultFunction::QuotientInteger as u8 => {
                Ok(DefaultFunction::QuotientInteger)
            }
            v if v == DefaultFunction::RemainderInteger as u8 => {
                Ok(DefaultFunction::RemainderInteger)
            }
            v if v == DefaultFunction::ModInteger as u8 => Ok(DefaultFunction::ModInteger),
            v if v == DefaultFunction::EqualsInteger as u8 => Ok(DefaultFunction::EqualsInteger),
            v if v == DefaultFunction::LessThanInteger as u8 => {
                Ok(DefaultFunction::LessThanInteger)
            }
            v if v == DefaultFunction::LessThanEqualsInteger as u8 => {
                Ok(DefaultFunction::LessThanEqualsInteger)
            }
            // ByteString functions
            v if v == DefaultFunction::AppendByteString as u8 => {
                Ok(DefaultFunction::AppendByteString)
            }
            v if v == DefaultFunction::ConsByteString as u8 => Ok(DefaultFunction::ConsByteString),
            v if v == DefaultFunction::SliceByteString as u8 => {
                Ok(DefaultFunction::SliceByteString)
            }
            v if v == DefaultFunction::LengthOfByteString as u8 => {
                Ok(DefaultFunction::LengthOfByteString)
            }
            v if v == DefaultFunction::IndexByteString as u8 => {
                Ok(DefaultFunction::IndexByteString)
            }
            v if v == DefaultFunction::EqualsByteString as u8 => {
                Ok(DefaultFunction::EqualsByteString)
            }
            v if v == DefaultFunction::LessThanByteString as u8 => {
                Ok(DefaultFunction::LessThanByteString)
            }
            v if v == DefaultFunction::LessThanEqualsByteString as u8 => {
                Ok(DefaultFunction::LessThanEqualsByteString)
            }
            // Cryptography and hash functions
            v if v == DefaultFunction::Sha2_256 as u8 => Ok(DefaultFunction::Sha2_256),
            v if v == DefaultFunction::Sha3_256 as u8 => Ok(DefaultFunction::Sha3_256),
            v if v == DefaultFunction::Blake2b_256 as u8 => Ok(DefaultFunction::Blake2b_256),
            v if v == DefaultFunction::VerifySignature as u8 => {
                Ok(DefaultFunction::VerifySignature)
            }
            v if v == DefaultFunction::VerifyEcdsaSecp256k1Signature as u8 => {
                Ok(DefaultFunction::VerifyEcdsaSecp256k1Signature)
            }
            v if v == DefaultFunction::VerifySchnorrSecp256k1Signature as u8 => {
                Ok(DefaultFunction::VerifySchnorrSecp256k1Signature)
            }
            // String functions
            v if v == DefaultFunction::AppendString as u8 => Ok(DefaultFunction::AppendString),
            v if v == DefaultFunction::EqualsString as u8 => Ok(DefaultFunction::EqualsString),
            v if v == DefaultFunction::EncodeUtf8 as u8 => Ok(DefaultFunction::EncodeUtf8),
            v if v == DefaultFunction::DecodeUtf8 as u8 => Ok(DefaultFunction::DecodeUtf8),
            // Bool function
            v if v == DefaultFunction::IfThenElse as u8 => Ok(DefaultFunction::IfThenElse),
            // Unit function
            v if v == DefaultFunction::ChooseUnit as u8 => Ok(DefaultFunction::ChooseUnit),
            // Tracing function
            v if v == DefaultFunction::Trace as u8 => Ok(DefaultFunction::Trace),
            // Pairs functions
            v if v == DefaultFunction::FstPair as u8 => Ok(DefaultFunction::FstPair),
            v if v == DefaultFunction::SndPair as u8 => Ok(DefaultFunction::SndPair),
            // List functions
            v if v == DefaultFunction::ChooseList as u8 => Ok(DefaultFunction::ChooseList),
            v if v == DefaultFunction::MkCons as u8 => Ok(DefaultFunction::MkCons),
            v if v == DefaultFunction::HeadList as u8 => Ok(DefaultFunction::HeadList),
            v if v == DefaultFunction::TailList as u8 => Ok(DefaultFunction::TailList),
            v if v == DefaultFunction::NullList as u8 => Ok(DefaultFunction::NullList),
            // Data functions
            // It is convenient to have a "choosing" function for a data type that has more than two
            // constructors to get pattern matching over it and we may end up having multiple such data
            // types, hence we include the name of the data type as a suffix.
            v if v == DefaultFunction::ChooseData as u8 => Ok(DefaultFunction::ChooseData),
            v if v == DefaultFunction::ConstrData as u8 => Ok(DefaultFunction::ConstrData),
            v if v == DefaultFunction::MapData as u8 => Ok(DefaultFunction::MapData),
            v if v == DefaultFunction::ListData as u8 => Ok(DefaultFunction::ListData),
            v if v == DefaultFunction::IData as u8 => Ok(DefaultFunction::IData),
            v if v == DefaultFunction::BData as u8 => Ok(DefaultFunction::BData),
            v if v == DefaultFunction::UnConstrData as u8 => Ok(DefaultFunction::UnConstrData),
            v if v == DefaultFunction::UnMapData as u8 => Ok(DefaultFunction::UnMapData),
            v if v == DefaultFunction::UnListData as u8 => Ok(DefaultFunction::UnListData),
            v if v == DefaultFunction::UnIData as u8 => Ok(DefaultFunction::UnIData),
            v if v == DefaultFunction::UnBData as u8 => Ok(DefaultFunction::UnBData),
            v if v == DefaultFunction::EqualsData as u8 => Ok(DefaultFunction::EqualsData),
            v if v == DefaultFunction::SerialiseData as u8 => Ok(DefaultFunction::SerialiseData),
            // Misc constructors
            // Constructors that we need for constructing e.g. Data. Polymorphic builtin
            // constructors are often problematic (See note [Representable built-in
            // functions over polymorphic built-in types])
            v if v == DefaultFunction::MkPairData as u8 => Ok(DefaultFunction::MkPairData),
            v if v == DefaultFunction::MkNilData as u8 => Ok(DefaultFunction::MkNilData),
            v if v == DefaultFunction::MkNilPairData as u8 => Ok(DefaultFunction::MkNilPairData),
            _ => Err(de::Error::Message(format!(
                "Default Function not found - {}",
                v
            ))),
        }
    }
}
