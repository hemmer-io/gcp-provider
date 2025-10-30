//! Firebaseappdistribution_api Service
//!
//! Auto-generated service module for firebaseappdistribution_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for firebaseappdistribution_api
pub struct Firebaseappdistribution_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Firebaseappdistribution_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get upload_statu resource handler
    pub fn upload_statu(&self) -> resources::Upload_statu<'_> {
        resources::Upload_statu::new(self.provider)
    }
    /// Get project resource handler
    pub fn project(&self) -> resources::Project<'_> {
        resources::Project::new(self.provider)
    }
    /// Get release resource handler
    pub fn release(&self) -> resources::Release<'_> {
        resources::Release::new(self.provider)
    }
    /// Get tester resource handler
    pub fn tester(&self) -> resources::Tester<'_> {
        resources::Tester::new(self.provider)
    }
    /// Get release_by_hash resource handler
    pub fn release_by_hash(&self) -> resources::Release_by_hash<'_> {
        resources::Release_by_hash::new(self.provider)
    }
    /// Get note resource handler
    pub fn note(&self) -> resources::Note<'_> {
        resources::Note::new(self.provider)
    }
    /// Get app resource handler
    pub fn app(&self) -> resources::App<'_> {
        resources::App::new(self.provider)
    }
    /// Get test resource handler
    pub fn test(&self) -> resources::Test<'_> {
        resources::Test::new(self.provider)
    }
    /// Get test_case resource handler
    pub fn test_case(&self) -> resources::Test_case<'_> {
        resources::Test_case::new(self.provider)
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
