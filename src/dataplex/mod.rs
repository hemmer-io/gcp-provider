//! Dataplex Service
//!
//! Auto-generated service module for dataplex

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for dataplex
pub struct DataplexService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> DataplexService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get contentitem resource handler
    pub fn contentitem(&self) -> resources::Contentitem<'_> {
        resources::Contentitem::new(self.provider)
    }
    /// Get encryption_config resource handler
    pub fn encryption_config(&self) -> resources::Encryption_config<'_> {
        resources::Encryption_config::new(self.provider)
    }
    /// Get environment resource handler
    pub fn environment(&self) -> resources::Environment<'_> {
        resources::Environment::new(self.provider)
    }
    /// Get operation resource handler
    pub fn operation(&self) -> resources::Operation<'_> {
        resources::Operation::new(self.provider)
    }
    /// Get job resource handler
    pub fn job(&self) -> resources::Job<'_> {
        resources::Job::new(self.provider)
    }
    /// Get session resource handler
    pub fn session(&self) -> resources::Session<'_> {
        resources::Session::new(self.provider)
    }
    /// Get lake resource handler
    pub fn lake(&self) -> resources::Lake<'_> {
        resources::Lake::new(self.provider)
    }
    /// Get data_product resource handler
    pub fn data_product(&self) -> resources::Data_product<'_> {
        resources::Data_product::new(self.provider)
    }
    /// Get categorie resource handler
    pub fn categorie(&self) -> resources::Categorie<'_> {
        resources::Categorie::new(self.provider)
    }
    /// Get data_scan resource handler
    pub fn data_scan(&self) -> resources::Data_scan<'_> {
        resources::Data_scan::new(self.provider)
    }
    /// Get term resource handler
    pub fn term(&self) -> resources::Term<'_> {
        resources::Term::new(self.provider)
    }
    /// Get asset resource handler
    pub fn asset(&self) -> resources::Asset<'_> {
        resources::Asset::new(self.provider)
    }
    /// Get location resource handler
    pub fn location(&self) -> resources::Location<'_> {
        resources::Location::new(self.provider)
    }
    /// Get data_attribute_binding resource handler
    pub fn data_attribute_binding(&self) -> resources::Data_attribute_binding<'_> {
        resources::Data_attribute_binding::new(self.provider)
    }
    /// Get entry_link_type resource handler
    pub fn entry_link_type(&self) -> resources::Entry_link_type<'_> {
        resources::Entry_link_type::new(self.provider)
    }
    /// Get content resource handler
    pub fn content(&self) -> resources::Content<'_> {
        resources::Content::new(self.provider)
    }
    /// Get change_request resource handler
    pub fn change_request(&self) -> resources::Change_request<'_> {
        resources::Change_request::new(self.provider)
    }
    /// Get action resource handler
    pub fn action(&self) -> resources::Action<'_> {
        resources::Action::new(self.provider)
    }
    /// Get aspect_type resource handler
    pub fn aspect_type(&self) -> resources::Aspect_type<'_> {
        resources::Aspect_type::new(self.provider)
    }
    /// Get metadata_job resource handler
    pub fn metadata_job(&self) -> resources::Metadata_job<'_> {
        resources::Metadata_job::new(self.provider)
    }
    /// Get entry_group resource handler
    pub fn entry_group(&self) -> resources::Entry_group<'_> {
        resources::Entry_group::new(self.provider)
    }
    /// Get glossarie resource handler
    pub fn glossarie(&self) -> resources::Glossarie<'_> {
        resources::Glossarie::new(self.provider)
    }
    /// Get entitie resource handler
    pub fn entitie(&self) -> resources::Entitie<'_> {
        resources::Entitie::new(self.provider)
    }
    /// Get entry_link resource handler
    pub fn entry_link(&self) -> resources::Entry_link<'_> {
        resources::Entry_link::new(self.provider)
    }
    /// Get attribute resource handler
    pub fn attribute(&self) -> resources::Attribute<'_> {
        resources::Attribute::new(self.provider)
    }
    /// Get entry_type resource handler
    pub fn entry_type(&self) -> resources::Entry_type<'_> {
        resources::Entry_type::new(self.provider)
    }
    /// Get governance_rule resource handler
    pub fn governance_rule(&self) -> resources::Governance_rule<'_> {
        resources::Governance_rule::new(self.provider)
    }
    /// Get task resource handler
    pub fn task(&self) -> resources::Task<'_> {
        resources::Task::new(self.provider)
    }
    /// Get zone resource handler
    pub fn zone(&self) -> resources::Zone<'_> {
        resources::Zone::new(self.provider)
    }
    /// Get partition resource handler
    pub fn partition(&self) -> resources::Partition<'_> {
        resources::Partition::new(self.provider)
    }
    /// Get entrie resource handler
    pub fn entrie(&self) -> resources::Entrie<'_> {
        resources::Entrie::new(self.provider)
    }
    /// Get data_asset resource handler
    pub fn data_asset(&self) -> resources::Data_asset<'_> {
        resources::Data_asset::new(self.provider)
    }
    /// Get data_taxonomie resource handler
    pub fn data_taxonomie(&self) -> resources::Data_taxonomie<'_> {
        resources::Data_taxonomie::new(self.provider)
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
