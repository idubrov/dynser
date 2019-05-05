//! https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.0.md
#![allow(non_snake_case)]
use dynser_derive::Object;
use serde_derive::Deserialize;
use serde_json::Value;
use std::collections::HashMap;

#[derive(Default, Debug, PartialEq, Deserialize, Object)]
pub struct OpenApi {
    #[primitive]
    pub openapi: String,
    pub info: Info,
    #[serde(default)]
    pub servers: Vec<Server>,
    pub paths: Paths,
    pub components: Option<Components>,
    #[serde(default)]
    pub security: Vec<SecurityRequirement>,
    #[serde(default)]
    pub tags: Vec<Tag>,
    pub externalDocs: Option<ExternalDocumentation>,

    #[cfg(not(feature = "no-flatten"))]
    #[serde(flatten)]
    #[default]
    pub extensions: HashMap<String, Value>,
}

#[derive(Default, Debug, PartialEq, Deserialize, Object)]
pub struct Info {
    #[primitive]
    pub title: String,
    #[primitive]
    pub description: Option<String>,
    #[primitive]
    pub termsOfService: Option<String>,
    pub contact: Option<Contact>,
    pub license: Option<License>,
    #[primitive]
    pub version: String,

    #[cfg(not(feature = "no-flatten"))]
    #[serde(flatten)]
    #[default]
    pub extensions: HashMap<String, Value>,
}

#[derive(Default, Debug, PartialEq, Deserialize, Object)]
pub struct Contact {
    #[primitive]
    pub name: Option<String>,
    #[primitive]
    pub url: Option<String>,
    #[primitive]
    pub email: Option<String>,

    #[cfg(not(feature = "no-flatten"))]
    #[serde(flatten)]
    #[default]
    pub extensions: HashMap<String, Value>,
}

#[derive(Default, Debug, PartialEq, Deserialize, Object)]
pub struct License {
    #[primitive]
    pub name: String,
    #[primitive]
    pub url: Option<String>,

    #[cfg(not(feature = "no-flatten"))]
    #[serde(flatten)]
    #[default]
    pub extensions: HashMap<String, Value>,
}

#[derive(Default, Debug, PartialEq, Deserialize, Object)]
pub struct Server {
    #[primitive]
    pub url: String,
    #[primitive]
    pub description: Option<String>,
    #[serde(default)]
    pub variables: HashMap<String, ServerVariable>,

    #[cfg(not(feature = "no-flatten"))]
    #[serde(flatten)]
    #[default]
    pub extensions: HashMap<String, Value>,
}

#[derive(Default, Debug, PartialEq, Deserialize, Object)]
pub struct ServerVariable {
    #[serde(default)]
    #[primitive]
    pub r#enum: Vec<String>,
    #[primitive]
    pub r#default: String,
    #[primitive]
    pub description: Option<String>,

    #[cfg(not(feature = "no-flatten"))]
    #[serde(flatten)]
    #[default]
    pub extensions: HashMap<String, Value>,
}

#[derive(Default, Debug, PartialEq, Deserialize, Object)]
pub struct Components {
    #[serde(default)]
    pub schemas: HashMap<String, Reference<Schema>>,
    #[serde(default)]
    pub responses: HashMap<String, Reference<Response>>,
    #[serde(default)]
    pub parameters: HashMap<String, Reference<Parameter>>,
    #[serde(default)]
    pub examples: HashMap<String, Reference<Example>>,
    #[serde(default)]
    pub requestBodies: HashMap<String, Reference<RequestBody>>,
    #[serde(default)]
    pub headers: HashMap<String, Reference<Header>>,
    #[serde(default)]
    pub securitySchemes: HashMap<String, Reference<SecurityScheme>>,
    #[serde(default)]
    pub links: HashMap<String, Reference<Link>>,
    #[serde(default)]
    pub callbacks: HashMap<String, Reference<Callback>>,

