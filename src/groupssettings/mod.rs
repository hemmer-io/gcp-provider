//! Groupssettings Service
//!
//! Auto-generated service module for groupssettings

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for groupssettings
pub struct GroupssettingsService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> GroupssettingsService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get group resource handler
    pub fn group(&self) -> resources::Group<'_> {
        resources::Group::new(self.provider)
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
