use core::fmt::Debug;
use cosmrs::{proto::{prost::{DecodeError, Name}, traits::Message}, tx::MessageExt, Any};
use std::fmt::Display;

use super::error::ChainError;

pub trait Msg:
    Clone + Sized
    + TryFrom<Self::Proto, Error = Self::Err> 
    + TryInto<Self::Proto, Error = Self::Err>
{
    /// Protobuf type
    type Proto: Default + MessageExt + Sized + Name;

    /// Protobuf conversion error type
    type Err: From<ChainError> + Debug + Display;

    /// Parse this message proto from [`Any`].
    fn from_any(any: &Any) -> Result<Self, Self::Err> {
        if any.type_url == Self::Proto::type_url() {
            let decoded = Self::Proto::decode(&*any.value)
                .map_err(|e| ChainError::prost_proto_decoding(e))?;
            Ok(Self::try_from(decoded)?)
        } else {
            let mut err = DecodeError::new(format!(
                "expected type URL: \"{}\" (got: \"{}\")",
                Self::Proto::type_url(),
                &any.type_url
            ));
            err.push("unexpected type URL", "type_url");
            Err(Self::Err::from(ChainError::prost_proto_decoding(err)))
        }
    }

    fn to_any(&self) -> Result<Any, Self::Err> {
        let mut bytes = Vec::new();
        let msg : Self::Proto = self.clone().try_into()?;
        Message::encode(&msg, &mut bytes)
            .map_err(|e| ChainError::prost_proto_encoding(e))?;

        Ok(Any {
            type_url: Self::Proto::type_url(),
            value: bytes,
        })
    }


}
