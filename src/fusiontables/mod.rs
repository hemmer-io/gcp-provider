//! Fusiontables Service
//!
//! Auto-generated service module for fusiontables

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for fusiontables
pub struct FusiontablesService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> FusiontablesService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get table resource handler
    pub fn table(&self) -> resources::Table<'_> {
        resources::Table::new(self.provider)
    }
    /// Get template resource handler
    pub fn template(&self) -> resources::Template<'_> {
        resources::Template::new(self.provider)
    }
    /// Get column resource handler
    pub fn column(&self) -> resources::Column<'_> {
        resources::Column::new(self.provider)
    }
    /// Get task resource handler
    pub fn task(&self) -> resources::Task<'_> {
        resources::Task::new(self.provider)
    }
    /// Get query resource handler
    pub fn query(&self) -> resources::Query<'_> {
        resources::Query::new(self.provider)
    }
    /// Get style resource handler
    pub fn style(&self) -> resources::Style<'_> {
        resources::Style::new(self.provider)
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
