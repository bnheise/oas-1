use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{Any, Example, MediaType, Referenceable, Schema};

/// The location of the parameter
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ParameterIn {
    Query,
    Header,
    Path,
    Cookie,
}

/// Describes a single operation parameter.
/// A unique parameter is defined by a combination of a name and location.
/// Parameter Locations
/// There are four possible parameter locations specified by the in field:
/// - path - Used together with Path Templating, where the parameter value is actually part of the operation's URL. This does not include the host or base path of the API. For example, in /items/{itemId}, the path parameter is itemId.
/// - query - Parameters that are appended to the URL. For example, in /items?id=###, the query parameter is id.
/// - header - Custom headers that are expected as part of the request. Note that RFC7230 states header names are case insensitive.
/// - cookie - Used to pass a specific cookie value to the API.
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Parameter {
    /// The name of the parameter
    pub name: String,
    /// The location of the parameter
    #[serde(alias = "in")]
    pub _in: ParameterIn,
    /// A brief description of the parameter. This could contain examples of use. CommonMark syntax MAY be used for rich text representation.
    pub description: Option<String>,
    /// Determines whether this parameter is mandatory
    pub required: Option<bool>,
    /// Specifies that a parameter is deprecated and SHOULD be transitioned out of usage. Default value is `false`.
    pub deprecated: Option<bool>,
    /// Sets the ability to pass empty-valued parameters
    pub allow_empty_value: Option<bool>,
    /// Describes how the parameter value will be serialized depending on the type of the parameter value
    pub style: Option<Style>,
    pub explode: Option<bool>,
    pub allow_reserved: Option<bool>,
    /// The schema defining the type used for the parameter.
    pub schema: Option<Referenceable<Schema>>,
    /// Example of the parameter's potential value.
    pub example: Option<Any>,
    /// Examples of the parameter's potential value.
    pub examples: Option<BTreeMap<String, Referenceable<Example>>>,
    /// A map containing the representations for the parameter. The key is the media type and the value describes it.
    pub content: Option<BTreeMap<String, MediaType>>,
    #[serde(flatten)]
    pub extras: Option<BTreeMap<String, Any>>,
}

/// Refer to the [documentation](https://swagger.io/specification/#style-values) for more information.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum Style {
    Matrix,
    Label,
    Form,
    Simple,
    SpaceDelimited,
    PipeDelimited,
    DeepObject,
}
