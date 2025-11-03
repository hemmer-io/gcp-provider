//! Orderinvoice resource
//!
//! Creates a refund invoice for one or more shipment groups, and triggers a refund for orderinvoice enabled orders. This can only be used for line items that have previously been charged using `createChargeInvoice`. All amounts (except for the summary) are incremental with respect to the previous invoice.

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// Orderinvoice resource handler
pub struct Orderinvoice<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Orderinvoice<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }


    /// Create a new orderinvoice
    ///
    /// Note: Parameter types are simplified. SDK may require specific enums/types.
    /// TODO: Convert String parameters to appropriate SDK types as needed.
    #[allow(unused_variables)]
    pub async fn create(&self, return_option: Option<String>, invoice_id: Option<String>, refund_only_option: Option<String>, operation_id: Option<String>, shipment_invoices: Option<Vec<String>>, order_id: String, merchant_id: String) -> Result<String> {

        todo!("Implement create for Gcp")

    }







}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_orderinvoice_operations() {
        // Test orderinvoice CRUD operations
    }
}
