//! Pagespeedapi resource
//!
//! Runs PageSpeed analysis on the page at the specified URL, and returns PageSpeed scores, a list of suggestions to make that page faster, and other information.

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// Pagespeedapi resource handler
pub struct Pagespeedapi<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Pagespeedapi<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }




    /// Read/describe a pagespeedapi
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
    async fn test_pagespeedapi_operations() {
        // Test pagespeedapi CRUD operations
    }
}
