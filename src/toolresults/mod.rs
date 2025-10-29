//! Toolresults Service
//!
//! Auto-generated service module for toolresults

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for toolresults
pub struct ToolresultsService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> ToolresultsService<'a> {
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
