//! Lro resource
//!
//! Lists operations that match the specified filter in the request. If the server doesn't support this method, it returns `UNIMPLEMENTED`.

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// Lro resource handler
pub struct Lro<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Lro<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }




    /// Read/describe a lro
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
    async fn test_lro_operations() {
        // Test lro CRUD operations
    }
}
