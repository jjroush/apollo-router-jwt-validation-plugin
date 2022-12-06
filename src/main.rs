mod plugins;

use anyhow::Result;
use apollo_router::register_plugin;

register_plugin!("example", "jwks", JwtValidationPlugin);


fn main() -> Result<()> {
    apollo_router::main()
}