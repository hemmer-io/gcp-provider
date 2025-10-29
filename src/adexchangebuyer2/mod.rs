//! Adexchangebuyer2 Service
//!
//! Auto-generated service module for adexchangebuyer2

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for adexchangebuyer2
pub struct Adexchangebuyer2Service<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Adexchangebuyer2Service<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get proposal resource handler
    pub fn proposal(&self) -> resources::Proposal<'_> {
        resources::Proposal::new(self.provider)
    }
    /// Get bid_metric resource handler
    pub fn bid_metric(&self) -> resources::Bid_metric<'_> {
        resources::Bid_metric::new(self.provider)
    }
    /// Get bid_responses_without_bid resource handler
    pub fn bid_responses_without_bid(&self) -> resources::Bid_responses_without_bid<'_> {
        resources::Bid_responses_without_bid::new(self.provider)
    }
    /// Get non_billable_winning_bid resource handler
    pub fn non_billable_winning_bid(&self) -> resources::Non_billable_winning_bid<'_> {
        resources::Non_billable_winning_bid::new(self.provider)
    }
    /// Get bid_response_error resource handler
    pub fn bid_response_error(&self) -> resources::Bid_response_error<'_> {
        resources::Bid_response_error::new(self.provider)
    }
    /// Get product resource handler
    pub fn product(&self) -> resources::Product<'_> {
        resources::Product::new(self.provider)
    }
    /// Get publisher_profile resource handler
    pub fn publisher_profile(&self) -> resources::Publisher_profile<'_> {
        resources::Publisher_profile::new(self.provider)
    }
    /// Get creative resource handler
    pub fn creative(&self) -> resources::Creative<'_> {
        resources::Creative::new(self.provider)
    }
    /// Get impression_metric resource handler
    pub fn impression_metric(&self) -> resources::Impression_metric<'_> {
        resources::Impression_metric::new(self.provider)
    }
    /// Get filter_set resource handler
    pub fn filter_set(&self) -> resources::Filter_set<'_> {
        resources::Filter_set::new(self.provider)
    }
    /// Get client resource handler
    pub fn client(&self) -> resources::Client<'_> {
        resources::Client::new(self.provider)
    }
    /// Get user resource handler
    pub fn user(&self) -> resources::User<'_> {
        resources::User::new(self.provider)
    }
    /// Get losing_bid resource handler
    pub fn losing_bid(&self) -> resources::Losing_bid<'_> {
        resources::Losing_bid::new(self.provider)
    }
    /// Get deal_association resource handler
    pub fn deal_association(&self) -> resources::Deal_association<'_> {
        resources::Deal_association::new(self.provider)
    }
    /// Get finalized_proposal resource handler
    pub fn finalized_proposal(&self) -> resources::Finalized_proposal<'_> {
        resources::Finalized_proposal::new(self.provider)
    }
    /// Get invitation resource handler
    pub fn invitation(&self) -> resources::Invitation<'_> {
        resources::Invitation::new(self.provider)
    }
    /// Get filtered_bid resource handler
    pub fn filtered_bid(&self) -> resources::Filtered_bid<'_> {
        resources::Filtered_bid::new(self.provider)
    }
    /// Get detail resource handler
    pub fn detail(&self) -> resources::Detail<'_> {
        resources::Detail::new(self.provider)
    }
    /// Get filtered_bid_request resource handler
    pub fn filtered_bid_request(&self) -> resources::Filtered_bid_request<'_> {
        resources::Filtered_bid_request::new(self.provider)
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
