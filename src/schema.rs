use crate::Any;
use crate::{Format, Type};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::collections::BTreeMap;

/// The Schema Object allows the definition of input and output data types. These types can be objects, but also primitives and arrays.
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Schema {
    #[serde(rename = "type")]
    pub _type: Option<Type>,
    pub format: Option<Format>,
    pub nullable: Option<bool>,
    #[serde(flatten)]
    pub extras: BTreeMap<String, Any>,
}
