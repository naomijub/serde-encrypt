use std::marker::PhantomData;

use serde::Deserialize;

use crate::error::Error;

/// Serialized plain-text to hold determine lifetime of `serde::Deserialize<'de>`.
#[derive(Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct SerializedPlain<T> {
    serialized_plain: Vec<u8>,
    _type: PhantomData<T>,
}

impl<T> SerializedPlain<T> {
    pub(in crate::traits) fn new(serialized_plain: Vec<u8>) -> Self {
        Self {
            serialized_plain,
            _type: PhantomData::default(),
        }
    }

    /// Deserialize to get plain message.
    ///
    /// # Failures
    ///
    /// - [DeserializationError](crate::error::ErrorKind::DeserializationError) when failed to deserialize decrypted message.
    pub fn deserialize<'de>(&'de self) -> Result<T, Error>
    where
        T: Sized + Deserialize<'de>,
    {
        serde_cbor::from_slice(&self.serialized_plain).map_err(|e| {
            Error::deserialization_error(&format!(
                "error on serde_cbor deserialization after decryption: {:?}",
                e
            ))
        })
    }
}
