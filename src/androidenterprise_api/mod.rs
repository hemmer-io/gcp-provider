//! Androidenterprise_api Service
//!
//! Auto-generated service module for androidenterprise_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for androidenterprise_api
pub struct Androidenterprise_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Androidenterprise_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get entitlement resource handler
    pub fn entitlement(&self) -> resources::Entitlement<'_> {
        resources::Entitlement::new(self.provider)
    }
    /// Get permission resource handler
    pub fn permission(&self) -> resources::Permission<'_> {
        resources::Permission::new(self.provider)
    }
    /// Get webapp resource handler
    pub fn webapp(&self) -> resources::Webapp<'_> {
        resources::Webapp::new(self.provider)
    }
    /// Get serviceaccountkey resource handler
    pub fn serviceaccountkey(&self) -> resources::Serviceaccountkey<'_> {
        resources::Serviceaccountkey::new(self.provider)
    }
    /// Get grouplicenseuser resource handler
    pub fn grouplicenseuser(&self) -> resources::Grouplicenseuser<'_> {
        resources::Grouplicenseuser::new(self.provider)
    }
    /// Get install resource handler
    pub fn install(&self) -> resources::Install<'_> {
        resources::Install::new(self.provider)
    }
    /// Get storelayoutpage resource handler
    pub fn storelayoutpage(&self) -> resources::Storelayoutpage<'_> {
        resources::Storelayoutpage::new(self.provider)
    }
    /// Get managedconfigurationssetting resource handler
    pub fn managedconfigurationssetting(&self) -> resources::Managedconfigurationssetting<'_> {
        resources::Managedconfigurationssetting::new(self.provider)
    }
    /// Get user resource handler
    pub fn user(&self) -> resources::User<'_> {
        resources::User::new(self.provider)
    }
    /// Get grouplicense resource handler
    pub fn grouplicense(&self) -> resources::Grouplicense<'_> {
        resources::Grouplicense::new(self.provider)
    }
    /// Get enrollment_token resource handler
    pub fn enrollment_token(&self) -> resources::Enrollment_token<'_> {
        resources::Enrollment_token::new(self.provider)
    }
    /// Get device resource handler
    pub fn device(&self) -> resources::Device<'_> {
        resources::Device::new(self.provider)
    }
    /// Get storelayoutcluster resource handler
    pub fn storelayoutcluster(&self) -> resources::Storelayoutcluster<'_> {
        resources::Storelayoutcluster::new(self.provider)
    }
    /// Get managedconfigurationsforuser resource handler
    pub fn managedconfigurationsforuser(&self) -> resources::Managedconfigurationsforuser<'_> {
        resources::Managedconfigurationsforuser::new(self.provider)
    }
    /// Get product resource handler
    pub fn product(&self) -> resources::Product<'_> {
        resources::Product::new(self.provider)
    }
    /// Get managedconfigurationsfordevice resource handler
    pub fn managedconfigurationsfordevice(&self) -> resources::Managedconfigurationsfordevice<'_> {
        resources::Managedconfigurationsfordevice::new(self.provider)
    }
    /// Get enterprise resource handler
    pub fn enterprise(&self) -> resources::Enterprise<'_> {
        resources::Enterprise::new(self.provider)
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
