//! Frame resource
//!
//! RetrieveFrames returns instances associated with the given study, series, SOP Instance UID and frame numbers. See [RetrieveTransaction](https://dicom.nema.org/medical/dicom/current/output/html/part18.html#sect_10.4). For details on the implementation of RetrieveFrames, see [DICOM frames](https://cloud.google.com/healthcare/docs/dicom#dicom_frames) in the Cloud Healthcare API conformance statement. For samples that show how to call RetrieveFrames, see [Retrieve DICOM data](https://cloud.google.com/healthcare/docs/how-tos/dicomweb#retrieve-dicom).

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// Frame resource handler
pub struct Frame<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Frame<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }




    /// Read/describe a frame
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
    async fn test_frame_operations() {
        // Test frame CRUD operations
    }
}
