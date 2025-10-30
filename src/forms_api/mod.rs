//! Forms_api Service
//!
//! Auto-generated service module for forms_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for forms_api
pub struct Forms_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Forms_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get response resource handler
    pub fn response(&self) -> resources::Response<'_> {
        resources::Response::new(self.provider)
    }
    /// Get watche resource handler
    pub fn watche(&self) -> resources::Watche<'_> {
        resources::Watche::new(self.provider)
    }
    /// Get form resource handler
    pub fn form(&self) -> resources::Form<'_> {
        resources::Form::new(self.provider)
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
