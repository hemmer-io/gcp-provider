//! Vmwareengine Service
//!
//! Auto-generated service module for vmwareengine

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for vmwareengine
pub struct VmwareengineService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> VmwareengineService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get operation resource handler
    pub fn operation(&self) -> resources::Operation<'_> {
        resources::Operation::new(self.provider)
    }
    /// Get network_policie resource handler
    pub fn network_policie(&self) -> resources::Network_policie<'_> {
        resources::Network_policie::new(self.provider)
    }
    /// Get dns_bind_permission resource handler
    pub fn dns_bind_permission(&self) -> resources::Dns_bind_permission<'_> {
        resources::Dns_bind_permission::new(self.provider)
    }
    /// Get external_access_rule resource handler
    pub fn external_access_rule(&self) -> resources::External_access_rule<'_> {
        resources::External_access_rule::new(self.provider)
    }
    /// Get cluster resource handler
    pub fn cluster(&self) -> resources::Cluster<'_> {
        resources::Cluster::new(self.provider)
    }
    /// Get node resource handler
    pub fn node(&self) -> resources::Node<'_> {
        resources::Node::new(self.provider)
    }
    /// Get peering_route resource handler
    pub fn peering_route(&self) -> resources::Peering_route<'_> {
        resources::Peering_route::new(self.provider)
    }
    /// Get external_addresse resource handler
    pub fn external_addresse(&self) -> resources::External_addresse<'_> {
        resources::External_addresse::new(self.provider)
    }
    /// Get logging_server resource handler
    pub fn logging_server(&self) -> resources::Logging_server<'_> {
        resources::Logging_server::new(self.provider)
    }
    /// Get network_peering resource handler
    pub fn network_peering(&self) -> resources::Network_peering<'_> {
        resources::Network_peering::new(self.provider)
    }
    /// Get node_type resource handler
    pub fn node_type(&self) -> resources::Node_type<'_> {
        resources::Node_type::new(self.provider)
    }
    /// Get upgrade resource handler
    pub fn upgrade(&self) -> resources::Upgrade<'_> {
        resources::Upgrade::new(self.provider)
    }
    /// Get private_connection resource handler
    pub fn private_connection(&self) -> resources::Private_connection<'_> {
        resources::Private_connection::new(self.provider)
    }
    /// Get private_cloud resource handler
    pub fn private_cloud(&self) -> resources::Private_cloud<'_> {
        resources::Private_cloud::new(self.provider)
    }
    /// Get vmware_engine_network resource handler
    pub fn vmware_engine_network(&self) -> resources::Vmware_engine_network<'_> {
        resources::Vmware_engine_network::new(self.provider)
    }
    /// Get subnet resource handler
    pub fn subnet(&self) -> resources::Subnet<'_> {
        resources::Subnet::new(self.provider)
    }
    /// Get management_dns_zone_binding resource handler
    pub fn management_dns_zone_binding(&self) -> resources::Management_dns_zone_binding<'_> {
        resources::Management_dns_zone_binding::new(self.provider)
    }
    /// Get announcement resource handler
    pub fn announcement(&self) -> resources::Announcement<'_> {
        resources::Announcement::new(self.provider)
    }
    /// Get location resource handler
    pub fn location(&self) -> resources::Location<'_> {
        resources::Location::new(self.provider)
    }
    /// Get hcx_activation_key resource handler
    pub fn hcx_activation_key(&self) -> resources::Hcx_activation_key<'_> {
        resources::Hcx_activation_key::new(self.provider)
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
