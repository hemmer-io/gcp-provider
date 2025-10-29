//! Paymentsresellersubscription Service
//!
//! Auto-generated service module for paymentsresellersubscription

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for paymentsresellersubscription
pub struct PaymentsresellersubscriptionService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> PaymentsresellersubscriptionService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get product resource handler
    pub fn product(&self) -> resources::Product<'_> {
        resources::Product::new(self.provider)
    }
    /// Get subscription resource handler
    pub fn subscription(&self) -> resources::Subscription<'_> {
        resources::Subscription::new(self.provider)
    }
    /// Get line_item resource handler
    pub fn line_item(&self) -> resources::Line_item<'_> {
        resources::Line_item::new(self.provider)
    }
    /// Get promotion resource handler
    pub fn promotion(&self) -> resources::Promotion<'_> {
        resources::Promotion::new(self.provider)
    }
    /// Get user_session resource handler
    pub fn user_session(&self) -> resources::User_session<'_> {
        resources::User_session::new(self.provider)
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
