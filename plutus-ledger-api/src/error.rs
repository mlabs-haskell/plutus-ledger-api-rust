use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConversionError {
    #[error("ByteString length must be {relation} {expected} but got {got} with value{value_hex}")]
    InvalidByteStringLength {
        ctx: String,
        expected: usize,
        got: usize,
        value_hex: String,
        relation: String,
    },

    #[error("String cannot be parsed as a hexadecimal value: {value_hex}")]
    HexDecodeError {
        value_hex: String,
        source: hex::FromHexError,
    },

    #[error(transparent)]
    ParseError(anyhow::Error),
}

impl ConversionError {
    pub fn invalid_bytestring_length(
        ctx: &str,
        expected: usize,
        relation: &str,
        bytes: &[u8],
    ) -> Self {
        ConversionError::InvalidByteStringLength {
            ctx: ctx.to_string(),
            expected,
            got: bytes.len(),
            relation: relation.to_string(),
            value_hex: hex::encode(bytes),
        }
    }

    pub fn hex_decode_error(err: hex::FromHexError, value_hex: &str) -> Self {
        ConversionError::HexDecodeError {
            source: err,
            value_hex: value_hex.to_string(),
        }
    }
}
