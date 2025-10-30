//! Gamesconfiguration_api Service
//!
//! Auto-generated service module for gamesconfiguration_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for gamesconfiguration_api
pub struct Gamesconfiguration_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Gamesconfiguration_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get leaderboard_configuration resource handler
    pub fn leaderboard_configuration(&self) -> resources::Leaderboard_configuration<'_> {
        resources::Leaderboard_configuration::new(self.provider)
    }
    /// Get achievement_configuration resource handler
    pub fn achievement_configuration(&self) -> resources::Achievement_configuration<'_> {
        resources::Achievement_configuration::new(self.provider)
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
