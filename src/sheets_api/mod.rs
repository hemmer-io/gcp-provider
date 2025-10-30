//! Sheets_api Service
//!
//! Auto-generated service module for sheets_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for sheets_api
pub struct Sheets_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Sheets_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get sheet resource handler
    pub fn sheet(&self) -> resources::Sheet<'_> {
        resources::Sheet::new(self.provider)
    }
    /// Get spreadsheet resource handler
    pub fn spreadsheet(&self) -> resources::Spreadsheet<'_> {
        resources::Spreadsheet::new(self.provider)
    }
    /// Get developer_metadata resource handler
    pub fn developer_metadata(&self) -> resources::Developer_metadata<'_> {
        resources::Developer_metadata::new(self.provider)
    }
    /// Get value resource handler
    pub fn value(&self) -> resources::Value<'_> {
        resources::Value::new(self.provider)
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
