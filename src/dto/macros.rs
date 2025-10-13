macro_rules! api_errors {
    (
        $error_name:ident,
        responses( $($groups:tt)* )
    ) => {
        api_errors_impl!(@parse_groups(
            error_name = $error_name,
            tokens = ($($groups)*),
            groups = []
        ));
    };
}

macro_rules! api_errors_impl {
    (@parse_groups(
        error_name = $error_name:ident,
        tokens = ( $group_name:ident = ( $($kvs:tt)* ), $($rest:tt)* ),
        groups = [ $($parsed_groups:tt)* ]
    )) => {
        api_errors_impl!(@parse_group_kvs(
            error_name = $error_name,
            group_name = $group_name,
            group_tokens = ( $($kvs)* ),
            rest_tokens = ( $($rest)* ),
            parsed_groups = [ $($parsed_groups)* ]
        ));
    };
    (@parse_groups(
        error_name = $error_name:ident,
        tokens = (),
        groups = [ $($parsed_groups:tt)* ]
    )) => {
        api_errors_impl!(@generate(
            error_name = $error_name,
            groups = [ $($parsed_groups)* ]
        ));
    };
    (@parse_group_kvs(
        error_name = $error_name:ident,
        group_name = $group_name:ident,
        group_tokens = (
            status = $status:ident,
            description = $description:expr,
            variants = ($(
                $variant_name:ident $(($( $fields:tt )*))? = $str:literal
                $(( $($example:tt)* ))?$(,)?
            )*)
        ),
        rest_tokens = ( $($rest:tt)* ),
        parsed_groups = [ $($parsed_groups:tt)* ]
    )) => {
        api_errors_impl!(@parse_groups(
            error_name = $error_name,
            tokens = ( $($rest)* ),
            groups = [
                $($parsed_groups)*
                {
                    group_name = $group_name,
                    status = $status,
                    description = $description,
                    variants = [ $({
                        name = $variant_name,
                        $( fields = ($($fields)*) )?,
                        str = $str,
                        $( example = ($($example)*) )?,
                    })* ]
                }
            ]
        ));
    };
    (@generate(
        error_name = $error_name:ident,
        groups = [
            $({
                group_name = $group_name:ident,
                status = $status:ident,
                description = $description:expr,
                variants = [$({
                    name = $variant_name:ident,
                    $( fields = ($($fields:tt)*) )?,
                    str = $str:literal,
                    $( example = ($($example:tt)*) )?,
                })*]
            })*
        ]
    )) => {
        #[allow(missing_docs)]
        #[derive(Debug, thiserror::Error, utoipa::ToSchema, serde::Serialize)]
        #[doc = stringify!($error_name)]
        #[doc = "error types"]
        pub enum $error_name { $($(
            #[error($str)]
            $variant_name $( $($fields)* )?,
        )*)* }

        #[allow(non_snake_case, unused_variables)]
        impl From<&$error_name> for axum::http::StatusCode {
            fn from(from: &$error_name) -> axum::http::StatusCode {
                match from { $($(
                    $error_name::$variant_name $( $($fields)* )? => axum::http::StatusCode::$status,
                )*)* }
            }
        }

        #[doc = stringify!($error_name)]
        #[doc = "error response example"]
        pub mod error_examples { paste::paste! {
            use crate::response::AppError;
            use super::$error_name;
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
            #[doc = stringify!($group_name)]
            #[doc = "error response example"]
            pub struct [< $group_name Dto >];

            impl IntoResponses for [< $group_name Dto >] {
                fn responses() -> BTreeMap<String, RefOr<Response>> {
                    let status_code = format!(
                            "{}", StatusCode::$status.as_u16()
                        );

                    let mut examples = vec![];
                    $(
                        {
                            let example = ExampleBuilder::new()
                                .summary(stringify!($variant_name))
                                .value(Some(serde_json::json!(
                                    AppError::$error_name(
                                            $error_name::$variant_name $($($example)*)?
                                        )
                                        .into_dto()
                                )))
                                .build();
                            examples.push((stringify!($variant_name), example));
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
        )*
        } }
    };
}
