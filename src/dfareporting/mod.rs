//! Dfareporting Service
//!
//! Auto-generated service module for dfareporting

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for dfareporting
pub struct DfareportingService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> DfareportingService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get metro resource handler
    pub fn metro(&self) -> resources::Metro<'_> {
        resources::Metro::new(self.provider)
    }
    /// Get mobile_app resource handler
    pub fn mobile_app(&self) -> resources::Mobile_app<'_> {
        resources::Mobile_app::new(self.provider)
    }
    /// Get floodlight_activitie resource handler
    pub fn floodlight_activitie(&self) -> resources::Floodlight_activitie<'_> {
        resources::Floodlight_activitie::new(self.provider)
    }
    /// Get dimension_value resource handler
    pub fn dimension_value(&self) -> resources::Dimension_value<'_> {
        resources::Dimension_value::new(self.provider)
    }
    /// Get campaign resource handler
    pub fn campaign(&self) -> resources::Campaign<'_> {
        resources::Campaign::new(self.provider)
    }
    /// Get creative_field_value resource handler
    pub fn creative_field_value(&self) -> resources::Creative_field_value<'_> {
        resources::Creative_field_value::new(self.provider)
    }
    /// Get event_tag resource handler
    pub fn event_tag(&self) -> resources::Event_tag<'_> {
        resources::Event_tag::new(self.provider)
    }
    /// Get order resource handler
    pub fn order(&self) -> resources::Order<'_> {
        resources::Order::new(self.provider)
    }
    /// Get remarketing_list_share resource handler
    pub fn remarketing_list_share(&self) -> resources::Remarketing_list_share<'_> {
        resources::Remarketing_list_share::new(self.provider)
    }
    /// Get account_permission resource handler
    pub fn account_permission(&self) -> resources::Account_permission<'_> {
        resources::Account_permission::new(self.provider)
    }
    /// Get placement_strategie resource handler
    pub fn placement_strategie(&self) -> resources::Placement_strategie<'_> {
        resources::Placement_strategie::new(self.provider)
    }
    /// Get account_user_profile resource handler
    pub fn account_user_profile(&self) -> resources::Account_user_profile<'_> {
        resources::Account_user_profile::new(self.provider)
    }
    /// Get site resource handler
    pub fn site(&self) -> resources::Site<'_> {
        resources::Site::new(self.provider)
    }
    /// Get browser resource handler
    pub fn browser(&self) -> resources::Browser<'_> {
        resources::Browser::new(self.provider)
    }
    /// Get project resource handler
    pub fn project(&self) -> resources::Project<'_> {
        resources::Project::new(self.provider)
    }
    /// Get postal_code resource handler
    pub fn postal_code(&self) -> resources::Postal_code<'_> {
        resources::Postal_code::new(self.provider)
    }
    /// Get size resource handler
    pub fn size(&self) -> resources::Size<'_> {
        resources::Size::new(self.provider)
    }
    /// Get operating_system resource handler
    pub fn operating_system(&self) -> resources::Operating_system<'_> {
        resources::Operating_system::new(self.provider)
    }
    /// Get targeting_template resource handler
    pub fn targeting_template(&self) -> resources::Targeting_template<'_> {
        resources::Targeting_template::new(self.provider)
    }
    /// Get user_role_permission resource handler
    pub fn user_role_permission(&self) -> resources::User_role_permission<'_> {
        resources::User_role_permission::new(self.provider)
    }
    /// Get conversion resource handler
    pub fn conversion(&self) -> resources::Conversion<'_> {
        resources::Conversion::new(self.provider)
    }
    /// Get advertiser resource handler
    pub fn advertiser(&self) -> resources::Advertiser<'_> {
        resources::Advertiser::new(self.provider)
    }
    /// Get user_role_permission_group resource handler
    pub fn user_role_permission_group(&self) -> resources::User_role_permission_group<'_> {
        resources::User_role_permission_group::new(self.provider)
    }
    /// Get mobile_carrier resource handler
    pub fn mobile_carrier(&self) -> resources::Mobile_carrier<'_> {
        resources::Mobile_carrier::new(self.provider)
    }
    /// Get ad resource handler
    pub fn ad(&self) -> resources::Ad<'_> {
        resources::Ad::new(self.provider)
    }
    /// Get account resource handler
    pub fn account(&self) -> resources::Account<'_> {
        resources::Account::new(self.provider)
    }
    /// Get change_log resource handler
    pub fn change_log(&self) -> resources::Change_log<'_> {
        resources::Change_log::new(self.provider)
    }
    /// Get region resource handler
    pub fn region(&self) -> resources::Region<'_> {
        resources::Region::new(self.provider)
    }
    /// Get subaccount resource handler
    pub fn subaccount(&self) -> resources::Subaccount<'_> {
        resources::Subaccount::new(self.provider)
    }
    /// Get placement resource handler
    pub fn placement(&self) -> resources::Placement<'_> {
        resources::Placement::new(self.provider)
    }
    /// Get file resource handler
    pub fn file(&self) -> resources::File<'_> {
        resources::File::new(self.provider)
    }
    /// Get countrie resource handler
    pub fn countrie(&self) -> resources::Countrie<'_> {
        resources::Countrie::new(self.provider)
    }
    /// Get advertiser_landing_page resource handler
    pub fn advertiser_landing_page(&self) -> resources::Advertiser_landing_page<'_> {
        resources::Advertiser_landing_page::new(self.provider)
    }
    /// Get advertiser_group resource handler
    pub fn advertiser_group(&self) -> resources::Advertiser_group<'_> {
        resources::Advertiser_group::new(self.provider)
    }
    /// Get floodlight_configuration resource handler
    pub fn floodlight_configuration(&self) -> resources::Floodlight_configuration<'_> {
        resources::Floodlight_configuration::new(self.provider)
    }
    /// Get user_role resource handler
    pub fn user_role(&self) -> resources::User_role<'_> {
        resources::User_role::new(self.provider)
    }
    /// Get user_profile resource handler
    pub fn user_profile(&self) -> resources::User_profile<'_> {
        resources::User_profile::new(self.provider)
    }
    /// Get order_document resource handler
    pub fn order_document(&self) -> resources::Order_document<'_> {
        resources::Order_document::new(self.provider)
    }
    /// Get creative_group resource handler
    pub fn creative_group(&self) -> resources::Creative_group<'_> {
        resources::Creative_group::new(self.provider)
    }
    /// Get account_permission_group resource handler
    pub fn account_permission_group(&self) -> resources::Account_permission_group<'_> {
        resources::Account_permission_group::new(self.provider)
    }
    /// Get language resource handler
    pub fn language(&self) -> resources::Language<'_> {
        resources::Language::new(self.provider)
    }
    /// Get video_format resource handler
    pub fn video_format(&self) -> resources::Video_format<'_> {
        resources::Video_format::new(self.provider)
    }
    /// Get dynamic_targeting_key resource handler
    pub fn dynamic_targeting_key(&self) -> resources::Dynamic_targeting_key<'_> {
        resources::Dynamic_targeting_key::new(self.provider)
    }
    /// Get compatible_field resource handler
    pub fn compatible_field(&self) -> resources::Compatible_field<'_> {
        resources::Compatible_field::new(self.provider)
    }
    /// Get operating_system_version resource handler
    pub fn operating_system_version(&self) -> resources::Operating_system_version<'_> {
        resources::Operating_system_version::new(self.provider)
    }
    /// Get remarketing_list resource handler
    pub fn remarketing_list(&self) -> resources::Remarketing_list<'_> {
        resources::Remarketing_list::new(self.provider)
    }
    /// Get content_categorie resource handler
    pub fn content_categorie(&self) -> resources::Content_categorie<'_> {
        resources::Content_categorie::new(self.provider)
    }
    /// Get placement_group resource handler
    pub fn placement_group(&self) -> resources::Placement_group<'_> {
        resources::Placement_group::new(self.provider)
    }
    /// Get creative_asset resource handler
    pub fn creative_asset(&self) -> resources::Creative_asset<'_> {
        resources::Creative_asset::new(self.provider)
    }
    /// Get targetable_remarketing_list resource handler
    pub fn targetable_remarketing_list(&self) -> resources::Targetable_remarketing_list<'_> {
        resources::Targetable_remarketing_list::new(self.provider)
    }
    /// Get directory_site resource handler
    pub fn directory_site(&self) -> resources::Directory_site<'_> {
        resources::Directory_site::new(self.provider)
    }
    /// Get report resource handler
    pub fn report(&self) -> resources::Report<'_> {
        resources::Report::new(self.provider)
    }
    /// Get connection_type resource handler
    pub fn connection_type(&self) -> resources::Connection_type<'_> {
        resources::Connection_type::new(self.provider)
    }
    /// Get campaign_creative_association resource handler
    pub fn campaign_creative_association(&self) -> resources::Campaign_creative_association<'_> {
        resources::Campaign_creative_association::new(self.provider)
    }
    /// Get creative_field resource handler
    pub fn creative_field(&self) -> resources::Creative_field<'_> {
        resources::Creative_field::new(self.provider)
    }
    /// Get citie resource handler
    pub fn citie(&self) -> resources::Citie<'_> {
        resources::Citie::new(self.provider)
    }
    /// Get platform_type resource handler
    pub fn platform_type(&self) -> resources::Platform_type<'_> {
        resources::Platform_type::new(self.provider)
    }
    /// Get floodlight_activity_group resource handler
    pub fn floodlight_activity_group(&self) -> resources::Floodlight_activity_group<'_> {
        resources::Floodlight_activity_group::new(self.provider)
    }
    /// Get inventory_item resource handler
    pub fn inventory_item(&self) -> resources::Inventory_item<'_> {
        resources::Inventory_item::new(self.provider)
    }
    /// Get creative resource handler
    pub fn creative(&self) -> resources::Creative<'_> {
        resources::Creative::new(self.provider)
    }
    /// Get account_active_ad_summarie resource handler
    pub fn account_active_ad_summarie(&self) -> resources::Account_active_ad_summarie<'_> {
        resources::Account_active_ad_summarie::new(self.provider)
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
