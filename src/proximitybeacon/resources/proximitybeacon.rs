//! Proximitybeacon resource
//!
//! Gets the Proximity Beacon API's current public key and associated
parameters used to initiate the Diffie-Hellman key exchange required to
register a beacon that broadcasts the Eddystone-EID format. This key
changes periodically; clients may cache it and re-use the same public key
to provision and register multiple beacons. However, clients should be
prepared to refresh this key when they encounter an error registering an
Eddystone-EID beacon.

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// Proximitybeacon resource handler
pub struct Proximitybeacon<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Proximitybeacon<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }




    /// Read/describe a proximitybeacon
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
    async fn test_proximitybeacon_operations() {
        // Test proximitybeacon CRUD operations
    }
}
