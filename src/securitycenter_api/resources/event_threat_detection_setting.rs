//! Event_threat_detection_setting resource
//!
//! Calculates the effective EventThreatDetectionSettings based on its level in the resource hierarchy and its settings. Settings provided closer to the target resource take precedence over those further away (e.g. folder will override organization level settings). The default SCC setting for the detector service defaults can be overridden at organization, folder and project levels. No assumptions should be made about the SCC defaults as it is considered an internal implementation detail.

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// Event_threat_detection_setting resource handler
pub struct Event_threat_detection_setting<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Event_threat_detection_setting<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }




    /// Read/describe a event_threat_detection_setting
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
    async fn test_event_threat_detection_setting_operations() {
        // Test event_threat_detection_setting CRUD operations
    }
}
