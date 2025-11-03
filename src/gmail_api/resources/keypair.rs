//! Keypair resource
//!
//! Creates and uploads a client-side encryption S/MIME public key certificate chain and private key metadata for the authenticated user. For administrators managing identities and keypairs for users in their organization, requests require authorization with a [service account](https://developers.google.com/identity/protocols/OAuth2ServiceAccount) that has [domain-wide delegation authority](https://developers.google.com/identity/protocols/OAuth2ServiceAccount#delegatingauthority) to impersonate users with the `https://www.googleapis.com/auth/gmail.settings.basic` scope. For users managing their own identities and keypairs, requests require [hardware key encryption](https://support.google.com/a/answer/14153163) turned on and configured.

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// Keypair resource handler
pub struct Keypair<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Keypair<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }


    /// Create a new keypair
    ///
    /// Note: Parameter types are simplified. SDK may require specific enums/types.
    /// TODO: Convert String parameters to appropriate SDK types as needed.
    #[allow(unused_variables)]
    pub async fn create(&self, subject_email_addresses: Option<Vec<String>>, disable_time: Option<String>, pem: Option<String>, key_pair_id: Option<String>, enablement_state: Option<String>, pkcs7: Option<String>, private_key_metadata: Option<Vec<String>>, user_id: String) -> Result<String> {

        todo!("Implement create for Gcp")

    }



    /// Read/describe a keypair
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
    async fn test_keypair_operations() {
        // Test keypair CRUD operations
    }
}
