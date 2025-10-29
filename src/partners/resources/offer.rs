//! Offer resource
//!
//! Lists the Offers available for the current user

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// Offer resource handler
pub struct Offer<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Offer<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }




    /// Read/describe a offer
    ///
    /// TODO: Map `id` parameter to appropriate SDK field(s)
    #[allow(unused_variables)]
    pub async fn read(&self, id: &str) -> Result<()> {

        todo!("Implement read for Gcp")

    }





}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_offer_operations() {
        // Test offer CRUD operations
    }
}
