//! Orderpayment resource
//!
//! Notify about failure to authorize user's payment method.

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// Orderpayment resource handler
pub struct Orderpayment<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Orderpayment<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }


    /// Create a new orderpayment
    ///
    /// Note: Parameter types are simplified. SDK may require specific enums/types.
    /// TODO: Convert String parameters to appropriate SDK types as needed.
    #[allow(unused_variables)]
    pub async fn create(&self, decline_reason: Option<String>, order_id: String, merchant_id: String) -> Result<String> {

        todo!("Implement create for Gcp")

    }







}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_orderpayment_operations() {
        // Test orderpayment CRUD operations
    }
}
