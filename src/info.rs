use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{Any, Contact, License};

/// The object provides metadata about the API. The metadata MAY be used by the clients if needed, and MAY be presented in editing or documentation generation tools for convenience.
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Info {
    /// The title of the API.
    pub title: String,
    /// A short description of the API. CommonMark syntax MAY be used for rich text representation.
    pub description: Option<String>,
    /// A URL to the Terms of Service for the API. MUST be in the format of a URL.
    pub terms_of_service: Option<String>,
    /// The contact information for the exposed API.
    pub contact: Option<Contact>,
    /// The license information for the exposed API.
    pub license: Option<License>,
    /// The version of the OpenAPI document (which is distinct from the OpenAPI Specification version or the API implementation version).
    pub version: String,
    #[serde(flatten)]
    pub extras: Option<BTreeMap<String, Any>>,
}
