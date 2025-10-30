//! Alertcenter_api Service
//!
//! Auto-generated service module for alertcenter_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for alertcenter_api
pub struct Alertcenter_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Alertcenter_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get alertcenter resource handler
    pub fn alertcenter(&self) -> resources::Alertcenter<'_> {
        resources::Alertcenter::new(self.provider)
    }
    /// Get alert resource handler
    pub fn alert(&self) -> resources::Alert<'_> {
        resources::Alert::new(self.provider)
    }
    /// Get feedback resource handler
    pub fn feedback(&self) -> resources::Feedback<'_> {
        resources::Feedback::new(self.provider)
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
