//! Callback resource
//!
//! Returns a list of active callbacks that belong to the execution with the given name. The returned callbacks are ordered by callback ID.

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// Callback resource handler
pub struct Callback<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Callback<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }




    /// Read/describe a callback
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
    async fn test_callback_operations() {
        // Test callback CRUD operations
    }
}
