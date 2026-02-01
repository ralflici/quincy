use crate::error::RouteError;
use crate::Result;
use ipnet::IpNet;
use route_manager::{Route, RouteManager};
use std::net::IpAddr;

/// Adds a list of routes to the routing table.
///
/// ### Arguments
/// - `networks` - the networks to be routed through the gateway
/// - `gateway` - the gateway to be used for the routes
/// - `interface_name` - the name of the interface to add the routes to
pub fn add_routes(networks: &[IpNet], gateway: &IpAddr, interface_name: &str) -> Result<()> {
    let mut manager = RouteManager::new().expect("RouteManager::new() is infallible");
    let if_name = interface_name.to_string();
    let routes = networks.iter().map(|n| {
        Route::new(n.addr(), n.prefix_len())
            .with_gateway(*gateway)
            .with_if_name(if_name.clone())
    });

    for route in routes {
        manager.add(&route).map_err(|e| RouteError::AddFailed {
            destination: route.destination().to_string(),
            message: e.to_string(),
        })?;
    }

    Ok(())
}
