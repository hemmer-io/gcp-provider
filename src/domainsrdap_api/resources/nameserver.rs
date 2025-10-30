//! Nameserver resource
//!
//! The RDAP API recognizes this command from the RDAP specification but does not support it. The response is a formatted 501 error.

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// Nameserver resource handler
pub struct Nameserver<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Nameserver<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }




    /// Read/describe a nameserver
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
    async fn test_nameserver_operations() {
        // Test nameserver CRUD operations
    }
}
