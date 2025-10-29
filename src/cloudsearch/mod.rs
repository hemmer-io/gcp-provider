//! Cloudsearch Service
//!
//! Auto-generated service module for cloudsearch

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for cloudsearch
pub struct CloudsearchService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> CloudsearchService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get media resource handler
    pub fn media(&self) -> resources::Media<'_> {
        resources::Media::new(self.provider)
    }
    /// Get datasource resource handler
    pub fn datasource(&self) -> resources::Datasource<'_> {
        resources::Datasource::new(self.provider)
    }
    /// Get operation resource handler
    pub fn operation(&self) -> resources::Operation<'_> {
        resources::Operation::new(self.provider)
    }
    /// Get query resource handler
    pub fn query(&self) -> resources::Query<'_> {
        resources::Query::new(self.provider)
    }
    /// Get source resource handler
    pub fn source(&self) -> resources::Source<'_> {
        resources::Source::new(self.provider)
    }
    /// Get cloudsearch resource handler
    pub fn cloudsearch(&self) -> resources::Cloudsearch<'_> {
        resources::Cloudsearch::new(self.provider)
    }
    /// Get unmappedid resource handler
    pub fn unmappedid(&self) -> resources::Unmappedid<'_> {
        resources::Unmappedid::new(self.provider)
    }
    /// Get searchapplication resource handler
    pub fn searchapplication(&self) -> resources::Searchapplication<'_> {
        resources::Searchapplication::new(self.provider)
    }
    /// Get lro resource handler
    pub fn lro(&self) -> resources::Lro<'_> {
        resources::Lro::new(self.provider)
    }
    /// Get item resource handler
    pub fn item(&self) -> resources::Item<'_> {
        resources::Item::new(self.provider)
    }
    /// Get stat resource handler
    pub fn stat(&self) -> resources::Stat<'_> {
        resources::Stat::new(self.provider)
    }
    /// Get setting resource handler
    pub fn setting(&self) -> resources::Setting<'_> {
        resources::Setting::new(self.provider)
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
