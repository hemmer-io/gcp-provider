//! Security_health_analytics_setting resource
//!
//! Calculates the effective SecurityHealthAnalyticsSettings based on its level in the resource hierarchy and its settings. Settings provided closer to the target resource take precedence over those further away (e.g. folder will override organization level settings). The default SCC setting for the detector service defaults can be overridden at organization, folder and project levels. No assumptions should be made about the SCC defaults as it is considered an internal implementation detail.

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// Security_health_analytics_setting resource handler
pub struct Security_health_analytics_setting<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Security_health_analytics_setting<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }




    /// Read/describe a security_health_analytics_setting
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
    async fn test_security_health_analytics_setting_operations() {
        // Test security_health_analytics_setting CRUD operations
    }
}
