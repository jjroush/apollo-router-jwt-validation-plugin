use apollo_router::register_plugin;
use apollo_router::plugin::PluginInit;
use apollo_router::plugin::Plugin;
use apollo_router::services::*;
use schemars::JsonSchema;
use serde::Deserialize;
// use std::println;
use tower::{util::BoxService, BoxError};

#[allow(dead_code)]
struct JwtValidationPlugin {
    configuration: Conf
}

#[derive(Debug, Default, Deserialize, JsonSchema)]
struct Conf {
    // Put your plugin confguration here. It's deserialzed from YAML automatically.
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
        service
    }

    // fn execution_service(
    //     self: &JwtValidationPlugin,
    //     service: BoxService<supergraph::Request, supergraph::Response, BoxError>,
    // ) -> BoxService<supergraph::Request, supergraph::Response, BoxError> {
    //     service
    // }

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
