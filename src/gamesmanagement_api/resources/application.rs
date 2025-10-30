//! Application resource
//!
//! Get the list of players hidden from the given application. This method is only available to user accounts for your developer console.

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// Application resource handler
pub struct Application<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Application<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }




    /// Read/describe a application
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
    async fn test_application_operations() {
        // Test application CRUD operations
    }
}
