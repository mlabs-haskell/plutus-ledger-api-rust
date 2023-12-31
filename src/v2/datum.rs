//! Types related to Plutus Datums
use crate::plutus_data::{
    verify_constr_fields, IsPlutusData, PlutusData, PlutusDataError, PlutusType,
};
pub use crate::v1::datum::{Datum, DatumHash};
#[cfg(feature = "lbf")]
use lbr_prelude::json::{self, Error, Json};
use num_bigint::BigInt;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Optional datum of a transaction
///
/// In case an inline datum is used, the data is embedded inside the transaction body, so it can be
/// directly retrieved. In case of a datum hash, an off-chain indexer is required to find the
/// associated datum by its hash.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum OutputDatum {
    None,
    DatumHash(DatumHash),
    InlineDatum(Datum),
}

impl IsPlutusData for OutputDatum {
    fn to_plutus_data(&self) -> PlutusData {
        match self {
            OutputDatum::None => PlutusData::Constr(BigInt::from(0), vec![]),
            OutputDatum::DatumHash(dat_hash) => {
                PlutusData::Constr(BigInt::from(1), vec![dat_hash.to_plutus_data()])
            }
            OutputDatum::InlineDatum(datum) => {
                PlutusData::Constr(BigInt::from(2), vec![datum.to_plutus_data()])
            }
        }
    }

    fn from_plutus_data(data: &PlutusData) -> Result<Self, PlutusDataError> {
        match data {
            PlutusData::Constr(flag, fields) => match u32::try_from(flag) {
                Ok(0) => {
                    verify_constr_fields(&fields, 0)?;
                    Ok(OutputDatum::None)
                }
                Ok(1) => {
                    verify_constr_fields(&fields, 1)?;
                    Ok(OutputDatum::DatumHash(DatumHash::from_plutus_data(
                        &fields[0],
                    )?))
                }
                Ok(2) => {
                    verify_constr_fields(&fields, 1)?;
                    Ok(OutputDatum::InlineDatum(Datum::from_plutus_data(
                        &fields[0],
                    )?))
                }
                _ => Err(PlutusDataError::UnexpectedPlutusInvariant {
                    wanted: "Constr field to be between 0..2".to_owned(),
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

#[cfg(feature = "lbf")]
impl Json for OutputDatum {
    fn to_json(&self) -> serde_json::Value {
        match self {
            OutputDatum::None => json::json_constructor("NoOutputDatum", &Vec::with_capacity(0)),
            OutputDatum::DatumHash(dat_hash) => {
                json::json_constructor("OutputDatumHash", &vec![dat_hash.to_json()])
            }
            OutputDatum::InlineDatum(datum) => {
                json::json_constructor("OutputDatum", &vec![datum.to_json()])
            }
        }
    }

    fn from_json(value: &serde_json::Value) -> Result<Self, Error> {
        json::case_json_constructor(
            "Plutus.V2.OutputDatum",
            vec![
                (
                    "NoOutputDatum",
                    Box::new(|ctor_fields| match &ctor_fields[..] {
                        [] => Ok(OutputDatum::None),
                        _ => Err(Error::UnexpectedArrayLength {
                            wanted: 0,
                            got: ctor_fields.len(),
                            parser: "Plutus.V2.OutputDatum".to_owned(),
                        }),
                    }),
                ),
                (
                    "OutputDatumHash",
                    Box::new(|ctor_fields| match &ctor_fields[..] {
                        [dat_hash] => Ok(OutputDatum::DatumHash(Json::from_json(&dat_hash)?)),
                        _ => Err(Error::UnexpectedArrayLength {
                            wanted: 1,
                            got: ctor_fields.len(),
                            parser: "Plutus.V2.OutputDatum".to_owned(),
                        }),
                    }),
                ),
                (
                    "OutputDatum",
                    Box::new(|ctor_fields| match &ctor_fields[..] {
                        [datum] => Ok(OutputDatum::InlineDatum(Json::from_json(datum)?)),
                        _ => Err(Error::UnexpectedArrayLength {
                            wanted: 1,
                            got: ctor_fields.len(),
                            parser: "Plutus.V2.OutputDatum".to_owned(),
                        }),
                    }),
                ),
            ],
            value,
        )
    }
}
