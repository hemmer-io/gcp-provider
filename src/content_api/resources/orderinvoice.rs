//! Orderinvoice resource
//!
//! Creates a charge invoice for a shipment group, and triggers a charge capture for orderinvoice enabled orders.

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
    pub async fn create(&self, operation_id: Option<String>, shipment_group_id: Option<String>, line_item_invoices: Option<Vec<String>>, invoice_id: Option<String>, invoice_summary: Option<String>, merchant_id: String, order_id: String) -> Result<String> {

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
