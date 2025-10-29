//! Translate Service
//!
//! Auto-generated service module for translate

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for translate
pub struct TranslateService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> TranslateService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get adaptive_mt_dataset resource handler
    pub fn adaptive_mt_dataset(&self) -> resources::Adaptive_mt_dataset<'_> {
        resources::Adaptive_mt_dataset::new(self.provider)
    }
    /// Get adaptive_mt_sentence resource handler
    pub fn adaptive_mt_sentence(&self) -> resources::Adaptive_mt_sentence<'_> {
        resources::Adaptive_mt_sentence::new(self.provider)
    }
    /// Get location resource handler
    pub fn location(&self) -> resources::Location<'_> {
        resources::Location::new(self.provider)
    }
    /// Get dataset resource handler
    pub fn dataset(&self) -> resources::Dataset<'_> {
        resources::Dataset::new(self.provider)
    }
    /// Get project resource handler
    pub fn project(&self) -> resources::Project<'_> {
        resources::Project::new(self.provider)
    }
    /// Get operation resource handler
    pub fn operation(&self) -> resources::Operation<'_> {
        resources::Operation::new(self.provider)
    }
    /// Get adaptive_mt_file resource handler
    pub fn adaptive_mt_file(&self) -> resources::Adaptive_mt_file<'_> {
        resources::Adaptive_mt_file::new(self.provider)
    }
    /// Get glossary_entrie resource handler
    pub fn glossary_entrie(&self) -> resources::Glossary_entrie<'_> {
        resources::Glossary_entrie::new(self.provider)
    }
    /// Get glossarie resource handler
    pub fn glossarie(&self) -> resources::Glossarie<'_> {
        resources::Glossarie::new(self.provider)
    }
    /// Get example resource handler
    pub fn example(&self) -> resources::Example<'_> {
        resources::Example::new(self.provider)
    }
    /// Get model resource handler
    pub fn model(&self) -> resources::Model<'_> {
        resources::Model::new(self.provider)
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
