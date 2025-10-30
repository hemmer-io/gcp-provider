//! Run resource
//!
//! Returns information about the particular transfer run.

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// Run resource handler
pub struct Run<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Run<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }




    /// Read/describe a run
    ///
    /// TODO: Map `id` parameter to appropriate SDK field(s)
    #[allow(unused_variables)]
    pub async fn read(&self, id: &str) -> Result<()> {

        todo!("Implement read for Gcp")

    }





    /// Delete a run
    ///
    /// TODO: Map `id` parameter to appropriate SDK field(s)
    #[allow(unused_variables)]
    pub async fn delete(&self, id: &str) -> Result<()> {

        todo!("Implement delete for Gcp")

    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_run_operations() {
        // Test run CRUD operations
    }
}
