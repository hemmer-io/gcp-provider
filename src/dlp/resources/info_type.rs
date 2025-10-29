//! Info_type resource
//!
//! Returns a list of the sensitive information types that the DLP API supports. See https://cloud.google.com/sensitive-data-protection/docs/infotypes-reference to learn more.

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// Info_type resource handler
pub struct Info_type<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Info_type<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }




    /// Read/describe a info_type
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
    async fn test_info_type_operations() {
        // Test info_type CRUD operations
    }
}
