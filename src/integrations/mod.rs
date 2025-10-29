//! Integrations Service
//!
//! Auto-generated service module for integrations

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for integrations
pub struct IntegrationsService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> IntegrationsService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get runtime_entity_schema resource handler
    pub fn runtime_entity_schema(&self) -> resources::Runtime_entity_schema<'_> {
        resources::Runtime_entity_schema::new(self.provider)
    }
    /// Get integration resource handler
    pub fn integration(&self) -> resources::Integration<'_> {
        resources::Integration::new(self.provider)
    }
    /// Get suspension resource handler
    pub fn suspension(&self) -> resources::Suspension<'_> {
        resources::Suspension::new(self.provider)
    }
    /// Get callback resource handler
    pub fn callback(&self) -> resources::Callback<'_> {
        resources::Callback::new(self.provider)
    }
    /// Get connector_platform_region resource handler
    pub fn connector_platform_region(&self) -> resources::Connector_platform_region<'_> {
        resources::Connector_platform_region::new(self.provider)
    }
    /// Get execution resource handler
    pub fn execution(&self) -> resources::Execution<'_> {
        resources::Execution::new(self.provider)
    }
    /// Get sfdc_channel resource handler
    pub fn sfdc_channel(&self) -> resources::Sfdc_channel<'_> {
        resources::Sfdc_channel::new(self.provider)
    }
    /// Get auth_config resource handler
    pub fn auth_config(&self) -> resources::Auth_config<'_> {
        resources::Auth_config::new(self.provider)
    }
    /// Get certificate resource handler
    pub fn certificate(&self) -> resources::Certificate<'_> {
        resources::Certificate::new(self.provider)
    }
    /// Get runtime_action_schema resource handler
    pub fn runtime_action_schema(&self) -> resources::Runtime_action_schema<'_> {
        resources::Runtime_action_schema::new(self.provider)
    }
    /// Get connection resource handler
    pub fn connection(&self) -> resources::Connection<'_> {
        resources::Connection::new(self.provider)
    }
    /// Get version resource handler
    pub fn version(&self) -> resources::Version<'_> {
        resources::Version::new(self.provider)
    }
    /// Get sfdc_instance resource handler
    pub fn sfdc_instance(&self) -> resources::Sfdc_instance<'_> {
        resources::Sfdc_instance::new(self.provider)
    }
    /// Get apps_script_project resource handler
    pub fn apps_script_project(&self) -> resources::Apps_script_project<'_> {
        resources::Apps_script_project::new(self.provider)
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
