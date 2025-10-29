//! Cloudtasks Service
//!
//! Auto-generated service module for cloudtasks

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for cloudtasks
pub struct CloudtasksService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> CloudtasksService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get location resource handler
    pub fn location(&self) -> resources::Location<'_> {
        resources::Location::new(self.provider)
    }
    /// Get task resource handler
    pub fn task(&self) -> resources::Task<'_> {
        resources::Task::new(self.provider)
    }
    /// Get queue resource handler
    pub fn queue(&self) -> resources::Queue<'_> {
        resources::Queue::new(self.provider)
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
