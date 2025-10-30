//! Report resource
//!
//! Generate report of the number of devices expiring in each month of the selected time frame. Devices are grouped by auto update expiration date and model. Further information can be found [here](https://support.google.com/chrome/a/answer/10564947).

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// Report resource handler
pub struct Report<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Report<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }




    /// Read/describe a report
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
    async fn test_report_operations() {
        // Test report CRUD operations
    }
}
