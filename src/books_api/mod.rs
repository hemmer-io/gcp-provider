//! Books_api Service
//!
//! Auto-generated service module for books_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for books_api
pub struct Books_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Books_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get volume resource handler
    pub fn volume(&self) -> resources::Volume<'_> {
        resources::Volume::new(self.provider)
    }
    /// Get serie resource handler
    pub fn serie(&self) -> resources::Serie<'_> {
        resources::Serie::new(self.provider)
    }
    /// Get promooffer resource handler
    pub fn promooffer(&self) -> resources::Promooffer<'_> {
        resources::Promooffer::new(self.provider)
    }
    /// Get readingposition resource handler
    pub fn readingposition(&self) -> resources::Readingposition<'_> {
        resources::Readingposition::new(self.provider)
    }
    /// Get personalizedstream resource handler
    pub fn personalizedstream(&self) -> resources::Personalizedstream<'_> {
        resources::Personalizedstream::new(self.provider)
    }
    /// Get bookshelve resource handler
    pub fn bookshelve(&self) -> resources::Bookshelve<'_> {
        resources::Bookshelve::new(self.provider)
    }
    /// Get mybook resource handler
    pub fn mybook(&self) -> resources::Mybook<'_> {
        resources::Mybook::new(self.provider)
    }
    /// Get annotation resource handler
    pub fn annotation(&self) -> resources::Annotation<'_> {
        resources::Annotation::new(self.provider)
    }
    /// Get associated resource handler
    pub fn associated(&self) -> resources::Associated<'_> {
        resources::Associated::new(self.provider)
    }
    /// Get annotation_data resource handler
    pub fn annotation_data(&self) -> resources::Annotation_data<'_> {
        resources::Annotation_data::new(self.provider)
    }
    /// Get myconfig resource handler
    pub fn myconfig(&self) -> resources::Myconfig<'_> {
        resources::Myconfig::new(self.provider)
    }
    /// Get volume_annotation resource handler
    pub fn volume_annotation(&self) -> resources::Volume_annotation<'_> {
        resources::Volume_annotation::new(self.provider)
    }
    /// Get onboarding resource handler
    pub fn onboarding(&self) -> resources::Onboarding<'_> {
        resources::Onboarding::new(self.provider)
    }
    /// Get dictionary resource handler
    pub fn dictionary(&self) -> resources::Dictionary<'_> {
        resources::Dictionary::new(self.provider)
    }
    /// Get layer resource handler
    pub fn layer(&self) -> resources::Layer<'_> {
        resources::Layer::new(self.provider)
    }
    /// Get membership resource handler
    pub fn membership(&self) -> resources::Membership<'_> {
        resources::Membership::new(self.provider)
    }
    /// Get notification resource handler
    pub fn notification(&self) -> resources::Notification<'_> {
        resources::Notification::new(self.provider)
    }
    /// Get useruploaded resource handler
    pub fn useruploaded(&self) -> resources::Useruploaded<'_> {
        resources::Useruploaded::new(self.provider)
    }
    /// Get recommended resource handler
    pub fn recommended(&self) -> resources::Recommended<'_> {
        resources::Recommended::new(self.provider)
    }
    /// Get cloudloading resource handler
    pub fn cloudloading(&self) -> resources::Cloudloading<'_> {
        resources::Cloudloading::new(self.provider)
    }
    /// Get familysharing resource handler
    pub fn familysharing(&self) -> resources::Familysharing<'_> {
        resources::Familysharing::new(self.provider)
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
