//! Message resource
//!
//! Allows a user to load live chat through a server-streamed RPC.

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// Message resource handler
pub struct Message<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Message<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }




    /// Read/describe a message
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
    async fn test_message_operations() {
        // Test message CRUD operations
    }
}
