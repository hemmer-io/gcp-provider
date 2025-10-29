//! Datacatalog Service
//!
//! Auto-generated service module for datacatalog

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for datacatalog
pub struct DatacatalogService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> DatacatalogService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get entrie resource handler
    pub fn entrie(&self) -> resources::Entrie<'_> {
        resources::Entrie::new(self.provider)
    }
    /// Get tag_template resource handler
    pub fn tag_template(&self) -> resources::Tag_template<'_> {
        resources::Tag_template::new(self.provider)
    }
    /// Get policy_tag resource handler
    pub fn policy_tag(&self) -> resources::Policy_tag<'_> {
        resources::Policy_tag::new(self.provider)
    }
    /// Get tag resource handler
    pub fn tag(&self) -> resources::Tag<'_> {
        resources::Tag::new(self.provider)
    }
    /// Get taxonomie resource handler
    pub fn taxonomie(&self) -> resources::Taxonomie<'_> {
        resources::Taxonomie::new(self.provider)
    }
    /// Get entry_group resource handler
    pub fn entry_group(&self) -> resources::Entry_group<'_> {
        resources::Entry_group::new(self.provider)
    }
    /// Get field resource handler
    pub fn field(&self) -> resources::Field<'_> {
        resources::Field::new(self.provider)
    }
    /// Get enum_value resource handler
    pub fn enum_value(&self) -> resources::Enum_value<'_> {
        resources::Enum_value::new(self.provider)
    }
    /// Get catalog resource handler
    pub fn catalog(&self) -> resources::Catalog<'_> {
        resources::Catalog::new(self.provider)
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
