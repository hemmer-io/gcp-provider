//! Plusdomains Service
//!
//! Auto-generated service module for plusdomains

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for plusdomains
pub struct PlusdomainsService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> PlusdomainsService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get people resource handler
    pub fn people(&self) -> resources::People<'_> {
        resources::People::new(self.provider)
    }
    /// Get circle resource handler
    pub fn circle(&self) -> resources::Circle<'_> {
        resources::Circle::new(self.provider)
    }
    /// Get media resource handler
    pub fn media(&self) -> resources::Media<'_> {
        resources::Media::new(self.provider)
    }
    /// Get audience resource handler
    pub fn audience(&self) -> resources::Audience<'_> {
        resources::Audience::new(self.provider)
    }
    /// Get activitie resource handler
    pub fn activitie(&self) -> resources::Activitie<'_> {
        resources::Activitie::new(self.provider)
    }
    /// Get comment resource handler
    pub fn comment(&self) -> resources::Comment<'_> {
        resources::Comment::new(self.provider)
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
