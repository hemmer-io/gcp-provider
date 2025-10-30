//! Contactcenterinsights_api Service
//!
//! Auto-generated service module for contactcenterinsights_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for contactcenterinsights_api
pub struct Contactcenterinsights_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Contactcenterinsights_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get issue resource handler
    pub fn issue(&self) -> resources::Issue<'_> {
        resources::Issue::new(self.provider)
    }
    /// Get dataset resource handler
    pub fn dataset(&self) -> resources::Dataset<'_> {
        resources::Dataset::new(self.provider)
    }
    /// Get operation resource handler
    pub fn operation(&self) -> resources::Operation<'_> {
        resources::Operation::new(self.provider)
    }
    /// Get conversation resource handler
    pub fn conversation(&self) -> resources::Conversation<'_> {
        resources::Conversation::new(self.provider)
    }
    /// Get feedback_label resource handler
    pub fn feedback_label(&self) -> resources::Feedback_label<'_> {
        resources::Feedback_label::new(self.provider)
    }
    /// Get insightsdata resource handler
    pub fn insightsdata(&self) -> resources::Insightsdata<'_> {
        resources::Insightsdata::new(self.provider)
    }
    /// Get qa_scorecard resource handler
    pub fn qa_scorecard(&self) -> resources::Qa_scorecard<'_> {
        resources::Qa_scorecard::new(self.provider)
    }
    /// Get qa_question resource handler
    pub fn qa_question(&self) -> resources::Qa_question<'_> {
        resources::Qa_question::new(self.provider)
    }
    /// Get note resource handler
    pub fn note(&self) -> resources::Note<'_> {
        resources::Note::new(self.provider)
    }
    /// Get qa_question_tag resource handler
    pub fn qa_question_tag(&self) -> resources::Qa_question_tag<'_> {
        resources::Qa_question_tag::new(self.provider)
    }
    /// Get phrase_matcher resource handler
    pub fn phrase_matcher(&self) -> resources::Phrase_matcher<'_> {
        resources::Phrase_matcher::new(self.provider)
    }
    /// Get location resource handler
    pub fn location(&self) -> resources::Location<'_> {
        resources::Location::new(self.provider)
    }
    /// Get authorized_view resource handler
    pub fn authorized_view(&self) -> resources::Authorized_view<'_> {
        resources::Authorized_view::new(self.provider)
    }
    /// Get encryption_spec resource handler
    pub fn encryption_spec(&self) -> resources::Encryption_spec<'_> {
        resources::Encryption_spec::new(self.provider)
    }
    /// Get issue_model resource handler
    pub fn issue_model(&self) -> resources::Issue_model<'_> {
        resources::Issue_model::new(self.provider)
    }
    /// Get analyse resource handler
    pub fn analyse(&self) -> resources::Analyse<'_> {
        resources::Analyse::new(self.provider)
    }
    /// Get analysis_rule resource handler
    pub fn analysis_rule(&self) -> resources::Analysis_rule<'_> {
        resources::Analysis_rule::new(self.provider)
    }
    /// Get segment resource handler
    pub fn segment(&self) -> resources::Segment<'_> {
        resources::Segment::new(self.provider)
    }
    /// Get view resource handler
    pub fn view(&self) -> resources::View<'_> {
        resources::View::new(self.provider)
    }
    /// Get assessment_rule resource handler
    pub fn assessment_rule(&self) -> resources::Assessment_rule<'_> {
        resources::Assessment_rule::new(self.provider)
    }
    /// Get assessment resource handler
    pub fn assessment(&self) -> resources::Assessment<'_> {
        resources::Assessment::new(self.provider)
    }
    /// Get revision resource handler
    pub fn revision(&self) -> resources::Revision<'_> {
        resources::Revision::new(self.provider)
    }
    /// Get authorized_view_set resource handler
    pub fn authorized_view_set(&self) -> resources::Authorized_view_set<'_> {
        resources::Authorized_view_set::new(self.provider)
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
