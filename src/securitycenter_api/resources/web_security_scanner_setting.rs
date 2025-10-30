//! Web_security_scanner_setting resource
//!
//! Calculates the effective WebSecurityScannerSettings based on its level in the resource hierarchy and its settings. Settings provided closer to the target resource take precedence over those further away (e.g. folder will override organization level settings). The default SCC setting for the detector service defaults can be overridden at organization, folder and project levels. No assumptions should be made about the SCC defaults as it is considered an internal implementation detail.

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// Web_security_scanner_setting resource handler
pub struct Web_security_scanner_setting<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Web_security_scanner_setting<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }




    /// Read/describe a web_security_scanner_setting
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
    async fn test_web_security_scanner_setting_operations() {
        // Test web_security_scanner_setting CRUD operations
    }
}
