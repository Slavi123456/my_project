use serde::de::DeserializeOwned;

pub trait Extractable: DeserializeOwned + Sized {}
