//! Bigquery Service
//!
//! Auto-generated service module for bigquery

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for bigquery
pub struct BigqueryService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> BigqueryService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get model resource handler
    pub fn model(&self) -> resources::Model<'_> {
        resources::Model::new(self.provider)
    }
    /// Get project resource handler
    pub fn project(&self) -> resources::Project<'_> {
        resources::Project::new(self.provider)
    }
    /// Get routine resource handler
    pub fn routine(&self) -> resources::Routine<'_> {
        resources::Routine::new(self.provider)
    }
    /// Get tabledata resource handler
    pub fn tabledata(&self) -> resources::Tabledata<'_> {
        resources::Tabledata::new(self.provider)
    }
    /// Get dataset resource handler
    pub fn dataset(&self) -> resources::Dataset<'_> {
        resources::Dataset::new(self.provider)
    }
    /// Get job resource handler
    pub fn job(&self) -> resources::Job<'_> {
        resources::Job::new(self.provider)
    }
    /// Get table resource handler
    pub fn table(&self) -> resources::Table<'_> {
        resources::Table::new(self.provider)
    }
    /// Get row_access_policie resource handler
    pub fn row_access_policie(&self) -> resources::Row_access_policie<'_> {
        resources::Row_access_policie::new(self.provider)
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
