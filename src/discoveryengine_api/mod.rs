//! Discoveryengine_api Service
//!
//! Auto-generated service module for discoveryengine_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for discoveryengine_api
pub struct Discoveryengine_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Discoveryengine_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get schema resource handler
    pub fn schema(&self) -> resources::Schema<'_> {
        resources::Schema::new(self.provider)
    }
    /// Get suggestion_deny_list_entrie resource handler
    pub fn suggestion_deny_list_entrie(&self) -> resources::Suggestion_deny_list_entrie<'_> {
        resources::Suggestion_deny_list_entrie::new(self.provider)
    }
    /// Get operation resource handler
    pub fn operation(&self) -> resources::Operation<'_> {
        resources::Operation::new(self.provider)
    }
    /// Get custom_model resource handler
    pub fn custom_model(&self) -> resources::Custom_model<'_> {
        resources::Custom_model::new(self.provider)
    }
    /// Get evaluation resource handler
    pub fn evaluation(&self) -> resources::Evaluation<'_> {
        resources::Evaluation::new(self.provider)
    }
    /// Get completion_config resource handler
    pub fn completion_config(&self) -> resources::Completion_config<'_> {
        resources::Completion_config::new(self.provider)
    }
    /// Get branche resource handler
    pub fn branche(&self) -> resources::Branche<'_> {
        resources::Branche::new(self.provider)
    }
    /// Get media resource handler
    pub fn media(&self) -> resources::Media<'_> {
        resources::Media::new(self.provider)
    }
    /// Get serving_config resource handler
    pub fn serving_config(&self) -> resources::Serving_config<'_> {
        resources::Serving_config::new(self.provider)
    }
    /// Get document resource handler
    pub fn document(&self) -> resources::Document<'_> {
        resources::Document::new(self.provider)
    }
    /// Get data_store resource handler
    pub fn data_store(&self) -> resources::Data_store<'_> {
        resources::Data_store::new(self.provider)
    }
    /// Get identity_mapping_store resource handler
    pub fn identity_mapping_store(&self) -> resources::Identity_mapping_store<'_> {
        resources::Identity_mapping_store::new(self.provider)
    }
    /// Get session resource handler
    pub fn session(&self) -> resources::Session<'_> {
        resources::Session::new(self.provider)
    }
    /// Get sample_query_set resource handler
    pub fn sample_query_set(&self) -> resources::Sample_query_set<'_> {
        resources::Sample_query_set::new(self.provider)
    }
    /// Get location resource handler
    pub fn location(&self) -> resources::Location<'_> {
        resources::Location::new(self.provider)
    }
    /// Get project resource handler
    pub fn project(&self) -> resources::Project<'_> {
        resources::Project::new(self.provider)
    }
    /// Get control resource handler
    pub fn control(&self) -> resources::Control<'_> {
        resources::Control::new(self.provider)
    }
    /// Get license_config resource handler
    pub fn license_config(&self) -> resources::License_config<'_> {
        resources::License_config::new(self.provider)
    }
    /// Get user_store resource handler
    pub fn user_store(&self) -> resources::User_store<'_> {
        resources::User_store::new(self.provider)
    }
    /// Get sample_querie resource handler
    pub fn sample_querie(&self) -> resources::Sample_querie<'_> {
        resources::Sample_querie::new(self.provider)
    }
    /// Get ranking_config resource handler
    pub fn ranking_config(&self) -> resources::Ranking_config<'_> {
        resources::Ranking_config::new(self.provider)
    }
    /// Get engine resource handler
    pub fn engine(&self) -> resources::Engine<'_> {
        resources::Engine::new(self.provider)
    }
    /// Get assistant resource handler
    pub fn assistant(&self) -> resources::Assistant<'_> {
        resources::Assistant::new(self.provider)
    }
    /// Get user_event resource handler
    pub fn user_event(&self) -> resources::User_event<'_> {
        resources::User_event::new(self.provider)
    }
    /// Get user_license resource handler
    pub fn user_license(&self) -> resources::User_license<'_> {
        resources::User_license::new(self.provider)
    }
    /// Get conversation resource handler
    pub fn conversation(&self) -> resources::Conversation<'_> {
        resources::Conversation::new(self.provider)
    }
    /// Get answer resource handler
    pub fn answer(&self) -> resources::Answer<'_> {
        resources::Answer::new(self.provider)
    }
    /// Get completion_suggestion resource handler
    pub fn completion_suggestion(&self) -> resources::Completion_suggestion<'_> {
        resources::Completion_suggestion::new(self.provider)
    }
    /// Get target_site resource handler
    pub fn target_site(&self) -> resources::Target_site<'_> {
        resources::Target_site::new(self.provider)
    }
    /// Get site_search_engine resource handler
    pub fn site_search_engine(&self) -> resources::Site_search_engine<'_> {
        resources::Site_search_engine::new(self.provider)
    }
    /// Get grounding_config resource handler
    pub fn grounding_config(&self) -> resources::Grounding_config<'_> {
        resources::Grounding_config::new(self.provider)
    }
    /// Get cmek_config resource handler
    pub fn cmek_config(&self) -> resources::Cmek_config<'_> {
        resources::Cmek_config::new(self.provider)
    }
    /// Get sitemap resource handler
    pub fn sitemap(&self) -> resources::Sitemap<'_> {
        resources::Sitemap::new(self.provider)
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
