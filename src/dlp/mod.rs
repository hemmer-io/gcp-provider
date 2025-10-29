//! Dlp Service
//!
//! Auto-generated service module for dlp

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for dlp
pub struct DlpService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> DlpService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get info_type resource handler
    pub fn info_type(&self) -> resources::Info_type<'_> {
        resources::Info_type::new(self.provider)
    }
    /// Get job_trigger resource handler
    pub fn job_trigger(&self) -> resources::Job_trigger<'_> {
        resources::Job_trigger::new(self.provider)
    }
    /// Get project_data_profile resource handler
    pub fn project_data_profile(&self) -> resources::Project_data_profile<'_> {
        resources::Project_data_profile::new(self.provider)
    }
    /// Get column_data_profile resource handler
    pub fn column_data_profile(&self) -> resources::Column_data_profile<'_> {
        resources::Column_data_profile::new(self.provider)
    }
    /// Get file_store_data_profile resource handler
    pub fn file_store_data_profile(&self) -> resources::File_store_data_profile<'_> {
        resources::File_store_data_profile::new(self.provider)
    }
    /// Get discovery_config resource handler
    pub fn discovery_config(&self) -> resources::Discovery_config<'_> {
        resources::Discovery_config::new(self.provider)
    }
    /// Get deidentify_template resource handler
    pub fn deidentify_template(&self) -> resources::Deidentify_template<'_> {
        resources::Deidentify_template::new(self.provider)
    }
    /// Get image resource handler
    pub fn image(&self) -> resources::Image<'_> {
        resources::Image::new(self.provider)
    }
    /// Get inspect_template resource handler
    pub fn inspect_template(&self) -> resources::Inspect_template<'_> {
        resources::Inspect_template::new(self.provider)
    }
    /// Get stored_info_type resource handler
    pub fn stored_info_type(&self) -> resources::Stored_info_type<'_> {
        resources::Stored_info_type::new(self.provider)
    }
    /// Get table_data_profile resource handler
    pub fn table_data_profile(&self) -> resources::Table_data_profile<'_> {
        resources::Table_data_profile::new(self.provider)
    }
    /// Get dlp_job resource handler
    pub fn dlp_job(&self) -> resources::Dlp_job<'_> {
        resources::Dlp_job::new(self.provider)
    }
    /// Get content resource handler
    pub fn content(&self) -> resources::Content<'_> {
        resources::Content::new(self.provider)
    }
    /// Get connection resource handler
    pub fn connection(&self) -> resources::Connection<'_> {
        resources::Connection::new(self.provider)
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
