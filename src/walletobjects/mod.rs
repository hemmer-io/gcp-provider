//! Walletobjects Service
//!
//! Auto-generated service module for walletobjects

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for walletobjects
pub struct WalletobjectsService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> WalletobjectsService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get transitobject resource handler
    pub fn transitobject(&self) -> resources::Transitobject<'_> {
        resources::Transitobject::new(self.provider)
    }
    /// Get offerobject resource handler
    pub fn offerobject(&self) -> resources::Offerobject<'_> {
        resources::Offerobject::new(self.provider)
    }
    /// Get eventticketclas resource handler
    pub fn eventticketclas(&self) -> resources::Eventticketclas<'_> {
        resources::Eventticketclas::new(self.provider)
    }
    /// Get loyaltyobject resource handler
    pub fn loyaltyobject(&self) -> resources::Loyaltyobject<'_> {
        resources::Loyaltyobject::new(self.provider)
    }
    /// Get jwt resource handler
    pub fn jwt(&self) -> resources::Jwt<'_> {
        resources::Jwt::new(self.provider)
    }
    /// Get transitclas resource handler
    pub fn transitclas(&self) -> resources::Transitclas<'_> {
        resources::Transitclas::new(self.provider)
    }
    /// Get media resource handler
    pub fn media(&self) -> resources::Media<'_> {
        resources::Media::new(self.provider)
    }
    /// Get flightobject resource handler
    pub fn flightobject(&self) -> resources::Flightobject<'_> {
        resources::Flightobject::new(self.provider)
    }
    /// Get giftcardobject resource handler
    pub fn giftcardobject(&self) -> resources::Giftcardobject<'_> {
        resources::Giftcardobject::new(self.provider)
    }
    /// Get loyaltyclas resource handler
    pub fn loyaltyclas(&self) -> resources::Loyaltyclas<'_> {
        resources::Loyaltyclas::new(self.provider)
    }
    /// Get offerclas resource handler
    pub fn offerclas(&self) -> resources::Offerclas<'_> {
        resources::Offerclas::new(self.provider)
    }
    /// Get eventticketobject resource handler
    pub fn eventticketobject(&self) -> resources::Eventticketobject<'_> {
        resources::Eventticketobject::new(self.provider)
    }
    /// Get giftcardclas resource handler
    pub fn giftcardclas(&self) -> resources::Giftcardclas<'_> {
        resources::Giftcardclas::new(self.provider)
    }
    /// Get permission resource handler
    pub fn permission(&self) -> resources::Permission<'_> {
        resources::Permission::new(self.provider)
    }
    /// Get private_content resource handler
    pub fn private_content(&self) -> resources::Private_content<'_> {
        resources::Private_content::new(self.provider)
    }
    /// Get genericclas resource handler
    pub fn genericclas(&self) -> resources::Genericclas<'_> {
        resources::Genericclas::new(self.provider)
    }
    /// Get flightclas resource handler
    pub fn flightclas(&self) -> resources::Flightclas<'_> {
        resources::Flightclas::new(self.provider)
    }
    /// Get smarttap resource handler
    pub fn smarttap(&self) -> resources::Smarttap<'_> {
        resources::Smarttap::new(self.provider)
    }
    /// Get genericobject resource handler
    pub fn genericobject(&self) -> resources::Genericobject<'_> {
        resources::Genericobject::new(self.provider)
    }
    /// Get issuer resource handler
    pub fn issuer(&self) -> resources::Issuer<'_> {
        resources::Issuer::new(self.provider)
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
