use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    Any, Components, ExternalDocumentation, Info, Parseable, PathItem, SecurityRequirement, Server,
    Tag,
};

#[skip_serializing_none]
/// the root document object of openAPI v3.0
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenAPIV3 {
    /// This string MUST be the semantic version number of the OpenAPI Specification version that the OpenAPI document uses. The `openapi` field SHOULD be used by tooling specifications and clients to interpret the OpenAPI document. This is not related to the API info.version string.
    pub openapi: Parseable<semver::Version>,
    /// Provides metadata about the API
    pub info: Info,
    /// An array of Server Objects, which provide connectivity information to a target server. If the `servers` property is not provided, or is an empty array, the default value would be a `Server` Object with a url value of `/`.
    pub servers: Option<Vec<Server>>,
    /// The available paths and operations for the API.
    pub paths: BTreeMap<String, PathItem>,
    /// An element to hold various schemas for the specification.
    pub components: Option<Components>,
    /// A declaration of which security mechanisms can be used across the API. The list of values includes alternative security requirement objects that can be used. Only one of the security requirement objects need to be satisfied to authorize a request. Individual operations can override this definition. To make security optional, an empty security requirement (`{}`) can be included in the array.
    pub security: Option<Vec<SecurityRequirement>>,
    /// A list of tags used by the specification with additional metadata. The order of the tags can be used to reflect on their order by the parsing tools. Not all tags that are used by the Operation Object must be declared. The tags that are not declared MAY be organized randomly or based on the tools' logic. Each tag name in the list MUST be unique.
    pub tags: Option<Vec<Tag>>,
    /// Additional external documentation.
    pub external_docs: Option<ExternalDocumentation>,
    #[serde(flatten)]
    pub extras: Option<BTreeMap<String, Any>>,
}
