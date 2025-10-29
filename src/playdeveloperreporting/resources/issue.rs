//! Issue resource
//!
//! Searches all error issues in which reports have been grouped.

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// Issue resource handler
pub struct Issue<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Issue<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }




    /// Read/describe a issue
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
    async fn test_issue_operations() {
        // Test issue CRUD operations
    }
}
