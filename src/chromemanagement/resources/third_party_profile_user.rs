//! Third_party_profile_user resource
//!
//! Moves a third party chrome profile user to a destination OU. All profiles associated to that user will be moved to the destination OU.

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// Third_party_profile_user resource handler
pub struct Third_party_profile_user<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Third_party_profile_user<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }


    /// Create a new third_party_profile_user
    ///
    /// Note: Parameter types are simplified. SDK may require specific enums/types.
    /// TODO: Convert String parameters to appropriate SDK types as needed.
    #[allow(unused_variables)]
    pub async fn create(&self, destination_org_unit: Option<String>, name: String) -> Result<String> {

        todo!("Implement create for Gcp")

    }







}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_third_party_profile_user_operations() {
        // Test third_party_profile_user CRUD operations
    }
}
