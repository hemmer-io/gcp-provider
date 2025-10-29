//! Css Service
//!
//! Auto-generated service module for css

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for css
pub struct CssService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> CssService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get label resource handler
    pub fn label(&self) -> resources::Label<'_> {
        resources::Label::new(self.provider)
    }
    /// Get account resource handler
    pub fn account(&self) -> resources::Account<'_> {
        resources::Account::new(self.provider)
    }
    /// Get css_product resource handler
    pub fn css_product(&self) -> resources::Css_product<'_> {
        resources::Css_product::new(self.provider)
    }
    /// Get quota resource handler
    pub fn quota(&self) -> resources::Quota<'_> {
        resources::Quota::new(self.provider)
    }
    /// Get css_product_input resource handler
    pub fn css_product_input(&self) -> resources::Css_product_input<'_> {
        resources::Css_product_input::new(self.provider)
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
