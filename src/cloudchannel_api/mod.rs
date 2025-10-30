//! Cloudchannel_api Service
//!
//! Auto-generated service module for cloudchannel_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for cloudchannel_api
pub struct Cloudchannel_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Cloudchannel_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get entitlement resource handler
    pub fn entitlement(&self) -> resources::Entitlement<'_> {
        resources::Entitlement::new(self.provider)
    }
    /// Get billable_sku resource handler
    pub fn billable_sku(&self) -> resources::Billable_sku<'_> {
        resources::Billable_sku::new(self.provider)
    }
    /// Get sku_group resource handler
    pub fn sku_group(&self) -> resources::Sku_group<'_> {
        resources::Sku_group::new(self.provider)
    }
    /// Get account resource handler
    pub fn account(&self) -> resources::Account<'_> {
        resources::Account::new(self.provider)
    }
    /// Get channel_partner_link resource handler
    pub fn channel_partner_link(&self) -> resources::Channel_partner_link<'_> {
        resources::Channel_partner_link::new(self.provider)
    }
    /// Get report_job resource handler
    pub fn report_job(&self) -> resources::Report_job<'_> {
        resources::Report_job::new(self.provider)
    }
    /// Get sku resource handler
    pub fn sku(&self) -> resources::Sku<'_> {
        resources::Sku::new(self.provider)
    }
    /// Get customer_repricing_config resource handler
    pub fn customer_repricing_config(&self) -> resources::Customer_repricing_config<'_> {
        resources::Customer_repricing_config::new(self.provider)
    }
    /// Get product resource handler
    pub fn product(&self) -> resources::Product<'_> {
        resources::Product::new(self.provider)
    }
    /// Get customer resource handler
    pub fn customer(&self) -> resources::Customer<'_> {
        resources::Customer::new(self.provider)
    }
    /// Get offer resource handler
    pub fn offer(&self) -> resources::Offer<'_> {
        resources::Offer::new(self.provider)
    }
    /// Get report resource handler
    pub fn report(&self) -> resources::Report<'_> {
        resources::Report::new(self.provider)
    }
    /// Get channel_partner_repricing_config resource handler
    pub fn channel_partner_repricing_config(&self) -> resources::Channel_partner_repricing_config<'_> {
        resources::Channel_partner_repricing_config::new(self.provider)
    }
    /// Get operation resource handler
    pub fn operation(&self) -> resources::Operation<'_> {
        resources::Operation::new(self.provider)
    }
    /// Get integrator resource handler
    pub fn integrator(&self) -> resources::Integrator<'_> {
        resources::Integrator::new(self.provider)
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
