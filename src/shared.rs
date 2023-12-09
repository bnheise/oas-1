use crate::parameter::Style;
use crate::Any;
use crate::PathItem;
use crate::Schema;
use crate::Server;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::collections::BTreeMap;
use url::Url;

/// A value that represents a more specific type than the general type that it
/// represents, but whose parsing might fail. A key example is a url, which
/// could be parsed to the [Url] type but may be an invalid url. This enum
/// will deserialize as the parsable type if the type is valid, but if it's
/// invalid, it will deserialize as the Invalid variant rendered as a string.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Parseable<V> {
    Valid(V),
    Invalid(String),
}

impl<V> Parseable<V> {
    pub fn is_valid(&self) -> bool {
        match self {
            Parseable::Valid(_) => true,
            Parseable::Invalid(_) => false,
        }
    }
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Referenceable<T> {
    Data(T),
    Reference(Reference),
}

impl<T> Referenceable<T> {
    pub fn is_reference(&self) -> bool {
        match self {
            Referenceable::Data(_) => false,
            Referenceable::Reference(_) => true,
        }
    }
}

/// Allows referencing an external resource for extended documentation.
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalDocumentation {
    /// A short description of the target documentation. CommonMark syntax MAY be used for rich text representation.
    pub description: Option<String>,
    /// The URL for the target documentation. Value MUST be in the format of a URL.
    pub url: Parseable<Url>,
    #[serde(flatten)]
    pub extras: Option<BTreeMap<String, Any>>,
}

/// Describes a single request body.
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestBody {
    /// A brief description of the request body.
    pub description: Option<String>,
    /// Determines if the request body is required in the request. Defaults to `false`.
    pub required: Option<bool>,
    /// The content of the request body.
    pub content: BTreeMap<String, MediaType>,
    #[serde(flatten)]
    pub extras: Option<BTreeMap<String, Any>>,
}

/// Each Media Type Object provides schema and examples for the media type identified by its key.
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaType {
    /// The schema defining the content of the request, response, or parameter.
    pub schema: Option<Referenceable<Schema>>,
    /// Example of the media type.
    pub example: Option<Any>,
    /// Examples of the media type.
    pub examples: Option<BTreeMap<String, Referenceable<Example>>>,
    /// A map between a property name and its encoding information.
    pub encoding: Option<BTreeMap<String, Encoding>>,
    #[serde(flatten)]
    pub extras: Option<BTreeMap<String, Any>>,
}

/// A single encoding definition applied to a single schema property.
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Encoding {
    /// The Content-Type for encoding a specific property.
    pub content_type: Option<Parseable<MediaType>>,
    /// map allowing additional information to be provided as headers, for example `Content-Disposition`. `Content-Type` is described separately and SHALL be ignored in this section. This property SHALL be ignored if the request body media type is not a `multipart`.
    pub headers: Option<BTreeMap<String, Referenceable<Header>>>,
    /// Describes how a specific property value will be serialized depending on its type.
    pub style: Option<Style>,
    pub explode: Option<bool>,
    pub allow_reserved: Option<bool>,
    #[serde(flatten)]
    pub extras: Option<BTreeMap<String, Any>>,
}

/// A container for the expected responses of an operation. The container maps a HTTP response code to the expected response.
/// The documentation is not necessarily expected to cover all possible HTTP response codes because they may not be known in advance. However, documentation is expected to cover a successful operation response and any known errors.
/// The default MAY be used as a default response object for all HTTP codes that are not covered individually by the specification.
/// The Responses Object MUST contain at least one response code, and it SHOULD be the response for a successful operation call.
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Responses {
    /// The documentation of responses other than the ones declared for specific HTTP response codes. Use this field to cover undeclared responses. A Reference Object can link to a response that the OpenAPI Object's components/responses section defines.
    pub default: Option<Referenceable<Response>>,
    #[serde(flatten)]
    pub data: BTreeMap<String, Referenceable<Response>>,
    #[serde(flatten)]
    pub extras: Option<BTreeMap<String, Any>>,
}

/// Describes a single response from an API Operation, including design-time, static `links` to operations based on the response.
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Response {
    /// A short description of the response.
    pub description: String,
    /// Maps a header name to its definition.
    pub headers: Option<BTreeMap<String, Referenceable<Header>>>,
    /// A map containing descriptions of potential response payloads.
    pub content: Option<BTreeMap<String, MediaType>>,
    /// A map of operations links that can be followed from the response.
    pub links: Option<BTreeMap<String, Referenceable<Link>>>,
    #[serde(flatten)]
    pub extras: Option<BTreeMap<String, Any>>,
}

/// A map of possible out-of band callbacks related to the parent operation.
/// Each value in the map is a Path Item Object that describes a set of requests
/// that may be initiated by the API provider and the expected responses.
/// The key value used to identify the path item object is an expression,
/// evaluated at runtime, that identifies a URL to use for the callback operation.
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Callback {
    #[serde(flatten)]
    pub data: BTreeMap<String, PathItem>,
    #[serde(flatten)]
    pub extras: Option<BTreeMap<String, Any>>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Example {
    /// Short description for the example.
    pub summary: Option<String>,
    /// Long description for the example.
    pub description: Option<String>,
    /// Embedded literal example.
    pub value: Option<Any>,
    pub external_value: Option<String>,
    #[serde(flatten)]
    pub extras: Option<BTreeMap<String, Any>>,
}

/// represents a possible design-time link for a response.
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Link {
    /// A relative or absolute URI reference to an OAS operation.
    pub operation_ref: Option<String>,
    /// The name of an existing, resolvable OAS operation
    pub operation_id: String,
    /// A map representing parameters to pass to an operation as specified with `operation_id` or identified via `operation_ef`.
    pub parameters: Option<BTreeMap<String, Any>>,
    /// A literal value or `{expression}` to use as a request body when calling the target operation.
    pub request_body: Option<Any>,
    /// A description of the link.
    pub description: Option<String>,
    /// A server object to be used by the target operation.
    pub server: Option<Server>,
    #[serde(flatten)]
    pub extras: Option<BTreeMap<String, Any>>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Header {
    pub description: Option<String>,
    pub required: Option<bool>,
    pub deprecated: Option<bool>,
    pub allow_empty_value: Option<bool>,
    pub style: Option<Style>,
    pub explode: Option<bool>,
    pub allow_reserved: Option<bool>,
    pub schema: Option<Referenceable<Schema>>,
    pub example: Option<Any>,
    pub examples: Option<BTreeMap<String, Referenceable<Example>>>,
    pub content: Option<BTreeMap<String, MediaType>>,
    #[serde(flatten)]
    pub extras: Option<BTreeMap<String, Any>>,
}

/// Adds metadata to a single tag that is used by the `Operation` Object. It is not mandatory to have a Tag Object per tag defined in the Operation Object instances.
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tag {
    /// The name of the tag.
    pub name: String,
    /// A short description for the tag.
    pub description: Option<String>,
    /// Additional external documentation for this tag.
    pub external_docs: Option<ExternalDocumentation>,
    #[serde(flatten)]
    pub extras: Option<BTreeMap<String, Any>>,
}

impl Tag {
    pub fn new(name: impl Into<String>, description: impl Into<Option<String>>) -> Tag {
        Self {
            name: name.into(),
            description: description.into(),
            external_docs: None,
            extras: None,
        }
    }
}

/// A simple object to allow referencing other components in the specification, internally and externally.
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reference {
    /// The reference string.
    #[serde(rename = "$ref")]
    pub _ref: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum Type {
    Null,
    Boolean,
    Object,
    Array,
    Integer,
    Number,
    String,
    /// Allows capturing potentially invalid values
    #[serde(untagged)]
    Other(String),
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum Format {
    DateTime,
    Date,
    Time,
    Duration,
    #[serde(rename = "int32")]
    Int32,
    #[serde(rename = "int64")]
    Int64,
    Float,
    Double,
    Email,
    IdnEmail,
    Hostname,
    IdnHostname,
    Password,
    #[serde(rename = "ipv4")]
    Ipv4,
    #[serde(rename = "ipv6")]
    Ipv6,
    Uri,
    UriReference,
    Iri,
    IriReference,
    Uuid,
    UriTemplate,
    JsonPointer,
    RelativeJsonPointer,
    Regex,
    /// Allows capturing potentially invalid values
    #[serde(untagged)]
    Other(String),
}

/// When using the discriminator, inline schemas will not be considered.
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Discriminator {
    /// The name of the property in the payload that will hold the discriminator value.
    pub property_name: String,
    /// An object to hold mappings between payload values and schema names or references.
    pub mapping: Option<BTreeMap<String, String>>,
    #[serde(flatten)]
    pub extras: Option<BTreeMap<String, Any>>,
}
