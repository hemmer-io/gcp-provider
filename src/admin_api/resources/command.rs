//! Command resource
//!
//! Gets command data a specific command issued to the device.

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// Command resource handler
pub struct Command<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Command<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }




    /// Read/describe a command
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
    async fn test_command_operations() {
        // Test command CRUD operations
    }
}
