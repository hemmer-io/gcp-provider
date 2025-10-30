//! Remotebuildexecution_api Service
//!
//! Auto-generated service module for remotebuildexecution_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for remotebuildexecution_api
pub struct Remotebuildexecution_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Remotebuildexecution_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get action resource handler
    pub fn action(&self) -> resources::Action<'_> {
        resources::Action::new(self.provider)
    }
    /// Get action_result resource handler
    pub fn action_result(&self) -> resources::Action_result<'_> {
        resources::Action_result::new(self.provider)
    }
    /// Get blob resource handler
    pub fn blob(&self) -> resources::Blob<'_> {
        resources::Blob::new(self.provider)
    }
    /// Get operation resource handler
    pub fn operation(&self) -> resources::Operation<'_> {
        resources::Operation::new(self.provider)
    }
    /// Get remotebuildexecution resource handler
    pub fn remotebuildexecution(&self) -> resources::Remotebuildexecution<'_> {
        resources::Remotebuildexecution::new(self.provider)
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
