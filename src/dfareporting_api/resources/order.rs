//! Order resource
//!
//! Gets one order by ID.

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
