use std::ops::ControlFlow;
use std::vec::Vec;

use apollo_router::graphql;
use apollo_router::layers::ServiceBuilderExt;
use apollo_router::register_plugin;
use apollo_router::plugin::PluginInit;
use apollo_router::plugin::Plugin;
use apollo_router::services::*;
use apollo_router::Context;

use reqwest::StatusCode;

use schemars::JsonSchema;
use base64::{decode};
use serde::Deserialize;
use tower::{util::BoxService, BoxError, ServiceBuilder, ServiceExt};

#[derive(Debug, Default, Deserialize, JsonSchema, Clone)]
struct Authorizer {
    name: String,
    issuer: String
}

#[allow(dead_code)]
struct JwtValidationPlugin {
    configuration: Conf
}

#[derive(Debug, Default, Deserialize, JsonSchema)]
struct Conf {
    authorizers: Vec<Authorizer>
}

#[async_trait::async_trait]
impl Plugin for JwtValidationPlugin {
    type Config = Conf;

    // This is invoked once after the router starts and compiled-in
    // plugins are registered
    async fn new(init: PluginInit<Self::Config>) -> Result<Self, BoxError> {
        println!("hello there!");
        Ok(JwtValidationPlugin { configuration: init.config })
    }

    // Only define the hooks you need to modify. Each default hook
    // implementation returns its associated service with no changes.
    fn supergraph_service(
        self: &JwtValidationPlugin,
        service: BoxService<supergraph::Request, supergraph::Response, BoxError>,
    ) -> BoxService<supergraph::Request, supergraph::Response, BoxError> {
        let token_header: &'static str = "Authorization";
        let authorizers = self.configuration.authorizers.clone();

        let mut issuers: Vec<String> = Vec::new();

        for i in 0..authorizers.len() {
            issuers.push(authorizers[i].issuer.clone());
        }


        ServiceBuilder::new()
            .checkpoint(move |req: supergraph::Request| {
                fn failure_message(
                    context: Context,
                    msg: String,
                    status: StatusCode,
                ) -> Result<ControlFlow<supergraph::Response, supergraph::Request>, BoxError>
                {
                    let res = supergraph::Response::error_builder()
                        .error(graphql::Error::builder().message(msg).build())
                        .status_code(status)
                        .context(context)
                        .build()?;
                    Ok(ControlFlow::Break(res))
                }

                let jwt_value_raw = match req.supergraph_request.headers().get(token_header) {
                    Some(value) => value.to_str(),
                    None =>
                        {
                            // Optional JWT
                            return Ok(ControlFlow::Continue(req))
                        }
                };

                let jwt_value = match jwt_value_raw {
                    Ok(value) => value,
                    Err(_not_a_string_error) => {
                        // Prepare an HTTP 400 response with a GraphQL error message
                        return failure_message(
                            req.context,
                            "Authorization' header is not convertible to a string".to_string(),
                            StatusCode::BAD_REQUEST,
                        );
                    }
                };

                if !jwt_value
                    .starts_with(&format!("{} ", "Bearer"))
                {
                    // Prepare an HTTP 400 response with a GraphQL error message
                    return failure_message(
                        req.context,
                        "Header is not correctly formatted".to_string(),
                        StatusCode::UNAUTHORIZED,
                    );
                }

                let jwt_parts: Vec<&str> = jwt_value.splitn(2, ' ').collect();

                if jwt_parts.len() != 2 {
                    // Prepare an HTTP 400 response with a GraphQL error message
                    return failure_message(
                        req.context,
                        "Header is not correctly formatted".to_string(),
                        StatusCode::UNAUTHORIZED,
                    );
                };

                let jwt: &str = jwt_parts[1];

                let splitted_jwt_strings: Vec<_> = jwt.split('.').collect();

                let jwt_payload_base64 = match splitted_jwt_strings.get(1).clone() {
                    Some(value) => value,
                    None => {
                        // Prepare an HTTP 400 response with a GraphQL error message
                        return failure_message(
                            req.context,
                            "Header is not correctly formatted".to_string(),
                            StatusCode::UNAUTHORIZED,
                        );
                    }
                };

                let jwt_payload_decoded_raw = base64::decode(jwt_payload_base64)?;

                let jwt_payload_decoded = String::from_utf8(jwt_payload_decoded_raw)?;



                // let jwt_payload_base64 = match jwt_value_raw {
                //     Ok(value) => value,
                //     Err(_not_a_string_error) => {
                //         // Prepare an HTTP 400 response with a GraphQL error message
                //         return failure_message(
                //             req.context,
                //             "Authorization' header is not convertible to a string".to_string(),
                //             StatusCode::BAD_REQUEST,
                //         );
                //     }
                // };

                println!("{:?}", jwt_payload_decoded);

                Ok(ControlFlow::Continue(req))
            })
            .service(service)
            .boxed()
    }

    // Unlike other hooks, this hook also passes the name of the subgraph
    // being invoked. That's because this service might invoke *multiple*
    // subgraphs for a single request, and this is called once for each.
    fn subgraph_service(
        &self,
        _name: &str,
        service: BoxService<subgraph::Request, subgraph::Response, BoxError>,
    ) -> BoxService<subgraph::Request, subgraph::Response, BoxError> {
        service
    }
}

register_plugin!("example", "jwks", JwtValidationPlugin);
