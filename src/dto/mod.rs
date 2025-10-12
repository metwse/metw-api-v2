macro_rules! api_errors {
    (
        $name:ident,
        {
            $($group:ty => (
                kinds($($kind:ty $(|)?)*),
                description($description:expr),
                status_code($status_code:expr),
            )),*
        }
    ) => { paste::paste! {
        #[allow(non_snake_case)]
        mod [< $name _mod >] {
            use utoipa::ToSchema;
            use serde::Serialize;
            use thiserror::Error;
            use axum::http::StatusCode;

            #[doc = stringify!($name)]
            #[doc = "error types"]
            #[derive(Error, Debug, ToSchema, Serialize)]
            pub enum $name {$($(
                #[error($description)]
                #[doc = $description]
                $kind,
            )*)*}

            impl From<&$name> for StatusCode {
                fn from(from: &$name) -> StatusCode {
                    match from {$(
                        #[allow(unused_variables)]
                        $($name::$kind => StatusCode::$status_code, )*
                    )*}
                }
            }

            #[doc = stringify!($name)]
            #[doc = "error response examples"]
            pub mod error_examples {
                use crate::response::AppError;
                use super::$name;
                use utoipa::{
                    IntoResponses,
                    openapi::{
                        RefOr, Response, ResponseBuilder, ResponsesBuilder,
                        example::ExampleBuilder,
                        content::ContentBuilder
                    }
                };
                use std::collections::BTreeMap;
                use axum::http::StatusCode;

            $(
                #[doc = stringify!($group)]
                #[doc = "error response example"]
                pub struct [< $group Dto >];

                impl IntoResponses for [< $group Dto >] {
                    fn responses() -> BTreeMap<String, RefOr<Response>> {
                        let status_code = format!(
                                "{}", StatusCode::$status_code.as_u16()
                            );

                        let mut examples = vec![];
                        $(
                            {
                                let example = ExampleBuilder::new()
                                    .summary(stringify!($kind))
                                    .value(Some(serde_json::json!(
                                        AppError::$name($name::$kind)
                                            .into_dto()
                                    )))
                                    .build();
                                examples.push((stringify!($kind), example));
                            }
                        )*

                        let content = ContentBuilder::new()
                            .examples_from_iter(examples)
                            .build();

                        let res = ResponseBuilder::new()
                            .description($description)
                            .content(
                                "application/json",
                                content
                            );

                        ResponsesBuilder::new()
                            .response(
                                status_code,
                                res,
                            )
                            .build()
                            .into()
                    }
                }
            )*}
        }

        pub use [< $name _mod >]::{
            $name, error_examples
        };
    }};
}

/// Gateway-spesific error types.
pub enum GatewayError {}

/// Gateway DTOs
pub mod gateway;

/// User DTOs
pub mod user;
