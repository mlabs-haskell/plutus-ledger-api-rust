//! Types related to Cardano transactions.
use super::{
    address::{Address, StakingCredential},
    crypto::{LedgerBytes, PaymentPubKeyHash},
    datum::{Datum, DatumHash},
    interval::PlutusInterval,
    value::{CurrencySymbol, Value},
};
use crate::plutus_data::{
    parse_constr, parse_constr_with_tag, parse_fixed_len_constr_fields, verify_constr_fields,
    IsPlutusData, PlutusData, PlutusDataError, PlutusType,
};
use crate::utils::{none, singleton};
use num_bigint::BigInt;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// An input of a transaction
///
/// Also know as `TxOutRef` from Plutus, this identifies a UTxO by its transacton hash and index
/// inside the transaction
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TransactionInput {
    pub transaction_id: TransactionHash,
    pub index: BigInt,
}

impl IsPlutusData for TransactionInput {
    fn to_plutus_data(&self) -> PlutusData {
        PlutusData::Constr(
            BigInt::from(0),
            vec![
                self.transaction_id.to_plutus_data(),
                self.index.to_plutus_data(),
            ],
        )
    }

    fn from_plutus_data(data: &PlutusData) -> Result<Self, PlutusDataError> {
        match data {
            PlutusData::Constr(flag, fields) => match u32::try_from(flag) {
                Ok(0) => {
                    verify_constr_fields(&fields, 2)?;
                    Ok(TransactionInput {
                        transaction_id: TransactionHash::from_plutus_data(&fields[0])?,
                        index: BigInt::from_plutus_data(&fields[1])?,
                    })
                }
                _ => Err(PlutusDataError::UnexpectedPlutusInvariant {
                    wanted: "Constr field between 0 and 1".to_owned(),
                    got: flag.to_string(),
                }),
            },

            _ => Err(PlutusDataError::UnexpectedPlutusType {
                wanted: PlutusType::Constr,
                got: PlutusType::from(data),
            }),
        }
    }
}

/// 32-bytes blake2b256 hash of a transaction body.
///
/// Also known as Transaction ID or `TxID`.
/// Note: Plutus docs might incorrectly state that it uses SHA256.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TransactionHash(pub LedgerBytes);

impl IsPlutusData for TransactionHash {
    fn to_plutus_data(&self) -> PlutusData {
        PlutusData::Constr(BigInt::from(0), vec![self.0.to_plutus_data()])
    }

    fn from_plutus_data(data: &PlutusData) -> Result<Self, PlutusDataError> {
        match data {
            PlutusData::Constr(flag, fields) => match u32::try_from(flag) {
                Ok(0) => {
                    verify_constr_fields(&fields, 1)?;
                    Ok(TransactionHash(IsPlutusData::from_plutus_data(&fields[0])?))
                }
                _ => Err(PlutusDataError::UnexpectedPlutusInvariant {
                    wanted: "Constr field to be 0".to_owned(),
                    got: flag.to_string(),
                }),
            },

            _ => Err(PlutusDataError::UnexpectedPlutusType {
                wanted: PlutusType::Constr,
                got: PlutusType::from(data),
            }),
        }
    }
}

/// An output of a transaction
///
/// This must include the target address, the hash of the datum attached, and the amount of output
/// tokens
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TransactionOutput {
    pub address: Address,
    pub value: Value,
    pub datum_hash: Option<DatumHash>,
}

impl IsPlutusData for TransactionOutput {
    fn to_plutus_data(&self) -> PlutusData {
        PlutusData::Constr(
            BigInt::from(0),
            vec![
                self.address.to_plutus_data(),
                self.value.to_plutus_data(),
                self.datum_hash.to_plutus_data(),
            ],
        )
    }

