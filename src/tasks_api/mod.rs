//! Tasks_api Service
//!
//! Auto-generated service module for tasks_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for tasks_api
pub struct Tasks_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Tasks_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get task resource handler
    pub fn task(&self) -> resources::Task<'_> {
        resources::Task::new(self.provider)
    }
    /// Get tasklist resource handler
    pub fn tasklist(&self) -> resources::Tasklist<'_> {
        resources::Tasklist::new(self.provider)
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
