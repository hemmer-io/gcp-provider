//! Blogger Service
//!
//! Auto-generated service module for blogger

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for blogger
pub struct BloggerService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> BloggerService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get comment resource handler
    pub fn comment(&self) -> resources::Comment<'_> {
        resources::Comment::new(self.provider)
    }
    /// Get post resource handler
    pub fn post(&self) -> resources::Post<'_> {
        resources::Post::new(self.provider)
    }
    /// Get page_view resource handler
    pub fn page_view(&self) -> resources::Page_view<'_> {
        resources::Page_view::new(self.provider)
    }
    /// Get blog resource handler
    pub fn blog(&self) -> resources::Blog<'_> {
        resources::Blog::new(self.provider)
    }
    /// Get blog_user_info resource handler
    pub fn blog_user_info(&self) -> resources::Blog_user_info<'_> {
        resources::Blog_user_info::new(self.provider)
    }
    /// Get user resource handler
    pub fn user(&self) -> resources::User<'_> {
        resources::User::new(self.provider)
    }
    /// Get page resource handler
    pub fn page(&self) -> resources::Page<'_> {
        resources::Page::new(self.provider)
    }
    /// Get post_user_info resource handler
    pub fn post_user_info(&self) -> resources::Post_user_info<'_> {
        resources::Post_user_info::new(self.provider)
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