    fn from_plutus_data(data: &PlutusData) -> Result<Self, PlutusDataError> {
        match data {
            PlutusData::Constr(flag, fields) => match u32::try_from(flag) {
                Ok(0) => {
                    verify_constr_fields(&fields, 3)?;
                    Ok(TransactionOutput {
                        address: Address::from_plutus_data(&fields[0])?,
                        value: Value::from_plutus_data(&fields[1])?,
                        datum_hash: <Option<DatumHash>>::from_plutus_data(&fields[2])?,
                    })
                }
                _ => Err(PlutusDataError::UnexpectedPlutusInvariant {
                    wanted: "Constr field to be 0".to_owned(),
                    got: flag.to_string(),
                }),
            },

            _ => Err(PlutusDataError::UnexpectedPlutusType {
                wanted: PlutusType::Constr,
                got: PlutusType::from(data),
            }),
        }
    }
}

/// POSIX time is measured as the number of milliseconds since 1970-01-01T00:00:00Z
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct POSIXTime(pub BigInt);

impl IsPlutusData for POSIXTime {
    fn to_plutus_data(&self) -> PlutusData {
        self.0.to_plutus_data()
    }

    fn from_plutus_data(data: &PlutusData) -> Result<Self, PlutusDataError> {
        IsPlutusData::from_plutus_data(data).map(Self)
    }
}

#[cfg(feature = "chrono")]
#[derive(thiserror::Error, Debug)]
pub enum POSIXTimeConversionError {
    #[error(transparent)]
    TryFromBigIntError(#[from] num_bigint::TryFromBigIntError<BigInt>),
    #[error("POSIXTime is out of bounds.")]
    OutOfBoundsError,
}

#[cfg(feature = "chrono")]
impl<Tz: chrono::TimeZone> From<chrono::DateTime<Tz>> for POSIXTime {
    fn from(datetime: chrono::DateTime<Tz>) -> POSIXTime {
        POSIXTime(BigInt::from(datetime.timestamp_millis()))
    }
}

#[cfg(feature = "chrono")]
impl TryFrom<POSIXTime> for chrono::DateTime<chrono::Utc> {
    type Error = POSIXTimeConversionError;

    fn try_from(posix_time: POSIXTime) -> Result<chrono::DateTime<chrono::Utc>, Self::Error> {
        let POSIXTime(millis) = posix_time;
        Ok(chrono::DateTime::from_timestamp_millis(
            <i64>::try_from(millis).map_err(POSIXTimeConversionError::TryFromBigIntError)?,
        )
        .ok_or(POSIXTimeConversionError::OutOfBoundsError)?)
    }
}

pub type POSIXTimeRange = PlutusInterval<POSIXTime>;

/// An input of a pending transaction.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TxInInfo {
    pub reference: TransactionInput,
    pub output: TransactionOutput,
}

impl IsPlutusData for TxInInfo {
    fn to_plutus_data(&self) -> PlutusData {
        PlutusData::Constr(
            BigInt::from(0),
            vec![
                self.reference.to_plutus_data(),
                self.output.to_plutus_data(),
            ],
        )
    }

    fn from_plutus_data(data: &PlutusData) -> Result<Self, PlutusDataError> {
        match data {
            PlutusData::Constr(flag, fields) => match u32::try_from(flag) {
                Ok(0) => {
                    verify_constr_fields(&fields, 2)?;
                    Ok(TxInInfo {
                        reference: TransactionInput::from_plutus_data(&fields[0])?,
                        output: TransactionOutput::from_plutus_data(&fields[1])?,
                    })
                }
                _ => Err(PlutusDataError::UnexpectedPlutusInvariant {
                    wanted: "Constr field to be 0".to_owned(),
                    got: flag.to_string(),
                }),
            },

            _ => Err(PlutusDataError::UnexpectedPlutusType {
                wanted: PlutusType::Constr,
                got: PlutusType::from(data),
            }),
        }
    }
}

impl From<(TransactionInput, TransactionOutput)> for TxInInfo {
    fn from((reference, output): (TransactionInput, TransactionOutput)) -> TxInInfo {
        TxInInfo { reference, output }
    }
}

