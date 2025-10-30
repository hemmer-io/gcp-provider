//! Admob_api Service
//!
//! Auto-generated service module for admob_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for admob_api
pub struct Admob_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Admob_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get mediation_ab_experiment resource handler
    pub fn mediation_ab_experiment(&self) -> resources::Mediation_ab_experiment<'_> {
        resources::Mediation_ab_experiment::new(self.provider)
    }
    /// Get campaign_report resource handler
    pub fn campaign_report(&self) -> resources::Campaign_report<'_> {
        resources::Campaign_report::new(self.provider)
    }
    /// Get mediation_group resource handler
    pub fn mediation_group(&self) -> resources::Mediation_group<'_> {
        resources::Mediation_group::new(self.provider)
    }
    /// Get ad_unit resource handler
    pub fn ad_unit(&self) -> resources::Ad_unit<'_> {
        resources::Ad_unit::new(self.provider)
    }
    /// Get ad_source resource handler
    pub fn ad_source(&self) -> resources::Ad_source<'_> {
        resources::Ad_source::new(self.provider)
    }
    /// Get adapter resource handler
    pub fn adapter(&self) -> resources::Adapter<'_> {
        resources::Adapter::new(self.provider)
    }
    /// Get ad_unit_mapping resource handler
    pub fn ad_unit_mapping(&self) -> resources::Ad_unit_mapping<'_> {
        resources::Ad_unit_mapping::new(self.provider)
    }
    /// Get app resource handler
    pub fn app(&self) -> resources::App<'_> {
        resources::App::new(self.provider)
    }
    /// Get mediation_report resource handler
    pub fn mediation_report(&self) -> resources::Mediation_report<'_> {
        resources::Mediation_report::new(self.provider)
    }
    /// Get network_report resource handler
    pub fn network_report(&self) -> resources::Network_report<'_> {
        resources::Network_report::new(self.provider)
    }
    /// Get account resource handler
    pub fn account(&self) -> resources::Account<'_> {
        resources::Account::new(self.provider)
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
