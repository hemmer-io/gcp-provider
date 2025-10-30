//! Networkservices_api Service
//!
//! Auto-generated service module for networkservices_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for networkservices_api
pub struct Networkservices_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Networkservices_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get tcp_route resource handler
    pub fn tcp_route(&self) -> resources::Tcp_route<'_> {
        resources::Tcp_route::new(self.provider)
    }
    /// Get service_lb_policie resource handler
    pub fn service_lb_policie(&self) -> resources::Service_lb_policie<'_> {
        resources::Service_lb_policie::new(self.provider)
    }
    /// Get gateway resource handler
    pub fn gateway(&self) -> resources::Gateway<'_> {
        resources::Gateway::new(self.provider)
    }
    /// Get version resource handler
    pub fn version(&self) -> resources::Version<'_> {
        resources::Version::new(self.provider)
    }
    /// Get lb_route_extension resource handler
    pub fn lb_route_extension(&self) -> resources::Lb_route_extension<'_> {
        resources::Lb_route_extension::new(self.provider)
    }
    /// Get grpc_route resource handler
    pub fn grpc_route(&self) -> resources::Grpc_route<'_> {
        resources::Grpc_route::new(self.provider)
    }
    /// Get authz_extension resource handler
    pub fn authz_extension(&self) -> resources::Authz_extension<'_> {
        resources::Authz_extension::new(self.provider)
    }
    /// Get http_route resource handler
    pub fn http_route(&self) -> resources::Http_route<'_> {
        resources::Http_route::new(self.provider)
    }
    /// Get lb_tcp_extension resource handler
    pub fn lb_tcp_extension(&self) -> resources::Lb_tcp_extension<'_> {
        resources::Lb_tcp_extension::new(self.provider)
    }
    /// Get lb_traffic_extension resource handler
    pub fn lb_traffic_extension(&self) -> resources::Lb_traffic_extension<'_> {
        resources::Lb_traffic_extension::new(self.provider)
    }
    /// Get lb_edge_extension resource handler
    pub fn lb_edge_extension(&self) -> resources::Lb_edge_extension<'_> {
        resources::Lb_edge_extension::new(self.provider)
    }
    /// Get meshe resource handler
    pub fn meshe(&self) -> resources::Meshe<'_> {
        resources::Meshe::new(self.provider)
    }
    /// Get endpoint_policie resource handler
    pub fn endpoint_policie(&self) -> resources::Endpoint_policie<'_> {
        resources::Endpoint_policie::new(self.provider)
    }
    /// Get location resource handler
    pub fn location(&self) -> resources::Location<'_> {
        resources::Location::new(self.provider)
    }
    /// Get tls_route resource handler
    pub fn tls_route(&self) -> resources::Tls_route<'_> {
        resources::Tls_route::new(self.provider)
    }
    /// Get service_binding resource handler
    pub fn service_binding(&self) -> resources::Service_binding<'_> {
        resources::Service_binding::new(self.provider)
    }
    /// Get operation resource handler
    pub fn operation(&self) -> resources::Operation<'_> {
        resources::Operation::new(self.provider)
    }
    /// Get route_view resource handler
    pub fn route_view(&self) -> resources::Route_view<'_> {
        resources::Route_view::new(self.provider)
    }
    /// Get wasm_plugin resource handler
    pub fn wasm_plugin(&self) -> resources::Wasm_plugin<'_> {
        resources::Wasm_plugin::new(self.provider)
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_service_creation() {
        // Service creation test
    }
}
