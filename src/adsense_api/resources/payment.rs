//! Payment resource
//!
//! List the payments for the specified AdSense account.

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// Payment resource handler
pub struct Payment<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Payment<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }




    /// Read/describe a payment
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
    async fn test_payment_operations() {
        // Test payment CRUD operations
    }
}
