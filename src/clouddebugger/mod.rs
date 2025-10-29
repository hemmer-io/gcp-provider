//! Clouddebugger Service
//!
//! Auto-generated service module for clouddebugger

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for clouddebugger
pub struct ClouddebuggerService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> ClouddebuggerService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get debuggee resource handler
    pub fn debuggee(&self) -> resources::Debuggee<'_> {
        resources::Debuggee::new(self.provider)
    }
    /// Get breakpoint resource handler
    pub fn breakpoint(&self) -> resources::Breakpoint<'_> {
        resources::Breakpoint::new(self.provider)
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
