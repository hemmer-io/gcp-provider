//! Playgrouping Service
//!
//! Auto-generated service module for playgrouping

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for playgrouping
pub struct PlaygroupingService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> PlaygroupingService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get token resource handler
    pub fn token(&self) -> resources::Token<'_> {
        resources::Token::new(self.provider)
    }
    /// Get tag resource handler
    pub fn tag(&self) -> resources::Tag<'_> {
        resources::Tag::new(self.provider)
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
