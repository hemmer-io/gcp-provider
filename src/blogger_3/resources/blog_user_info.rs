//! Blog_user_info resource
//!
//! Gets one blog and user info pair by blogId and userId.

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// Blog_user_info resource handler
pub struct Blog_user_info<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Blog_user_info<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }




    /// Read/describe a blog_user_info
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
    async fn test_blog_user_info_operations() {
        // Test blog_user_info CRUD operations
    }
}
