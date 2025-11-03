//! Uri resource
//!
//! This method is used to check whether a URI is on a given threatList. Multiple threatLists may be searched in a single query. The response will list all requested threatLists the URI was found to match. If the URI is not found on any of the requested ThreatList an empty response will be returned.

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// Uri resource handler
pub struct Uri<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Uri<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }




    /// Read/describe a uri
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
    async fn test_uri_operations() {
        // Test uri CRUD operations
    }
}