/// Partial representation of digests of certificates on the ledger.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum DCert {
    DelegRegKey(StakingCredential),
    DelegDeRegKey(StakingCredential),
    DelegDelegate(
        /// Delegator
        StakingCredential,
        /// Delegatee
        PaymentPubKeyHash,
    ),
    /// A digest of the PoolParam
    PoolRegister(
        /// Pool id
        PaymentPubKeyHash,
        /// Pool VFR
        PaymentPubKeyHash,
    ),
    PoolRetire(
        PaymentPubKeyHash,
        /// Epoch
        BigInt,
    ),
    Genesis,
    Mir,
}

impl IsPlutusData for DCert {
    fn to_plutus_data(&self) -> PlutusData {
        let (tag, fields) = match self {
            DCert::DelegRegKey(c) => (0u32, singleton(c.to_plutus_data())),
            DCert::DelegDeRegKey(c) => (1, singleton(c.to_plutus_data())),
            DCert::DelegDelegate(c, pkh) => (2, vec![c.to_plutus_data(), pkh.to_plutus_data()]),
            DCert::PoolRegister(pkh, pkh1) => {
                (3, vec![pkh.to_plutus_data(), pkh1.to_plutus_data()])
            }
            DCert::PoolRetire(pkh, i) => (4, vec![pkh.to_plutus_data(), i.to_plutus_data()]),
            DCert::Genesis => (5, none()),
            DCert::Mir => (6, none()),
        };

        PlutusData::Constr(BigInt::from(tag), fields)
    }

    fn from_plutus_data(data: &PlutusData) -> Result<Self, PlutusDataError> {
        let (tag, fields) = parse_constr(data)?;

        match tag {
            0 => {
                let [field] = parse_fixed_len_constr_fields::<1>(fields)?;
                IsPlutusData::from_plutus_data(field).map(Self::DelegRegKey)
            }
            1 => {
                let [field] = parse_fixed_len_constr_fields::<1>(fields)?;
                IsPlutusData::from_plutus_data(field).map(Self::DelegDeRegKey)
            }
            2 => {
                let [field1, field2] = parse_fixed_len_constr_fields::<2>(fields)?;
                Ok(Self::DelegDelegate(
                    IsPlutusData::from_plutus_data(field1)?,
                    IsPlutusData::from_plutus_data(field2)?,
                ))
            }
            3 => {
                let [field1, field2] = parse_fixed_len_constr_fields::<2>(fields)?;
                Ok(Self::PoolRegister(
                    IsPlutusData::from_plutus_data(field1)?,
                    IsPlutusData::from_plutus_data(field2)?,
                ))
            }
            4 => {
                let [field1, field2] = parse_fixed_len_constr_fields::<2>(fields)?;
                Ok(Self::PoolRetire(
                    IsPlutusData::from_plutus_data(field1)?,
                    IsPlutusData::from_plutus_data(field2)?,
                ))
            }
            5 => {
                let [] = parse_fixed_len_constr_fields::<0>(fields)?;
                Ok(Self::Genesis)
            }
            6 => {
                let [] = parse_fixed_len_constr_fields::<0>(fields)?;
                Ok(Self::Mir)
            }
            bad_tag => Err(PlutusDataError::UnexpectedPlutusInvariant {
                wanted: "Constr tag to be 0, 1, 2, 3, 4, 5 or 6".to_owned(),
                got: bad_tag.to_string(),
            }),
        }
    }
}

/// The purpose of the script that's currently running.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ScriptPurpose {
    Minting(CurrencySymbol),
    Spending(TransactionInput),
    Rewarding(StakingCredential),
    Certifying(DCert),
}

impl IsPlutusData for ScriptPurpose {
    fn to_plutus_data(&self) -> PlutusData {
        let (tag, field) = match self {
            ScriptPurpose::Minting(cs) => (0u32, cs.to_plutus_data()),
            ScriptPurpose::Spending(i) => (1, i.to_plutus_data()),
            ScriptPurpose::Rewarding(c) => (2, c.to_plutus_data()),
            ScriptPurpose::Certifying(c) => (3, c.to_plutus_data()),
        };

        PlutusData::Constr(BigInt::from(tag), singleton(field))
    }

