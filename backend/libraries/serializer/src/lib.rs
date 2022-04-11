use rmp_serde::{Deserializer, Serializer};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::error::Error;
use std::io::{Read, Write};

pub fn serialize<T, W>(value: T, writer: W) -> Result<(), impl Error>
where
    T: Serialize,
    W: Write,
{
    let mut serializer = Serializer::new(writer).with_struct_map();
    match value.serialize(&mut serializer) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

pub fn deserialize<T, R>(reader: R) -> Result<T, impl Error>
where
    T: DeserializeOwned,
    R: Read,
{
    let mut deserializer = Deserializer::new(reader);
    T::deserialize(&mut deserializer)
}
