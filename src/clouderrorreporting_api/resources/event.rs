//! Event resource
//!
//! Report an individual error event and record the event to a log. This endpoint accepts **either** an OAuth token, **or** an [API key](https://support.google.com/cloud/answer/6158862) for authentication. To use an API key, append it to the URL as the value of a `key` parameter. For example: `POST https://clouderrorreporting.googleapis.com/v1beta1/{projectName}/events:report?key=123ABC456` **Note:** [Error Reporting] (https://cloud.google.com/error-reporting) is a service built on Cloud Logging and can analyze log entries when all of the following are true: * Customer-managed encryption keys (CMEK) are disabled on the log bucket. * The log bucket satisfies one of the following: * The log bucket is stored in the same project where the logs originated. * The logs were routed to a project, and then that project stored those logs in a log bucket that it owns.

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// Event resource handler
pub struct Event<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Event<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }


    /// Create a new event
    ///
    /// Note: Parameter types are simplified. SDK may require specific enums/types.
    /// TODO: Convert String parameters to appropriate SDK types as needed.
    #[allow(unused_variables)]
    pub async fn create(&self, event_time: Option<String>, context: Option<String>, message: Option<String>, service_context: Option<String>, project_name: String) -> Result<String> {

        todo!("Implement create for Gcp")

    }



    /// Read/describe a event
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
    async fn test_event_operations() {
        // Test event CRUD operations
    }
}
