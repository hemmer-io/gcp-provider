//! Store_info resource
//!
//! List StoreInfos owned or managed by the partner.

See _Authentication and Authorization rules_ and
_List methods rules_ for more information about this method.

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// Store_info resource handler
pub struct Store_info<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Store_info<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }




    /// Read/describe a store_info
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
    async fn test_store_info_operations() {
        // Test store_info CRUD operations
    }
}
