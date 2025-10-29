//! Games Service
//!
//! Auto-generated service module for games

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for games
pub struct GamesService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> GamesService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get score resource handler
    pub fn score(&self) -> resources::Score<'_> {
        resources::Score::new(self.provider)
    }
    /// Get achievement_definition resource handler
    pub fn achievement_definition(&self) -> resources::Achievement_definition<'_> {
        resources::Achievement_definition::new(self.provider)
    }
    /// Get event resource handler
    pub fn event(&self) -> resources::Event<'_> {
        resources::Event::new(self.provider)
    }
    /// Get revision resource handler
    pub fn revision(&self) -> resources::Revision<'_> {
        resources::Revision::new(self.provider)
    }
    /// Get snapshot resource handler
    pub fn snapshot(&self) -> resources::Snapshot<'_> {
        resources::Snapshot::new(self.provider)
    }
    /// Get metagame resource handler
    pub fn metagame(&self) -> resources::Metagame<'_> {
        resources::Metagame::new(self.provider)
    }
    /// Get leaderboard resource handler
    pub fn leaderboard(&self) -> resources::Leaderboard<'_> {
        resources::Leaderboard::new(self.provider)
    }
    /// Get stat resource handler
    pub fn stat(&self) -> resources::Stat<'_> {
        resources::Stat::new(self.provider)
    }
    /// Get application resource handler
    pub fn application(&self) -> resources::Application<'_> {
        resources::Application::new(self.provider)
    }
    /// Get accesstoken resource handler
    pub fn accesstoken(&self) -> resources::Accesstoken<'_> {
        resources::Accesstoken::new(self.provider)
    }
    /// Get recall resource handler
    pub fn recall(&self) -> resources::Recall<'_> {
        resources::Recall::new(self.provider)
    }
    /// Get achievement resource handler
    pub fn achievement(&self) -> resources::Achievement<'_> {
        resources::Achievement::new(self.provider)
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
