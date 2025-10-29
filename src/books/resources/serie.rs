//! Serie resource
//!
//! Returns Series metadata for the given series ids.

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// Serie resource handler
pub struct Serie<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Serie<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }




    /// Read/describe a serie
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
    async fn test_serie_operations() {
        // Test serie CRUD operations
    }
}
