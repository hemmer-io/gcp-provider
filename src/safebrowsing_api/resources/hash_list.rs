//! Hash_list resource
//!
//! Get the latest contents of a hash list. A hash list may either by a threat list or a non-threat list such as the Global Cache. This is a standard Get method as defined by https://google.aip.dev/131 and the HTTP method is also GET.

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// Hash_list resource handler
pub struct Hash_list<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Hash_list<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }




    /// Read/describe a hash_list
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
    async fn test_hash_list_operations() {
        // Test hash_list CRUD operations
    }
}
