//! Playdeveloperreporting_api Service
//!
//! Auto-generated service module for playdeveloperreporting_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for playdeveloperreporting_api
pub struct Playdeveloperreporting_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Playdeveloperreporting_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get lmkrate resource handler
    pub fn lmkrate(&self) -> resources::Lmkrate<'_> {
        resources::Lmkrate::new(self.provider)
    }
    /// Get anomalie resource handler
    pub fn anomalie(&self) -> resources::Anomalie<'_> {
        resources::Anomalie::new(self.provider)
    }
    /// Get slowrenderingrate resource handler
    pub fn slowrenderingrate(&self) -> resources::Slowrenderingrate<'_> {
        resources::Slowrenderingrate::new(self.provider)
    }
    /// Get stuckbackgroundwakelockrate resource handler
    pub fn stuckbackgroundwakelockrate(&self) -> resources::Stuckbackgroundwakelockrate<'_> {
        resources::Stuckbackgroundwakelockrate::new(self.provider)
    }
    /// Get count resource handler
    pub fn count(&self) -> resources::Count<'_> {
        resources::Count::new(self.provider)
    }
    /// Get report resource handler
    pub fn report(&self) -> resources::Report<'_> {
        resources::Report::new(self.provider)
    }
    /// Get app resource handler
    pub fn app(&self) -> resources::App<'_> {
        resources::App::new(self.provider)
    }
    /// Get excessivewakeuprate resource handler
    pub fn excessivewakeuprate(&self) -> resources::Excessivewakeuprate<'_> {
        resources::Excessivewakeuprate::new(self.provider)
    }
    /// Get slowstartrate resource handler
    pub fn slowstartrate(&self) -> resources::Slowstartrate<'_> {
        resources::Slowstartrate::new(self.provider)
    }
    /// Get issue resource handler
    pub fn issue(&self) -> resources::Issue<'_> {
        resources::Issue::new(self.provider)
    }
    /// Get crashrate resource handler
    pub fn crashrate(&self) -> resources::Crashrate<'_> {
        resources::Crashrate::new(self.provider)
    }
    /// Get anrrate resource handler
    pub fn anrrate(&self) -> resources::Anrrate<'_> {
        resources::Anrrate::new(self.provider)
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
