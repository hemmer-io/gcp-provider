//! Managedkafka Service
//!
//! Auto-generated service module for managedkafka

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for managedkafka
pub struct ManagedkafkaService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> ManagedkafkaService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get version resource handler
    pub fn version(&self) -> resources::Version<'_> {
        resources::Version::new(self.provider)
    }
    /// Get acl resource handler
    pub fn acl(&self) -> resources::Acl<'_> {
        resources::Acl::new(self.provider)
    }
    /// Get config resource handler
    pub fn config(&self) -> resources::Config<'_> {
        resources::Config::new(self.provider)
    }
    /// Get schema resource handler
    pub fn schema(&self) -> resources::Schema<'_> {
        resources::Schema::new(self.provider)
    }
    /// Get connect_cluster resource handler
    pub fn connect_cluster(&self) -> resources::Connect_cluster<'_> {
        resources::Connect_cluster::new(self.provider)
    }
    /// Get schema_registrie resource handler
    pub fn schema_registrie(&self) -> resources::Schema_registrie<'_> {
        resources::Schema_registrie::new(self.provider)
    }
    /// Get cluster resource handler
    pub fn cluster(&self) -> resources::Cluster<'_> {
        resources::Cluster::new(self.provider)
    }
    /// Get location resource handler
    pub fn location(&self) -> resources::Location<'_> {
        resources::Location::new(self.provider)
    }
    /// Get subject resource handler
    pub fn subject(&self) -> resources::Subject<'_> {
        resources::Subject::new(self.provider)
    }
    /// Get type resource handler
    pub fn type(&self) -> resources::Type<'_> {
        resources::Type::new(self.provider)
    }
    /// Get context resource handler
    pub fn context(&self) -> resources::Context<'_> {
        resources::Context::new(self.provider)
    }
    /// Get compatibility resource handler
    pub fn compatibility(&self) -> resources::Compatibility<'_> {
        resources::Compatibility::new(self.provider)
    }
    /// Get topic resource handler
    pub fn topic(&self) -> resources::Topic<'_> {
        resources::Topic::new(self.provider)
    }
    /// Get mode resource handler
    pub fn mode(&self) -> resources::Mode<'_> {
        resources::Mode::new(self.provider)
    }
    /// Get connector resource handler
    pub fn connector(&self) -> resources::Connector<'_> {
        resources::Connector::new(self.provider)
    }
    /// Get referencedby resource handler
    pub fn referencedby(&self) -> resources::Referencedby<'_> {
        resources::Referencedby::new(self.provider)
    }
    /// Get consumer_group resource handler
    pub fn consumer_group(&self) -> resources::Consumer_group<'_> {
        resources::Consumer_group::new(self.provider)
    }
    /// Get operation resource handler
    pub fn operation(&self) -> resources::Operation<'_> {
        resources::Operation::new(self.provider)
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
