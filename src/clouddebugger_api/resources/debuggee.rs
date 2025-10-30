//! Debuggee resource
//!
//! Registers the debuggee with the controller service. All agents attached to the same application must call this method with exactly the same request content to get back the same stable `debuggee_id`. Agents should call this method again whenever `google.rpc.Code.NOT_FOUND` is returned from any controller method. This protocol allows the controller service to disable debuggees, recover from data loss, or change the `debuggee_id` format. Agents must handle `debuggee_id` value changing upon re-registration.

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// Debuggee resource handler
pub struct Debuggee<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Debuggee<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }


    /// Create a new debuggee
    ///
    /// Note: Parameter types are simplified. SDK may require specific enums/types.
    /// TODO: Convert String parameters to appropriate SDK types as needed.
    #[allow(unused_variables)]
    pub async fn create(&self, debuggee: Option<String>) -> Result<String> {

        todo!("Implement create for Gcp")

    }



    /// Read/describe a debuggee
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
    async fn test_debuggee_operations() {
        // Test debuggee CRUD operations
    }
}
