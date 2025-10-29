//! Retail Service
//!
//! Auto-generated service module for retail

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for retail
pub struct RetailService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> RetailService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get product resource handler
    pub fn product(&self) -> resources::Product<'_> {
        resources::Product::new(self.provider)
    }
    /// Get generative_question resource handler
    pub fn generative_question(&self) -> resources::Generative_question<'_> {
        resources::Generative_question::new(self.provider)
    }
    /// Get control resource handler
    pub fn control(&self) -> resources::Control<'_> {
        resources::Control::new(self.provider)
    }
    /// Get project resource handler
    pub fn project(&self) -> resources::Project<'_> {
        resources::Project::new(self.provider)
    }
    /// Get operation resource handler
    pub fn operation(&self) -> resources::Operation<'_> {
        resources::Operation::new(self.provider)
    }
    /// Get branche resource handler
    pub fn branche(&self) -> resources::Branche<'_> {
        resources::Branche::new(self.provider)
    }
    /// Get catalog resource handler
    pub fn catalog(&self) -> resources::Catalog<'_> {
        resources::Catalog::new(self.provider)
    }
    /// Get serving_config resource handler
    pub fn serving_config(&self) -> resources::Serving_config<'_> {
        resources::Serving_config::new(self.provider)
    }
    /// Get attributes_config resource handler
    pub fn attributes_config(&self) -> resources::Attributes_config<'_> {
        resources::Attributes_config::new(self.provider)
    }
    /// Get placement resource handler
    pub fn placement(&self) -> resources::Placement<'_> {
        resources::Placement::new(self.provider)
    }
    /// Get user_event resource handler
    pub fn user_event(&self) -> resources::User_event<'_> {
        resources::User_event::new(self.provider)
    }
    /// Get merchant_center_account_link resource handler
    pub fn merchant_center_account_link(&self) -> resources::Merchant_center_account_link<'_> {
        resources::Merchant_center_account_link::new(self.provider)
    }
    /// Get retail_project resource handler
    pub fn retail_project(&self) -> resources::Retail_project<'_> {
        resources::Retail_project::new(self.provider)
    }
    /// Get model resource handler
    pub fn model(&self) -> resources::Model<'_> {
        resources::Model::new(self.provider)
    }
    /// Get completion_data resource handler
    pub fn completion_data(&self) -> resources::Completion_data<'_> {
        resources::Completion_data::new(self.provider)
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
