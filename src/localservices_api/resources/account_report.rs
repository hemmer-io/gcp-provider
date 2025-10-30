//! Account_report resource
//!
//! Get account reports containing aggregate account data of all linked GLS accounts. Caller needs to provide their manager customer id and the associated auth credential that allows them read permissions on their linked accounts.

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// Account_report resource handler
pub struct Account_report<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Account_report<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }




    /// Read/describe a account_report
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
    async fn test_account_report_operations() {
        // Test account_report CRUD operations
    }
}
