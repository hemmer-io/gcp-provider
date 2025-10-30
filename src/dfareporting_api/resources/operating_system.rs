//! Operating_system resource
//!
//! Gets one operating system by DART ID.

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// Operating_system resource handler
pub struct Operating_system<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Operating_system<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }




    /// Read/describe a operating_system
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
    async fn test_operating_system_operations() {
        // Test operating_system CRUD operations
    }
}