    #[cfg(not(feature = "no-flatten"))]
    #[serde(flatten)]
    #[default]
    pub extensions: HashMap<String, Value>,
}

pub type Paths = HashMap<String, Path>;

#[derive(Default, Debug, PartialEq, Deserialize, Object)]
pub struct Path {
    // FIXME: should support "renaming"
    #[primitive]
    #[serde(rename = "$ref")]
    pub r#ref: Option<String>,
    #[primitive]
    pub summary: Option<String>,
    #[primitive]
    pub description: Option<String>,
    pub get: Option<Operation>,
    pub put: Option<Operation>,
    pub post: Option<Operation>,
    pub delete: Option<Operation>,
    pub options: Option<Operation>,
    pub head: Option<Operation>,
    pub patch: Option<Operation>,
    pub trace: Option<Operation>,
    #[serde(default)]
    pub servers: Vec<Server>,
    #[serde(default)]
    pub parameters: Vec<Reference<Parameter>>,

    #[cfg(not(feature = "no-flatten"))]
    #[serde(flatten)]
    #[default]
    pub extensions: HashMap<String, Value>,
}

#[derive(Default, Debug, PartialEq, Deserialize, Object)]
pub struct Operation {
    #[serde(default)]
    #[primitive]
    pub tags: Vec<String>,
    #[primitive]
    pub summary: Option<String>,
    #[primitive]
    pub description: Option<String>,
    pub externalDocs: Option<ExternalDocumentation>,
    #[primitive]
    pub operationId: Option<String>,
    #[serde(default)]
    pub parameters: Vec<Reference<Parameter>>,
    pub requestBody: Option<Reference<RequestBody>>,
    pub responses: Responses,
    #[serde(default)]
    pub callbacks: HashMap<String, Reference<Callback>>,
    #[serde(default)]
    #[primitive]
    pub deprecated: bool,
    #[serde(default)]
    pub security: Vec<SecurityRequirement>,
    #[serde(default)]
    pub servers: Vec<Server>,

    #[cfg(not(feature = "no-flatten"))]
    #[serde(flatten)]
    #[default]
    pub extensions: HashMap<String, Value>,
}

#[derive(Default, Debug, PartialEq, Deserialize, Object)]
pub struct ExternalDocumentation {
    #[primitive]
    pub description: Option<String>,
    #[primitive]
    pub url: String,

    #[cfg(not(feature = "no-flatten"))]
    #[serde(flatten)]
    #[default]
    pub extensions: HashMap<String, Value>,
}

#[derive(Default, Debug, PartialEq, Deserialize, Object)]
pub struct Parameter {
    #[primitive]
    pub name: String,
    #[primitive]
    pub r#in: String,
    #[primitive]
    pub description: Option<String>,
    #[serde(default)]
    #[primitive]
    pub required: bool,
    #[serde(default)]
    #[primitive]
    pub deprecated: bool,
    #[serde(default)]
    #[primitive]
    pub allowEmptyValue: bool,

    #[primitive]
    pub style: Option<String>,
    #[serde(default)]
    #[primitive]
    pub explode: bool,
    #[serde(default)]
    #[primitive]
    pub allowReserved: bool,
    pub schema: Option<Reference<Schema>>,
    #[any]
    pub example: Option<Value>,
    #[serde(default)]
    pub examples: HashMap<String, Reference<Example>>,

    #[serde(default)]
    pub content: HashMap<String, MediaType>,

    #[cfg(not(feature = "no-flatten"))]
    #[serde(flatten)]
    #[default]
    pub extensions: HashMap<String, Value>,
}

#[derive(Default, Debug, PartialEq, Deserialize, Object)]
pub struct RequestBody {
    #[primitive]
    pub description: Option<String>,
    #[serde(default)]
    pub content: HashMap<String, MediaType>,
    #[serde(default)]
    #[primitive]
    pub required: bool,

