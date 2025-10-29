//! Androidpublisher Service
//!
//! Auto-generated service module for androidpublisher

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for androidpublisher
pub struct AndroidpublisherService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> AndroidpublisherService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get purchase_option resource handler
    pub fn purchase_option(&self) -> resources::Purchase_option<'_> {
        resources::Purchase_option::new(self.provider)
    }
    /// Get application resource handler
    pub fn application(&self) -> resources::Application<'_> {
        resources::Application::new(self.provider)
    }
    /// Get review resource handler
    pub fn review(&self) -> resources::Review<'_> {
        resources::Review::new(self.provider)
    }
    /// Get device_tier_config resource handler
    pub fn device_tier_config(&self) -> resources::Device_tier_config<'_> {
        resources::Device_tier_config::new(self.provider)
    }
    /// Get apk resource handler
    pub fn apk(&self) -> resources::Apk<'_> {
        resources::Apk::new(self.provider)
    }
    /// Get generatedapk resource handler
    pub fn generatedapk(&self) -> resources::Generatedapk<'_> {
        resources::Generatedapk::new(self.provider)
    }
    /// Get internalappsharingartifact resource handler
    pub fn internalappsharingartifact(&self) -> resources::Internalappsharingartifact<'_> {
        resources::Internalappsharingartifact::new(self.provider)
    }
    /// Get voidedpurchase resource handler
    pub fn voidedpurchase(&self) -> resources::Voidedpurchase<'_> {
        resources::Voidedpurchase::new(self.provider)
    }
    /// Get listing resource handler
    pub fn listing(&self) -> resources::Listing<'_> {
        resources::Listing::new(self.provider)
    }
    /// Get grant resource handler
    pub fn grant(&self) -> resources::Grant<'_> {
        resources::Grant::new(self.provider)
    }
    /// Get monetization resource handler
    pub fn monetization(&self) -> resources::Monetization<'_> {
        resources::Monetization::new(self.provider)
    }
    /// Get order resource handler
    pub fn order(&self) -> resources::Order<'_> {
        resources::Order::new(self.provider)
    }
    /// Get user resource handler
    pub fn user(&self) -> resources::User<'_> {
        resources::User::new(self.provider)
    }
    /// Get variant resource handler
    pub fn variant(&self) -> resources::Variant<'_> {
        resources::Variant::new(self.provider)
    }
    /// Get base_plan resource handler
    pub fn base_plan(&self) -> resources::Base_plan<'_> {
        resources::Base_plan::new(self.provider)
    }
    /// Get edit resource handler
    pub fn edit(&self) -> resources::Edit<'_> {
        resources::Edit::new(self.provider)
    }
    /// Get offer resource handler
    pub fn offer(&self) -> resources::Offer<'_> {
        resources::Offer::new(self.provider)
    }
    /// Get subscriptionsv2 resource handler
    pub fn subscriptionsv2(&self) -> resources::Subscriptionsv2<'_> {
        resources::Subscriptionsv2::new(self.provider)
    }
    /// Get expansionfile resource handler
    pub fn expansionfile(&self) -> resources::Expansionfile<'_> {
        resources::Expansionfile::new(self.provider)
    }
    /// Get bundle resource handler
    pub fn bundle(&self) -> resources::Bundle<'_> {
        resources::Bundle::new(self.provider)
    }
    /// Get track resource handler
    pub fn track(&self) -> resources::Track<'_> {
        resources::Track::new(self.provider)
    }
    /// Get onetimeproduct resource handler
    pub fn onetimeproduct(&self) -> resources::Onetimeproduct<'_> {
        resources::Onetimeproduct::new(self.provider)
    }
    /// Get externaltransaction resource handler
    pub fn externaltransaction(&self) -> resources::Externaltransaction<'_> {
        resources::Externaltransaction::new(self.provider)
    }
    /// Get countryavailability resource handler
    pub fn countryavailability(&self) -> resources::Countryavailability<'_> {
        resources::Countryavailability::new(self.provider)
    }
    /// Get productsv2 resource handler
    pub fn productsv2(&self) -> resources::Productsv2<'_> {
        resources::Productsv2::new(self.provider)
    }
    /// Get image resource handler
    pub fn image(&self) -> resources::Image<'_> {
        resources::Image::new(self.provider)
    }
    /// Get apprecovery resource handler
    pub fn apprecovery(&self) -> resources::Apprecovery<'_> {
        resources::Apprecovery::new(self.provider)
    }
    /// Get deobfuscationfile resource handler
    pub fn deobfuscationfile(&self) -> resources::Deobfuscationfile<'_> {
        resources::Deobfuscationfile::new(self.provider)
    }
    /// Get product resource handler
    pub fn product(&self) -> resources::Product<'_> {
        resources::Product::new(self.provider)
    }
    /// Get tester resource handler
    pub fn tester(&self) -> resources::Tester<'_> {
        resources::Tester::new(self.provider)
    }
    /// Get detail resource handler
    pub fn detail(&self) -> resources::Detail<'_> {
        resources::Detail::new(self.provider)
    }
    /// Get inappproduct resource handler
    pub fn inappproduct(&self) -> resources::Inappproduct<'_> {
        resources::Inappproduct::new(self.provider)
    }
    /// Get subscription resource handler
    pub fn subscription(&self) -> resources::Subscription<'_> {
        resources::Subscription::new(self.provider)
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
