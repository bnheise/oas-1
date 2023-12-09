mod components;
mod contact;
mod info;
mod license;
mod openapiv3;
mod parameter;
mod path;
mod schema;
mod security;
mod server;
mod shared;
pub use components::*;
pub use contact::*;
pub use info::*;
pub use license::*;
pub use openapiv3::*;
pub use parameter::*;
pub use path::*;
pub use schema::*;
pub use security::*;
pub use server::*;
pub use shared::*;

pub type Any = serde_json::Value;

macro_rules! impl_serde_json {
    ($($st:ty,)+) => {
        $(
        impl $st {

            pub fn to_string(&self) -> String {
                serde_json::to_string(&self).unwrap()
            }
            pub fn to_value(&self) -> serde_json::Value {
                serde_json::to_value(&self).unwrap()
            }
        }
        )+
    };
}
impl_serde_json! {
    OpenAPIV3, Info, Contact, License, Server, ServerVariable, Components, PathItem,
    Operation, ExternalDocumentation, ParameterIn, Parameter, RequestBody, MediaType,
    Encoding, Responses, Response, Callback, Example, Link, Header, Tag, Reference,
    Schema, Discriminator, SecurityType, SecurityScheme, OauthFlows, OauthFlow, SecurityRequirement,
}

#[cfg(test)]
mod test {
    mod pass {
        use crate::{OpenAPIV3, Parseable, Server};
        use assert_json_diff::assert_json_eq;
        use url::Url;

        macro_rules! pass {
            ($t:ty, $value:expr) => {
                serde_json::from_str::<$t>($value).unwrap();
                let new =
                    serde_json::to_value(&serde_json::from_str::<$t>($value).unwrap()).unwrap();
                let original = serde_json::from_str::<serde_json::Value>($value).unwrap();
                assert_json_eq!(dbg!(new), original);
            };
        }
        #[test]
        fn should_should_pass() {
            // pass! { OpenAPIV3, include_str!("../openapi3-examples/3.0/pass/swagger2openapi/openapi.json") }
            pass! { OpenAPIV3, include_str!("../examples/v3.0/json/api-with-examples.json") }
            pass! { OpenAPIV3, include_str!("../examples/v3.0/json/callback-example.json") }
            pass! { OpenAPIV3, include_str!("../examples/v3.0/json/link-example.json") }
            pass! { OpenAPIV3, include_str!("../examples/v3.0/json/petstore-expanded.json") }
            pass! { OpenAPIV3, include_str!("../examples/v3.0/json/petstore.json") }
            pass! { OpenAPIV3, include_str!("../examples/v3.0/json/uspto.json") }
        }

        #[test]
        fn parseable_returns_string_if_value_is_invalid() {
            let test = r#"{"url": "hello"}"#;

            let actual: Server = serde_json::from_str(test).expect("Should deserialize");

            match actual.url {
                Parseable::Valid(_) => panic!("Should be invalid"),
                Parseable::Invalid(invalid) => assert_eq!(&invalid, "hello"),
            }
        }

        #[test]
        fn parseable_returns_the_parsed_value_if_valid() {
            let test = r#"{"url": "http://google.com"}"#;

            let actual: Server = serde_json::from_str(test).unwrap();

            match actual.url {
                Parseable::Valid(valid) => {
                    assert_eq!(valid, Url::parse("http://google.com").unwrap())
                }
                Parseable::Invalid(_) => panic!("Should be invalid"),
            }
        }
    }
}