    #[cfg(not(feature = "no-flatten"))]
    #[serde(flatten)]
    #[default]
    pub extensions: HashMap<String, Value>,
}

#[derive(Default, Debug, PartialEq, Deserialize, Object)]
pub struct MediaType {
    pub schema: Option<Reference<Schema>>,
    #[any]
    pub example: Option<Value>,
    #[serde(default)]
    pub examples: HashMap<String, Reference<Example>>,
    #[serde(default)]
    pub encoding: HashMap<String, Encoding>,

    #[cfg(not(feature = "no-flatten"))]
    #[serde(flatten)]
    #[default]
    pub extensions: HashMap<String, Value>,
}

#[derive(Default, Debug, PartialEq, Deserialize, Object)]
pub struct Encoding {
    #[primitive]
    pub contentType: Option<String>,
    #[serde(default)]
    pub headers: HashMap<String, Reference<Header>>,
    #[primitive]
    pub style: Option<String>,
    #[serde(default)]
    #[primitive]
    pub explode: bool,
    #[serde(default)]
    #[primitive]
    pub allowReserved: bool,

    #[cfg(not(feature = "no-flatten"))]
    #[serde(flatten)]
    #[default]
    pub extensions: HashMap<String, Value>,
}

pub type Responses = HashMap<String, Reference<Response>>;

#[derive(Default, Debug, PartialEq, Deserialize, Object)]
pub struct Response {
    #[primitive]
    pub description: Option<String>,
    #[serde(default)]
    pub headers: HashMap<String, Reference<Header>>,
    #[serde(default)]
    pub content: HashMap<String, MediaType>,
    #[serde(default)]
    pub links: HashMap<String, Reference<Link>>,

    #[cfg(not(feature = "no-flatten"))]
    #[serde(flatten)]
    #[default]
    pub extensions: HashMap<String, Value>,
}

pub type Callback = HashMap<String, Path>;

#[derive(Default, Debug, PartialEq, Deserialize, Object)]
pub struct Example {
    #[primitive]
    pub summary: Option<String>,
    #[primitive]
    pub description: Option<String>,
    #[any]
    pub value: Option<Value>,
    #[primitive]
    #[primitive]
    pub externalValue: Option<String>,

    #[cfg(not(feature = "no-flatten"))]
    #[serde(flatten)]
    #[default]
    pub extensions: HashMap<String, Value>,
}

#[derive(Default, Debug, PartialEq, Deserialize, Object)]
pub struct Link {
    #[primitive]
    pub operationRef: Option<String>,
    #[primitive]
    pub operationId: Option<String>,
    #[serde(default)]
    pub parameters: HashMap<String, Value>,
    #[any]
    pub requestBody: Option<Value>,
    #[primitive]
    pub description: Option<String>,
    pub server: Option<Server>,

    #[cfg(not(feature = "no-flatten"))]
    #[serde(flatten)]
    #[default]
    pub extensions: HashMap<String, Value>,
}

pub type Header = Parameter;

#[derive(Default, Debug, PartialEq, Deserialize, Object)]
pub struct Tag {
    #[primitive]
    pub name: String,
    #[primitive]
    pub description: Option<String>,
    pub externalDocs: Option<ExternalDocumentation>,

    #[cfg(not(feature = "no-flatten"))]
    #[serde(flatten)]
    #[default]
    pub extensions: HashMap<String, Value>,
}

// FIXME: too big, so let's do JSON for now
// FIXME: has to do HashMap -- we don't implement `Object` for `Value`!
pub type Schema = HashMap<String, Value>;

#[derive(Default, Debug, PartialEq, Deserialize, Object)]
pub struct SecurityScheme {
    #[primitive]
    pub r#type: String,
    #[any]
    pub description: Option<Value>,
    #[serde(default)]
    #[primitive]
    pub name: String,
    #[serde(default)]
    #[primitive]
    pub r#in: String,
    #[serde(default)]
    #[primitive]
    pub scheme: String,
    #[primitive]
    pub bearerFormat: Option<String>,
    pub flows: Option<OAuthFlows>,
    #[primitive]
    pub openIdConnectUrl: Option<String>,

