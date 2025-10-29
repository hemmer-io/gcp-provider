//! Firebase Service
//!
//! Auto-generated service module for firebase

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for firebase
pub struct FirebaseService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> FirebaseService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get sha resource handler
    pub fn sha(&self) -> resources::Sha<'_> {
        resources::Sha::new(self.provider)
    }
    /// Get available_location resource handler
    pub fn available_location(&self) -> resources::Available_location<'_> {
        resources::Available_location::new(self.provider)
    }
    /// Get android_app resource handler
    pub fn android_app(&self) -> resources::Android_app<'_> {
        resources::Android_app::new(self.provider)
    }
    /// Get operation resource handler
    pub fn operation(&self) -> resources::Operation<'_> {
        resources::Operation::new(self.provider)
    }
    /// Get available_project resource handler
    pub fn available_project(&self) -> resources::Available_project<'_> {
        resources::Available_project::new(self.provider)
    }
    /// Get project resource handler
    pub fn project(&self) -> resources::Project<'_> {
        resources::Project::new(self.provider)
    }
    /// Get default_location resource handler
    pub fn default_location(&self) -> resources::Default_location<'_> {
        resources::Default_location::new(self.provider)
    }
    /// Get ios_app resource handler
    pub fn ios_app(&self) -> resources::Ios_app<'_> {
        resources::Ios_app::new(self.provider)
    }
    /// Get web_app resource handler
    pub fn web_app(&self) -> resources::Web_app<'_> {
        resources::Web_app::new(self.provider)
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
