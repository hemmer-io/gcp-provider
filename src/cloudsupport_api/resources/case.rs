//! Case resource
//!
//! Create a new case and associate it with a parent. It must have the following fields set: `display_name`, `description`, `classification`, and `priority`. If you're just testing the API and don't want to route your case to an agent, set `testCase=true`. EXAMPLES: cURL: ```shell parent="projects/some-project" curl \ --request POST \ --header "Authorization: Bearer $(gcloud auth print-access-token)" \ --header 'Content-Type: application/json' \ --data '{ "display_name": "Test case created by me.", "description": "a random test case, feel free to close", "classification": { "id": "100IK2AKCLHMGRJ9CDGMOCGP8DM6UTB4BT262T31BT1M2T31DHNMENPO6KS36CPJ786L2TBFEHGN6NPI64R3CDHN8880G08I1H3MURR7DHII0GRCDTQM8" }, "time_zone": "-07:00", "subscriber_email_addresses": [ "foo@domain.com", "bar@domain.com" ], "testCase": true, "priority": "P3" }' \ "https://cloudsupport.googleapis.com/v2/$parent/cases" ``` Python: ```python import googleapiclient.discovery api_version = "v2" supportApiService = googleapiclient.discovery.build( serviceName="cloudsupport", version=api_version, discoveryServiceUrl=f"https://cloudsupport.googleapis.com/$discovery/rest?version={api_version}", ) request = supportApiService.cases().create( parent="projects/some-project", body={ "displayName": "A Test Case", "description": "This is a test case.", "testCase": True, "priority": "P2", "classification": { "id": "100IK2AKCLHMGRJ9CDGMOCGP8DM6UTB4BT262T31BT1M2T31DHNMENPO6KS36CPJ786L2TBFEHGN6NPI64R3CDHN8880G08I1H3MURR7DHII0GRCDTQM8" }, }, ) print(request.execute()) ```

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// Case resource handler
pub struct Case<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Case<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }


    /// Create a new case
    ///
    /// Note: Parameter types are simplified. SDK may require specific enums/types.
    /// TODO: Convert String parameters to appropriate SDK types as needed.
    #[allow(unused_variables)]
    pub async fn create(&self, update_time: Option<String>, contact_email: Option<String>, display_name: Option<String>, state: Option<String>, time_zone: Option<String>, description: Option<String>, subscriber_email_addresses: Option<Vec<String>>, language_code: Option<String>, name: Option<String>, priority: Option<String>, create_time: Option<String>, creator: Option<String>, classification: Option<String>, test_case: Option<bool>, escalated: Option<bool>, parent: String) -> Result<String> {

        todo!("Implement create for Gcp")

    }



    /// Read/describe a case
    ///
    /// TODO: Map `id` parameter to appropriate SDK field(s)
    #[allow(unused_variables)]
    pub async fn read(&self, id: &str) -> Result<()> {

        todo!("Implement read for Gcp")

    }



    /// Update a case
    ///
    /// TODO: Map `id` and update fields to appropriate SDK parameters
    #[allow(unused_variables)]
    pub async fn update(&self, id: &str, update_time: Option<String>, contact_email: Option<String>, display_name: Option<String>, state: Option<String>, time_zone: Option<String>, description: Option<String>, subscriber_email_addresses: Option<Vec<String>>, language_code: Option<String>, name: Option<String>, priority: Option<String>, create_time: Option<String>, creator: Option<String>, classification: Option<String>, test_case: Option<bool>, escalated: Option<bool>) -> Result<()> {

        todo!("Implement update for Gcp")

    }



}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_case_operations() {
        // Test case CRUD operations
    }
}
