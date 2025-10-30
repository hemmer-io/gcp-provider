//! Order resource
//!
//! Sets (or overrides if it already exists) merchant provided annotations in the form of key-value pairs. A common use case would be to supply us with additional structured information about a line item that cannot be provided via other methods. Submitted key-value pairs can be retrieved as part of the orders resource.

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// Order resource handler
pub struct Order<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Order<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }


    /// Create a new order
    ///
    /// Note: Parameter types are simplified. SDK may require specific enums/types.
    /// TODO: Convert String parameters to appropriate SDK types as needed.
    #[allow(unused_variables)]
    pub async fn create(&self, annotations: Option<Vec<String>>, operation_id: Option<String>, product_id: Option<String>, line_item_id: Option<String>, merchant_id: String, order_id: String) -> Result<String> {

        todo!("Implement create for Gcp")

    }



    /// Read/describe a order
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
    async fn test_order_operations() {
        // Test order CRUD operations
    }
}
