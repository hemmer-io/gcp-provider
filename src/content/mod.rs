//! Content Service
//!
//! Auto-generated service module for content

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for content
pub struct ContentService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> ContentService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get datafeedstatuse resource handler
    pub fn datafeedstatuse(&self) -> resources::Datafeedstatuse<'_> {
        resources::Datafeedstatuse::new(self.provider)
    }
    /// Get product resource handler
    pub fn product(&self) -> resources::Product<'_> {
        resources::Product::new(self.provider)
    }
    /// Get shippingsetting resource handler
    pub fn shippingsetting(&self) -> resources::Shippingsetting<'_> {
        resources::Shippingsetting::new(self.provider)
    }
    /// Get orderinvoice resource handler
    pub fn orderinvoice(&self) -> resources::Orderinvoice<'_> {
        resources::Orderinvoice::new(self.provider)
    }
    /// Get datafeed resource handler
    pub fn datafeed(&self) -> resources::Datafeed<'_> {
        resources::Datafeed::new(self.provider)
    }
    /// Get accountstatuse resource handler
    pub fn accountstatuse(&self) -> resources::Accountstatuse<'_> {
        resources::Accountstatuse::new(self.provider)
    }
    /// Get accounttax resource handler
    pub fn accounttax(&self) -> resources::Accounttax<'_> {
        resources::Accounttax::new(self.provider)
    }
    /// Get po resource handler
    pub fn po(&self) -> resources::Po<'_> {
        resources::Po::new(self.provider)
    }
    /// Get account resource handler
    pub fn account(&self) -> resources::Account<'_> {
        resources::Account::new(self.provider)
    }
    /// Get orderreturn resource handler
    pub fn orderreturn(&self) -> resources::Orderreturn<'_> {
        resources::Orderreturn::new(self.provider)
    }
    /// Get order resource handler
    pub fn order(&self) -> resources::Order<'_> {
        resources::Order::new(self.provider)
    }
    /// Get liasetting resource handler
    pub fn liasetting(&self) -> resources::Liasetting<'_> {
        resources::Liasetting::new(self.provider)
    }
    /// Get orderreport resource handler
    pub fn orderreport(&self) -> resources::Orderreport<'_> {
        resources::Orderreport::new(self.provider)
    }
    /// Get productstatuse resource handler
    pub fn productstatuse(&self) -> resources::Productstatuse<'_> {
        resources::Productstatuse::new(self.provider)
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
