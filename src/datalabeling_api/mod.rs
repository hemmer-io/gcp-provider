//! Datalabeling_api Service
//!
//! Auto-generated service module for datalabeling_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for datalabeling_api
pub struct Datalabeling_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Datalabeling_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get video resource handler
    pub fn video(&self) -> resources::Video<'_> {
        resources::Video::new(self.provider)
    }
    /// Get operation resource handler
    pub fn operation(&self) -> resources::Operation<'_> {
        resources::Operation::new(self.provider)
    }
    /// Get annotated_dataset resource handler
    pub fn annotated_dataset(&self) -> resources::Annotated_dataset<'_> {
        resources::Annotated_dataset::new(self.provider)
    }
    /// Get instruction resource handler
    pub fn instruction(&self) -> resources::Instruction<'_> {
        resources::Instruction::new(self.provider)
    }
    /// Get example resource handler
    pub fn example(&self) -> resources::Example<'_> {
        resources::Example::new(self.provider)
    }
    /// Get evaluation_job resource handler
    pub fn evaluation_job(&self) -> resources::Evaluation_job<'_> {
        resources::Evaluation_job::new(self.provider)
    }
    /// Get image resource handler
    pub fn image(&self) -> resources::Image<'_> {
        resources::Image::new(self.provider)
    }
    /// Get data_item resource handler
    pub fn data_item(&self) -> resources::Data_item<'_> {
        resources::Data_item::new(self.provider)
    }
    /// Get feedback_thread resource handler
    pub fn feedback_thread(&self) -> resources::Feedback_thread<'_> {
        resources::Feedback_thread::new(self.provider)
    }
    /// Get example_comparison resource handler
    pub fn example_comparison(&self) -> resources::Example_comparison<'_> {
        resources::Example_comparison::new(self.provider)
    }
    /// Get text resource handler
    pub fn text(&self) -> resources::Text<'_> {
        resources::Text::new(self.provider)
    }
    /// Get annotation_spec_set resource handler
    pub fn annotation_spec_set(&self) -> resources::Annotation_spec_set<'_> {
        resources::Annotation_spec_set::new(self.provider)
    }
    /// Get dataset resource handler
    pub fn dataset(&self) -> resources::Dataset<'_> {
        resources::Dataset::new(self.provider)
    }
    /// Get feedback_message resource handler
    pub fn feedback_message(&self) -> resources::Feedback_message<'_> {
        resources::Feedback_message::new(self.provider)
    }
    /// Get evaluation resource handler
    pub fn evaluation(&self) -> resources::Evaluation<'_> {
        resources::Evaluation::new(self.provider)
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