    #[cfg(not(feature = "no-flatten"))]
    #[serde(flatten)]
    #[default]
    pub extensions: HashMap<String, Value>,
}

#[derive(Default, Debug, PartialEq, Deserialize, Object)]
pub struct OAuthFlows {
    pub implicit: Option<OAuthFlow>,
    pub password: Option<OAuthFlow>,
    pub clientCredentials: Option<OAuthFlow>,
    pub authorizationCode: Option<OAuthFlow>,

    #[cfg(not(feature = "no-flatten"))]
    #[serde(flatten)]
    #[default]
    pub extensions: HashMap<String, Value>,
}

#[derive(Default, Debug, PartialEq, Deserialize, Object)]
pub struct OAuthFlow {
    #[serde(default)]
    #[primitive]
    pub authorizationUrl: String,
    #[primitive]
    pub tokenUrl: String,
    #[primitive]
    pub refreshUrl: Option<String>,
    pub scopes: HashMap<String, String>,

    #[cfg(not(feature = "no-flatten"))]
    #[serde(flatten)]
    #[default]
    pub extensions: HashMap<String, Value>,
}

pub type SecurityRequirement = HashMap<String, Vec<String>>;

/// Make it easier on serde -- don't use untagged enums
#[cfg(feature = "no-flatten")]
pub type Reference<T> = T;

#[cfg(not(feature = "no-flatten"))]
pub use self::reference::{Reference, ReferenceValue};

#[cfg(not(feature = "no-flatten"))]
mod reference {
    use crate::reflection::{FieldMutReflection, Object, ReflectionError};
    use serde_derive::Deserialize;

    #[derive(Default, Debug, PartialEq, Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct ReferenceValue {
        #[serde(rename = "$ref")]
        reference: String,
    }

    #[derive(Debug, PartialEq, Deserialize)]
    #[serde(untagged)]
    pub enum Reference<T> {
        Reference(ReferenceValue),
        Other(T),
    }

    // FIXME: ugly -- we use empty reference as an indication of "no value"
    // This makes `$ref: ""` to have no effect, etc.
    impl<T> Default for Reference<T> {
        fn default() -> Self {
            Reference::Reference(Default::default())
        }
    }

    impl<T> Object for Reference<T>
    where
        T: Object + Default,
    {
        fn create(&mut self, field_name: &str) -> Result<FieldMutReflection, ReflectionError> {
            match self {
                Reference::Other(other) => other.create(field_name),
                Reference::Reference { .. } if field_name == "$ref" => match self {
                    Reference::Reference(r) => Ok(FieldMutReflection::Primitive(&mut r.reference)),
                    Reference::Other(_) => unreachable!(),
                },
                Reference::Reference { .. } => {
                    let previous_ref = if let Reference::Reference(r) = self {
                        std::mem::replace(&mut r.reference, String::new())
                    } else {
                        unreachable!()
                    };

                    // Mutate ourselves into other
                    *self = Reference::Other(T::default());
                    match self {
                        Reference::Reference { .. } => unreachable!(),
                        Reference::Other(other) => {
                            // Move previously saved reference to the new object
                            if !previous_ref.is_empty() {
                                use crate::reflection::PrimitiveValue;
                                match other.create("$ref")? {
                                    FieldMutReflection::Primitive(primitive) => {
                                        primitive.set(PrimitiveValue::String(previous_ref))?;
                                    }
                                    _ => {
                                        return Err(ReflectionError::InvalidField {
                                            name: "$ref".to_string(),
                                        })
                                    }
                                }
                            }
                            other.create(field_name)
                        }
                    }
                }
            }
        }
    }
}
