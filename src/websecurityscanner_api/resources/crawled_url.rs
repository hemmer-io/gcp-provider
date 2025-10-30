//! Crawled_url resource
//!
//! List CrawledUrls under a given ScanRun.

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// Crawled_url resource handler
pub struct Crawled_url<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Crawled_url<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }




    /// Read/describe a crawled_url
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
    async fn test_crawled_url_operations() {
        // Test crawled_url CRUD operations
    }
}
