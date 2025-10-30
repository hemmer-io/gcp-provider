//! Avail resource
//!
//! Get an Avail given its avail group id and avail id.

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// Avail resource handler
pub struct Avail<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Avail<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }




    /// Read/describe a avail
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
    async fn test_avail_operations() {
        // Test avail CRUD operations
    }
}
