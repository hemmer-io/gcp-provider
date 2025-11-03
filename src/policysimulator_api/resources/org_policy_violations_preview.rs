//! Org_policy_violations_preview resource
//!
//! CreateOrgPolicyViolationsPreview creates an OrgPolicyViolationsPreview for the proposed changes in the provided OrgPolicyViolationsPreview.OrgPolicyOverlay. The changes to OrgPolicy are specified by this `OrgPolicyOverlay`. The resources to scan are inferred from these specified changes.

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// Org_policy_violations_preview resource handler
pub struct Org_policy_violations_preview<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Org_policy_violations_preview<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }


    /// Create a new org_policy_violations_preview
    ///
    /// Note: Parameter types are simplified. SDK may require specific enums/types.
    /// TODO: Convert String parameters to appropriate SDK types as needed.
    #[allow(unused_variables)]
    pub async fn create(&self, resource_counts: Option<String>, violations_count: Option<i64>, custom_constraints: Option<Vec<String>>, overlay: Option<String>, create_time: Option<String>, name: Option<String>, state: Option<String>, parent: String) -> Result<String> {

        todo!("Implement create for Gcp")

    }



    /// Read/describe a org_policy_violations_preview
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
    async fn test_org_policy_violations_preview_operations() {
        // Test org_policy_violations_preview CRUD operations
    }
}
