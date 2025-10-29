//! Healthcare Service
//!
//! Auto-generated service module for healthcare

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for healthcare
pub struct HealthcareService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> HealthcareService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get location resource handler
    pub fn location(&self) -> resources::Location<'_> {
        resources::Location::new(self.provider)
    }
    /// Get message resource handler
    pub fn message(&self) -> resources::Message<'_> {
        resources::Message::new(self.provider)
    }
    /// Get hl7_v2_store resource handler
    pub fn hl7_v2_store(&self) -> resources::Hl7_v2_store<'_> {
        resources::Hl7_v2_store::new(self.provider)
    }
    /// Get fhir resource handler
    pub fn fhir(&self) -> resources::Fhir<'_> {
        resources::Fhir::new(self.provider)
    }
    /// Get operation resource handler
    pub fn operation(&self) -> resources::Operation<'_> {
        resources::Operation::new(self.provider)
    }
    /// Get dicom_store resource handler
    pub fn dicom_store(&self) -> resources::Dicom_store<'_> {
        resources::Dicom_store::new(self.provider)
    }
    /// Get consent_artifact resource handler
    pub fn consent_artifact(&self) -> resources::Consent_artifact<'_> {
        resources::Consent_artifact::new(self.provider)
    }
    /// Get dataset resource handler
    pub fn dataset(&self) -> resources::Dataset<'_> {
        resources::Dataset::new(self.provider)
    }
    /// Get studie resource handler
    pub fn studie(&self) -> resources::Studie<'_> {
        resources::Studie::new(self.provider)
    }
    /// Get serie resource handler
    pub fn serie(&self) -> resources::Serie<'_> {
        resources::Serie::new(self.provider)
    }
    /// Get data_mapper_workspace resource handler
    pub fn data_mapper_workspace(&self) -> resources::Data_mapper_workspace<'_> {
        resources::Data_mapper_workspace::new(self.provider)
    }
    /// Get bulkdata resource handler
    pub fn bulkdata(&self) -> resources::Bulkdata<'_> {
        resources::Bulkdata::new(self.provider)
    }
    /// Get attribute_definition resource handler
    pub fn attribute_definition(&self) -> resources::Attribute_definition<'_> {
        resources::Attribute_definition::new(self.provider)
    }
    /// Get consent_store resource handler
    pub fn consent_store(&self) -> resources::Consent_store<'_> {
        resources::Consent_store::new(self.provider)
    }
    /// Get nlp resource handler
    pub fn nlp(&self) -> resources::Nlp<'_> {
        resources::Nlp::new(self.provider)
    }
    /// Get consent resource handler
    pub fn consent(&self) -> resources::Consent<'_> {
        resources::Consent::new(self.provider)
    }
    /// Get user_data_mapping resource handler
    pub fn user_data_mapping(&self) -> resources::User_data_mapping<'_> {
        resources::User_data_mapping::new(self.provider)
    }
    /// Get instance resource handler
    pub fn instance(&self) -> resources::Instance<'_> {
        resources::Instance::new(self.provider)
    }
    /// Get fhir_store resource handler
    pub fn fhir_store(&self) -> resources::Fhir_store<'_> {
        resources::Fhir_store::new(self.provider)
    }
    /// Get frame resource handler
    pub fn frame(&self) -> resources::Frame<'_> {
        resources::Frame::new(self.provider)
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
