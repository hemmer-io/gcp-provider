//! Gamesmanagement_api Service
//!
//! Auto-generated service module for gamesmanagement_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for gamesmanagement_api
pub struct Gamesmanagement_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Gamesmanagement_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get achievement resource handler
    pub fn achievement(&self) -> resources::Achievement<'_> {
        resources::Achievement::new(self.provider)
    }
    /// Get application resource handler
    pub fn application(&self) -> resources::Application<'_> {
        resources::Application::new(self.provider)
    }
    /// Get event resource handler
    pub fn event(&self) -> resources::Event<'_> {
        resources::Event::new(self.provider)
    }
    /// Get score resource handler
    pub fn score(&self) -> resources::Score<'_> {
        resources::Score::new(self.provider)
    }
    /// Get player resource handler
    pub fn player(&self) -> resources::Player<'_> {
        resources::Player::new(self.provider)
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
