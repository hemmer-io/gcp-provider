//! External_system resource
//!
//! Auto-generated resource

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// External_system resource handler
pub struct External_system<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> External_system<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }






    /// Update a external_system
    ///
    /// TODO: Map `id` and update fields to appropriate SDK parameters
    #[allow(unused_variables)]
    pub async fn update(&self, id: &str, case_sla: Option<String>, name: Option<String>, external_system_update_time: Option<String>, case_close_time: Option<String>, case_priority: Option<String>, case_create_time: Option<String>, external_uid: Option<String>, status: Option<String>, ticket_info: Option<String>, case_uri: Option<String>, assignees: Option<Vec<String>>) -> Result<()> {

        todo!("Implement update for Gcp")

    }



}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_external_system_operations() {
        // Test external_system CRUD operations
    }
}
