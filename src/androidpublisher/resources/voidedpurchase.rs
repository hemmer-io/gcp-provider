//! Voidedpurchase resource
//!
//! Lists the purchases that were canceled, refunded or charged-back.

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// Voidedpurchase resource handler
pub struct Voidedpurchase<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Voidedpurchase<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }




    /// Read/describe a voidedpurchase
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
    async fn test_voidedpurchase_operations() {
        // Test voidedpurchase CRUD operations
    }
}
