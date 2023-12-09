use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    Any, Callback, ExternalDocumentation, Parameter, Referenceable, RequestBody, Responses,
    SecurityRequirement, Server,
};

/// Describes the operations available on a single path. A Path Item MAY be empty, due to ACL constraints. The path itself is still exposed to the documentation viewer but they will not know which operations and parameters are available.
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PathItem {
    /// Allows for an external definition of this path item. The referenced structure MUST be in the format of a Path Item Object. In case a Path Item Object field appears both in the defined object and the referenced object, the behavior is undefined.
    #[serde(rename = "$ref")]
    pub _ref: Option<String>,
    /// An optional, string summary, intended to apply to all operations in this path.
    pub summary: Option<String>,
    /// An optional, string description, intended to apply to all operations in this path. CommonMark syntax MAY be used for rich text representation.
    pub description: Option<String>,
    /// A definition of a GET operation on this path.
    pub get: Option<Operation>,
    /// A definition of a PUT operation on this path.
    pub put: Option<Operation>,
    /// A definition of a POST operation on this path.
    pub post: Option<Operation>,
    /// A definition of a DELETE operation on this path.
    pub delete: Option<Operation>,
    /// A definition of a OPTIONS operation on this path.
    pub options: Option<Operation>,
    /// A definition of a HEAD operation on this path.
    pub head: Option<Operation>,
    /// A definition of a PATCH operation on this path.
    pub patch: Option<Operation>,
    /// A definition of a TRACE operation on this path.
    pub trace: Option<Operation>,
    /// An alternative `server` array to service all operations in this path.
    pub servers: Option<Vec<Server>>,
    /// A list of parameters that are applicable for all the operations described under this path. These parameters can be overridden at the operation level, but cannot be removed there. The list MUST NOT include duplicated parameters. A unique parameter is defined by a combination of a name and location. The list can use the Reference Object to link to parameters that are defined at the OpenAPI Object's components/parameters.
    pub parameters: Option<Vec<Referenceable<Parameter>>>,
    #[serde(flatten)]
    pub extras: Option<BTreeMap<String, Any>>,
}

/// Describes a single API operation on a path.
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Operation {
    /// A list of tags for API documentation control. Tags can be used for logical grouping of operations by resources or any other qualifier.
    pub tags: Option<Vec<String>>,
    /// A short summary of what the operation does.
    pub summary: Option<String>,
    /// A verbose explanation of the operation behavior. CommonMark syntax MAY be used for rich text representation.
    pub description: Option<String>,
    /// Additional external documentation for this operation.
    pub external_docs: Option<ExternalDocumentation>,
    /// Unique string used to identify the operation. The id MUST be unique among all operations described in the API. The operationId value is case-sensitive. Tools and libraries MAY use the operationId to uniquely identify an operation, therefore, it is RECOMMENDED to follow common programming naming conventions.
    pub operation_id: Option<String>,
    /// A list of parameters that are applicable for this operation. If a parameter is already defined at the Path Item, the new definition will override it but can never remove it. The list MUST NOT include duplicated parameters. A unique parameter is defined by a combination of a name and location. The list can use the Reference Object to link to parameters that are defined at the OpenAPI Object's components/parameters.
    pub parameters: Option<Vec<Referenceable<Parameter>>>,
    /// The request body applicable for this operation. The requestBody is only supported in HTTP methods where the HTTP 1.1 specification RFC7231 has explicitly defined semantics for request bodies. In other cases where the HTTP spec is vague, requestBody SHALL be ignored by consumers.
    pub request_body: Option<Referenceable<RequestBody>>,
    /// The list of possible responses as they are returned from executing this operation.
    pub responses: Responses,
    /// A map of possible out-of band callbacks related to the parent operation. The key is a unique identifier for the Callback Object. Each value in the map is a Callback Object that describes a request that may be initiated by the API provider and the expected responses.
    pub callbacks: Option<BTreeMap<String, Referenceable<Callback>>>,
    /// Declares this operation to be deprecated. Consumers SHOULD refrain from usage of the declared operation. Default value is `false`.
    pub deprecated: Option<bool>,
    /// A declaration of which security mechanisms can be used for this operation. The list of values includes alternative security requirement objects that can be used. Only one of the security requirement objects need to be satisfied to authorize a request. To make security optional, an empty security requirement (`{}`) can be included in the array. This definition overrides any declared top-level security. To remove a top-level security declaration, an empty array can be used.
    pub security: Option<Vec<SecurityRequirement>>,
    /// An alternative server array to service this operation. If an alternative server object is specified at the Path Item Object or Root level, it will be overridden by this value.
    pub servers: Option<Vec<Server>>,
    #[serde(flatten)]
    pub extras: Option<BTreeMap<String, Any>>,
}
