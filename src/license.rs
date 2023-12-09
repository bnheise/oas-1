use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use url::Url;

use crate::{Any, Parseable};

/// License information for the exposed API.
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct License {
    /// The license name used for the API.
    pub name: String,
    /// A URL to the license used for the API. MUST be in the format of a URL.
    pub url: Option<Parseable<Url>>,
    #[serde(flatten)]
    pub extras: Option<BTreeMap<String, Any>>,
}
