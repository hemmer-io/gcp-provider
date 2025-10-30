//! Apiproduct resource
//!
//! Creates an API product in an organization. You create API products after you have proxied backend services using API proxies. An API product is a collection of API resources combined with quota settings and metadata that you can use to deliver customized and productized API bundles to your developer community. This metadata can include: - Scope - Environments - API proxies - Extensible profile API products enable you repackage APIs on the fly, without having to do any additional coding or configuration. Apigee recommends that you start with a simple API product including only required elements. You then provision credentials to apps to enable them to start testing your APIs. After you have authentication and authorization working against a simple API product, you can iterate to create finer-grained API products, defining different sets of API resources for each API product. **WARNING:** - If you don't specify an API proxy in the request body, *any* app associated with the product can make calls to *any* API in your entire organization. - If you don't specify an environment in the request body, the product allows access to all environments. For more information, see What is an API product?

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// Apiproduct resource handler
pub struct Apiproduct<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Apiproduct<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }


    /// Create a new apiproduct
    ///
    /// Note: Parameter types are simplified. SDK may require specific enums/types.
    /// TODO: Convert String parameters to appropriate SDK types as needed.
    #[allow(unused_variables)]
    pub async fn create(&self, display_name: Option<String>, graphql_operation_group: Option<String>, proxies: Option<Vec<String>>, quota_interval: Option<String>, operation_group: Option<String>, quota_time_unit: Option<String>, api_resources: Option<Vec<String>>, environments: Option<Vec<String>>, last_modified_at: Option<String>, approval_type: Option<String>, grpc_operation_group: Option<String>, description: Option<String>, space: Option<String>, name: Option<String>, quota_counter_scope: Option<String>, quota: Option<String>, scopes: Option<Vec<String>>, attributes: Option<Vec<String>>, created_at: Option<String>, parent: String) -> Result<String> {

        todo!("Implement create for Gcp")

    }



    /// Read/describe a apiproduct
    ///
    /// TODO: Map `id` parameter to appropriate SDK field(s)
    #[allow(unused_variables)]
    pub async fn read(&self, id: &str) -> Result<()> {

        todo!("Implement read for Gcp")

    }



    /// Update a apiproduct
    ///
    /// TODO: Map `id` and update fields to appropriate SDK parameters
    #[allow(unused_variables)]
    pub async fn update(&self, id: &str, display_name: Option<String>, graphql_operation_group: Option<String>, proxies: Option<Vec<String>>, quota_interval: Option<String>, operation_group: Option<String>, quota_time_unit: Option<String>, api_resources: Option<Vec<String>>, environments: Option<Vec<String>>, last_modified_at: Option<String>, approval_type: Option<String>, grpc_operation_group: Option<String>, description: Option<String>, space: Option<String>, name: Option<String>, quota_counter_scope: Option<String>, quota: Option<String>, scopes: Option<Vec<String>>, attributes: Option<Vec<String>>, created_at: Option<String>) -> Result<()> {

        todo!("Implement update for Gcp")

    }



    /// Delete a apiproduct
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
    async fn test_apiproduct_operations() {
        // Test apiproduct CRUD operations
    }
}
