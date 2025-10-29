//! Forms Service
//!
//! Auto-generated service module for forms

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for forms
pub struct FormsService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> FormsService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get form resource handler
    pub fn form(&self) -> resources::Form<'_> {
        resources::Form::new(self.provider)
    }
    /// Get watche resource handler
    pub fn watche(&self) -> resources::Watche<'_> {
        resources::Watche::new(self.provider)
    }
    /// Get response resource handler
    pub fn response(&self) -> resources::Response<'_> {
        resources::Response::new(self.provider)
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
