//! Photo resource
//!
//! After the client finishes uploading the photo with the returned UploadRef, CreatePhoto publishes the uploaded Photo to Street View on Google Maps. Currently, the only way to set heading, pitch, and roll in CreatePhoto is through the [Photo Sphere XMP metadata](https://developers.google.com/streetview/spherical-metadata) in the photo bytes. CreatePhoto ignores the `pose.heading`, `pose.pitch`, `pose.roll`, `pose.altitude`, and `pose.level` fields in Pose. This method returns the following error codes: * google.rpc.Code.INVALID_ARGUMENT if the request is malformed or if the uploaded photo is not a 360 photo. * google.rpc.Code.NOT_FOUND if the upload reference does not exist. * google.rpc.Code.RESOURCE_EXHAUSTED if the account has reached the storage limit.

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// Photo resource handler
pub struct Photo<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Photo<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }


    /// Create a new photo
    ///
    /// Note: Parameter types are simplified. SDK may require specific enums/types.
    /// TODO: Convert String parameters to appropriate SDK types as needed.
    #[allow(unused_variables)]
    pub async fn create(&self, connections: Option<Vec<String>>, places: Option<Vec<String>>, upload_reference: Option<String>, view_count: Option<String>, photo_id: Option<String>, capture_time: Option<String>, share_link: Option<String>, transfer_status: Option<String>, upload_time: Option<String>, maps_publish_status: Option<String>, pose: Option<String>, download_url: Option<String>, thumbnail_url: Option<String>) -> Result<String> {

        todo!("Implement create for Gcp")

    }



    /// Read/describe a photo
    ///
    /// TODO: Map `id` parameter to appropriate SDK field(s)
    #[allow(unused_variables)]
    pub async fn read(&self, id: &str) -> Result<()> {

        todo!("Implement read for Gcp")

    }



    /// Update a photo
    ///
    /// TODO: Map `id` and update fields to appropriate SDK parameters
    #[allow(unused_variables)]
    pub async fn update(&self, id: &str, connections: Option<Vec<String>>, places: Option<Vec<String>>, upload_reference: Option<String>, view_count: Option<String>, photo_id: Option<String>, capture_time: Option<String>, share_link: Option<String>, transfer_status: Option<String>, upload_time: Option<String>, maps_publish_status: Option<String>, pose: Option<String>, download_url: Option<String>, thumbnail_url: Option<String>) -> Result<()> {

        todo!("Implement update for Gcp")

    }



    /// Delete a photo
    ///
    /// TODO: Map `id` parameter to appropriate SDK field(s)
    #[allow(unused_variables)]
    pub async fn delete(&self, id: &str) -> Result<()> {

        todo!("Implement delete for Gcp")

    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_photo_operations() {
        // Test photo CRUD operations
    }
}
