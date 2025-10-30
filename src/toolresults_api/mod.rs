//! Toolresults_api Service
//!
//! Auto-generated service module for toolresults_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for toolresults_api
pub struct Toolresults_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Toolresults_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }


}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_service_creation() {
        // Service creation test
    }
}
