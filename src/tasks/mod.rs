//! Tasks Service
//!
//! Auto-generated service module for tasks

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for tasks
pub struct TasksService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> TasksService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get tasklist resource handler
    pub fn tasklist(&self) -> resources::Tasklist<'_> {
        resources::Tasklist::new(self.provider)
    }
    /// Get task resource handler
    pub fn task(&self) -> resources::Task<'_> {
        resources::Task::new(self.provider)
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
