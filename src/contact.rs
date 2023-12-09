use std::collections::BTreeMap;

use email_address::EmailAddress;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use url::Url;

use crate::{Any, Parseable};

/// Contact information for the exposed API.
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contact {
    /// The identifying name of the contact person/organization.
    pub name: Option<String>,
    /// The URL pointing to the contact information. MUST be in the format of a URL.
    pub url: Option<Parseable<Url>>,
    /// The email address of the contact person/organization. MUST be in the format of an email address.
    pub email: Option<Parseable<EmailAddress>>,
    #[serde(flatten)]
    pub extras: Option<BTreeMap<String, Any>>,
}
