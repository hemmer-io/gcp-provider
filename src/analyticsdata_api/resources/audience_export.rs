//! Audience_export resource
//!
//! Creates an audience export for later retrieval. This method quickly returns the audience export's resource name and initiates a long running asynchronous request to form an audience export. To export the users in an audience export, first create the audience export through this method and then send the audience resource name to the `QueryAudienceExport` method. See [Creating an Audience Export](https://developers.google.com/analytics/devguides/reporting/data/v1/audience-list-basics) for an introduction to Audience Exports with examples. An audience export is a snapshot of the users currently in the audience at the time of audience export creation. Creating audience exports for one audience on different days will return different results as users enter and exit the audience. Audiences in Google Analytics 4 allow you to segment your users in the ways that are important to your business. To learn more, see https://support.google.com/analytics/answer/9267572. Audience exports contain the users in each audience. Audience Export APIs have some methods at alpha and other methods at beta stability. The intention is to advance methods to beta stability after some feedback and adoption. To give your feedback on this API, complete the [Google Analytics Audience Export API Feedback](https://forms.gle/EeA5u5LW6PEggtCEA) form.

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// Audience_export resource handler
pub struct Audience_export<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Audience_export<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }


    /// Create a new audience_export
    ///
    /// Note: Parameter types are simplified. SDK may require specific enums/types.
    /// TODO: Convert String parameters to appropriate SDK types as needed.
    #[allow(unused_variables)]
    pub async fn create(&self, error_message: Option<String>, audience_display_name: Option<String>, creation_quota_tokens_charged: Option<i64>, dimensions: Option<Vec<String>>, audience: Option<String>, row_count: Option<i64>, begin_creating_time: Option<String>, percentage_completed: Option<f64>, name: Option<String>, state: Option<String>, parent: String) -> Result<String> {

        todo!("Implement create for Gcp")

    }



    /// Read/describe a audience_export
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
    async fn test_audience_export_operations() {
        // Test audience_export CRUD operations
    }
}
