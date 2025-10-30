//! Youtubeanalytics_api Service
//!
//! Auto-generated service module for youtubeanalytics_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for youtubeanalytics_api
pub struct Youtubeanalytics_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Youtubeanalytics_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get group_item resource handler
    pub fn group_item(&self) -> resources::Group_item<'_> {
        resources::Group_item::new(self.provider)
    }
    /// Get group resource handler
    pub fn group(&self) -> resources::Group<'_> {
        resources::Group::new(self.provider)
    }
    /// Get report resource handler
    pub fn report(&self) -> resources::Report<'_> {
        resources::Report::new(self.provider)
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
