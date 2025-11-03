//! Tabledata resource
//!
//! Streams data into BigQuery one record at a time without needing to run a load job.

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// Tabledata resource handler
pub struct Tabledata<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Tabledata<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }


    /// Create a new tabledata
    ///
    /// Note: Parameter types are simplified. SDK may require specific enums/types.
    /// TODO: Convert String parameters to appropriate SDK types as needed.
    #[allow(unused_variables)]
    pub async fn create(&self, template_suffix: Option<String>, ignore_unknown_values: Option<bool>, kind: Option<String>, skip_invalid_rows: Option<bool>, trace_id: Option<String>, rows: Option<Vec<String>>, project_id: String, table_id: String, dataset_id: String) -> Result<String> {

        todo!("Implement create for Gcp")

    }



    /// Read/describe a tabledata
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
    async fn test_tabledata_operations() {
        // Test tabledata CRUD operations
    }
}
