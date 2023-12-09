use crate::{
    Any, Callback, Example, Header, Link, Parameter, Referenceable, RequestBody, Response, Schema,
    SecurityScheme,
};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::collections::BTreeMap;

/// Holds a set of reusable objects for different aspects of the OAS. All objects defined within the components object will have no effect on the API unless they are explicitly referenced from properties outside the components object.
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Components {
    /// An object to hold reusable Schema Objects.
    pub schemas: Option<BTreeMap<String, Referenceable<Schema>>>,
    /// An object to hold reusable Response Objects.
    pub responses: Option<BTreeMap<String, Referenceable<Response>>>,
    /// An object to hold reusable Parameter Objects.
    pub parameters: Option<BTreeMap<String, Referenceable<Parameter>>>,
    /// An object to hold reusable Example Objects.
    pub examples: Option<BTreeMap<String, Referenceable<Example>>>,
    /// An object to hold reusable Request Body Objects.
    pub request_bodies: Option<BTreeMap<String, Referenceable<RequestBody>>>,
    /// An object to hold reusable Header Objects.
    pub headers: Option<BTreeMap<String, Referenceable<Header>>>,
    /// An object to hold reusable Security Scheme Objects.
    pub security_schemes: Option<BTreeMap<String, Referenceable<SecurityScheme>>>,
    /// An object to hold reusable Link Objects.
    pub links: Option<BTreeMap<String, Referenceable<Link>>>,
    /// An object to hold reusable Callback Objects.
    pub callbacks: Option<BTreeMap<String, Referenceable<Callback>>>,
    #[serde(flatten)]
    pub extras: Option<BTreeMap<String, Any>>,
}
