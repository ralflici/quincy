#[cfg(not(feature = "route-manager"))]
#[cfg(unix)]
mod posix;
#[cfg(not(feature = "route-manager"))]
#[cfg(unix)]
pub use posix::add_routes;

#[cfg(not(feature = "route-manager"))]
#[cfg(target_os = "windows")]
mod windows;
#[cfg(not(feature = "route-manager"))]
#[cfg(target_os = "windows")]
pub use windows::add_routes;

#[cfg(feature = "route-manager")]
mod unified;
#[cfg(feature = "route-manager")]
pub use unified::add_routes;
