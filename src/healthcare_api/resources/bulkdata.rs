//! Bulkdata resource
//!
//! Returns uncompressed, unencoded bytes representing the referenced bulkdata tag from an instance. See [Retrieve Transaction](https://dicom.nema.org/medical/dicom/current/output/html/part18.html#sect_10.4). For details on the implementation of RetrieveBulkdata, see [Bulkdata resources](https://cloud.google.com/healthcare/docs/dicom#bulkdata-resources) in the Cloud Healthcare API conformance statement. For samples that show how to call RetrieveBulkdata, see [Retrieve bulkdata](https://cloud.google.com/healthcare/docs/how-tos/dicomweb#retrieve-bulkdata).

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// Bulkdata resource handler
pub struct Bulkdata<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Bulkdata<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }




    /// Read/describe a bulkdata
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
    async fn test_bulkdata_operations() {
        // Test bulkdata CRUD operations
    }
}
