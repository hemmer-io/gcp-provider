//! Libraryagent Service
//!
//! Auto-generated service module for libraryagent

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for libraryagent
pub struct LibraryagentService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> LibraryagentService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get book resource handler
    pub fn book(&self) -> resources::Book<'_> {
        resources::Book::new(self.provider)
    }
    /// Get shelve resource handler
    pub fn shelve(&self) -> resources::Shelve<'_> {
        resources::Shelve::new(self.provider)
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