    fn from_plutus_data(plutus_data: &PlutusData) -> Result<Self, PlutusDataError> {
        let (tag, fields) = parse_constr(plutus_data)?;
        let [field] = parse_fixed_len_constr_fields(fields)?;

        match tag {
            0 => IsPlutusData::from_plutus_data(field).map(Self::Minting),
            1 => IsPlutusData::from_plutus_data(field).map(Self::Spending),
            2 => IsPlutusData::from_plutus_data(field).map(Self::Rewarding),
            3 => IsPlutusData::from_plutus_data(field).map(Self::Certifying),
            bad_tag => Err(PlutusDataError::UnexpectedPlutusInvariant {
                got: bad_tag.to_string(),
                wanted: format!("Constr tag to be 0, 1, 2 or 3"),
            }),
        }
    }
}

/// A pending transaction as seen by validator scripts, also known as TxInfo in Plutus
#[derive(Debug, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TransactionInfo {
    pub inputs: Vec<TxInInfo>,
    pub outputs: Vec<TransactionOutput>,
    pub fee: Value,
    pub mint: Value,
    pub d_cert: Vec<DCert>,
    pub wdrl: Vec<(StakingCredential, BigInt)>,
    pub valid_range: POSIXTimeRange,
    pub signatories: Vec<PaymentPubKeyHash>,
    pub datums: Vec<(DatumHash, Datum)>,
    pub id: TransactionHash,
}

impl IsPlutusData for TransactionInfo {
    fn to_plutus_data(&self) -> PlutusData {
        PlutusData::Constr(
            BigInt::from(0),
            vec![
                self.inputs.to_plutus_data(),
                self.outputs.to_plutus_data(),
                self.fee.to_plutus_data(),
                self.mint.to_plutus_data(),
                self.d_cert.to_plutus_data(),
                self.wdrl.to_plutus_data(),
                self.valid_range.to_plutus_data(),
                self.signatories.to_plutus_data(),
                self.datums.to_plutus_data(),
                self.id.to_plutus_data(),
            ],
        )
    }

    fn from_plutus_data(data: &PlutusData) -> Result<Self, PlutusDataError> {
        let fields = parse_constr_with_tag(data, 0)?;
        let [inputs, outputs, fee, mint, d_cert, wdrl, valid_range, signatories, datums, id] =
            parse_fixed_len_constr_fields(fields)?;

        Ok(Self {
            inputs: IsPlutusData::from_plutus_data(inputs)?,
            outputs: IsPlutusData::from_plutus_data(outputs)?,
            fee: IsPlutusData::from_plutus_data(fee)?,
            mint: IsPlutusData::from_plutus_data(mint)?,
            d_cert: IsPlutusData::from_plutus_data(d_cert)?,
            wdrl: IsPlutusData::from_plutus_data(wdrl)?,
            valid_range: IsPlutusData::from_plutus_data(valid_range)?,
            signatories: IsPlutusData::from_plutus_data(signatories)?,
            datums: IsPlutusData::from_plutus_data(datums)?,
            id: IsPlutusData::from_plutus_data(id)?,
        })
    }
}

/// The context that is presented to the currently-executing script.
#[derive(Debug, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ScriptContext {
    pub tx_info: TransactionInfo,
    pub purpose: ScriptPurpose,
}

impl IsPlutusData for ScriptContext {
    fn to_plutus_data(&self) -> PlutusData {
        PlutusData::Constr(
            BigInt::from(0),
            vec![self.tx_info.to_plutus_data(), self.purpose.to_plutus_data()],
        )
    }

    fn from_plutus_data(data: &PlutusData) -> Result<Self, PlutusDataError> {
        let fields = parse_constr_with_tag(data, 0)?;
        let [tx_info, purpose] = parse_fixed_len_constr_fields(fields)?;

        Ok(Self {
            tx_info: IsPlutusData::from_plutus_data(tx_info)?,
            purpose: IsPlutusData::from_plutus_data(purpose)?,
        })
    }
}
