//! Displayvideo_api service for Gcp provider
//!
//! This module handles all displayvideo_api resources and their CRUD operations.

use hemmer_core::Result;
use hemmer_provider::{ResourceInput, ResourceOutput, ResourcePlan};

/// Displayvideo_api service handler
pub struct Displayvideo_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Displayvideo_apiService<'a> {
    /// Create a new service handler
    pub fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Plan changes to a resource
    pub async fn plan_resource(
        &self,
        resource_name: &str,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        match resource_name {
            "inventory_source_group" => {
                self.plan_inventory_source_group(current_state, desired_input)
                    .await
            }
            "user" => self.plan_user(current_state, desired_input).await,
            "operation" => self.plan_operation(current_state, desired_input).await,
            "asset" => self.plan_asset(current_state, desired_input).await,
            "media" => self.plan_media(current_state, desired_input).await,
            "custom_bidding_algorithm" => {
                self.plan_custom_bidding_algorithm(current_state, desired_input)
                    .await
            }
            "targeting_option" => {
                self.plan_targeting_option(current_state, desired_input)
                    .await
            }
            "guaranteed_order" => {
                self.plan_guaranteed_order(current_state, desired_input)
                    .await
            }
            "campaign" => self.plan_campaign(current_state, desired_input).await,
            "negative_keyword_list" => {
                self.plan_negative_keyword_list(current_state, desired_input)
                    .await
            }
            "partner" => self.plan_partner(current_state, desired_input).await,
            "assigned_inventory_source" => {
                self.plan_assigned_inventory_source(current_state, desired_input)
                    .await
            }
            "combined_audience" => {
                self.plan_combined_audience(current_state, desired_input)
                    .await
            }
            "creative" => self.plan_creative(current_state, desired_input).await,
            "sdfdownloadtask" => {
                self.plan_sdfdownloadtask(current_state, desired_input)
                    .await
            }
            "site" => self.plan_site(current_state, desired_input).await,
            "first_and_third_party_audience" => {
                self.plan_first_and_third_party_audience(current_state, desired_input)
                    .await
            }
            "insertion_order" => {
                self.plan_insertion_order(current_state, desired_input)
                    .await
            }
            "invoice" => self.plan_invoice(current_state, desired_input).await,
            "negative_keyword" => {
                self.plan_negative_keyword(current_state, desired_input)
                    .await
            }
            "advertiser" => self.plan_advertiser(current_state, desired_input).await,
            "google_audience" => {
                self.plan_google_audience(current_state, desired_input)
                    .await
            }
            "assigned_targeting_option" => {
                self.plan_assigned_targeting_option(current_state, desired_input)
                    .await
            }
            "manual_trigger" => self.plan_manual_trigger(current_state, desired_input).await,
            "channel" => self.plan_channel(current_state, desired_input).await,
            "custom_list" => self.plan_custom_list(current_state, desired_input).await,
            "assigned_location" => {
                self.plan_assigned_location(current_state, desired_input)
                    .await
            }
            "line_item" => self.plan_line_item(current_state, desired_input).await,
            "script" => self.plan_script(current_state, desired_input).await,
            "location_list" => self.plan_location_list(current_state, desired_input).await,
            "floodlight_group" => {
                self.plan_floodlight_group(current_state, desired_input)
                    .await
            }
            "inventory_source" => {
                self.plan_inventory_source(current_state, desired_input)
                    .await
            }
            "media" => self.plan_media(current_state, desired_input).await,
            "operation" => self.plan_operation(current_state, desired_input).await,
            "youtube_ad_group_ad" => {
                self.plan_youtube_ad_group_ad(current_state, desired_input)
                    .await
            }
            "assigned_inventory_source" => {
                self.plan_assigned_inventory_source(current_state, desired_input)
                    .await
            }
            "inventory_source" => {
                self.plan_inventory_source(current_state, desired_input)
                    .await
            }
            "negative_keyword_list" => {
                self.plan_negative_keyword_list(current_state, desired_input)
                    .await
            }
            "media" => self.plan_media(current_state, desired_input).await,
            "partner" => self.plan_partner(current_state, desired_input).await,
            "line_item" => self.plan_line_item(current_state, desired_input).await,
            "advertiser" => self.plan_advertiser(current_state, desired_input).await,
            "youtube_ad_group" => {
                self.plan_youtube_ad_group(current_state, desired_input)
                    .await
            }
            "negative_keyword" => {
                self.plan_negative_keyword(current_state, desired_input)
                    .await
            }
            "assigned_location" => {
                self.plan_assigned_location(current_state, desired_input)
                    .await
            }
            "user" => self.plan_user(current_state, desired_input).await,
            "sdfdownloadtask" => {
                self.plan_sdfdownloadtask(current_state, desired_input)
                    .await
            }
            "google_audience" => {
                self.plan_google_audience(current_state, desired_input)
                    .await
            }
            "campaign" => self.plan_campaign(current_state, desired_input).await,
            "site" => self.plan_site(current_state, desired_input).await,
            "floodlight_activitie" => {
                self.plan_floodlight_activitie(current_state, desired_input)
                    .await
            }
            "combined_audience" => {
                self.plan_combined_audience(current_state, desired_input)
                    .await
            }
            "targeting_option" => {
                self.plan_targeting_option(current_state, desired_input)
                    .await
            }
            "custom_list" => self.plan_custom_list(current_state, desired_input).await,
            "creative" => self.plan_creative(current_state, desired_input).await,
            "channel" => self.plan_channel(current_state, desired_input).await,
            "asset" => self.plan_asset(current_state, desired_input).await,
            "operation" => self.plan_operation(current_state, desired_input).await,
            "location_list" => self.plan_location_list(current_state, desired_input).await,
            "floodlight_group" => {
                self.plan_floodlight_group(current_state, desired_input)
                    .await
            }
            "custom_bidding_algorithm" => {
                self.plan_custom_bidding_algorithm(current_state, desired_input)
                    .await
            }
            "guaranteed_order" => {
                self.plan_guaranteed_order(current_state, desired_input)
                    .await
            }
            "script" => self.plan_script(current_state, desired_input).await,
            "insertion_order" => {
                self.plan_insertion_order(current_state, desired_input)
                    .await
            }
            "manual_trigger" => self.plan_manual_trigger(current_state, desired_input).await,
            "inventory_source_group" => {
                self.plan_inventory_source_group(current_state, desired_input)
                    .await
            }
            "assigned_targeting_option" => {
                self.plan_assigned_targeting_option(current_state, desired_input)
                    .await
            }
            "invoice" => self.plan_invoice(current_state, desired_input).await,
            "media" => self.plan_media(current_state, desired_input).await,
            "operation" => self.plan_operation(current_state, desired_input).await,
            "campaign" => self.plan_campaign(current_state, desired_input).await,
            "advertiser" => self.plan_advertiser(current_state, desired_input).await,
            "negative_keyword_list" => {
                self.plan_negative_keyword_list(current_state, desired_input)
                    .await
            }
            "line_item" => self.plan_line_item(current_state, desired_input).await,
            "assigned_targeting_option" => {
                self.plan_assigned_targeting_option(current_state, desired_input)
                    .await
            }
            "script" => self.plan_script(current_state, desired_input).await,
            "youtube_asset_association" => {
                self.plan_youtube_asset_association(current_state, desired_input)
                    .await
            }
            "media" => self.plan_media(current_state, desired_input).await,
            "floodlight_group" => {
                self.plan_floodlight_group(current_state, desired_input)
                    .await
            }
            "assigned_location" => {
                self.plan_assigned_location(current_state, desired_input)
                    .await
            }
            "negative_keyword" => {
                self.plan_negative_keyword(current_state, desired_input)
                    .await
            }
            "inventory_source" => {
                self.plan_inventory_source(current_state, desired_input)
                    .await
            }
            "assigned_inventory_source" => {
                self.plan_assigned_inventory_source(current_state, desired_input)
                    .await
            }
            "first_party_and_partner_audience" => {
                self.plan_first_party_and_partner_audience(current_state, desired_input)
                    .await
            }
            "insertion_order" => {
                self.plan_insertion_order(current_state, desired_input)
                    .await
            }
            "sdfdownloadtask" => {
                self.plan_sdfdownloadtask(current_state, desired_input)
                    .await
            }
            "ad_asset" => self.plan_ad_asset(current_state, desired_input).await,
            "operation" => self.plan_operation(current_state, desired_input).await,
            "google_audience" => {
                self.plan_google_audience(current_state, desired_input)
                    .await
            }
            "user" => self.plan_user(current_state, desired_input).await,
            "guaranteed_order" => {
                self.plan_guaranteed_order(current_state, desired_input)
                    .await
            }
            "ad_group_ad" => self.plan_ad_group_ad(current_state, desired_input).await,
            "partner" => self.plan_partner(current_state, desired_input).await,
            "floodlight_activitie" => {
                self.plan_floodlight_activitie(current_state, desired_input)
                    .await
            }
            "creative" => self.plan_creative(current_state, desired_input).await,
            "custom_list" => self.plan_custom_list(current_state, desired_input).await,
            "combined_audience" => {
                self.plan_combined_audience(current_state, desired_input)
                    .await
            }
            "channel" => self.plan_channel(current_state, desired_input).await,
            "asset" => self.plan_asset(current_state, desired_input).await,
            "invoice" => self.plan_invoice(current_state, desired_input).await,
            "rule" => self.plan_rule(current_state, desired_input).await,
            "inventory_source_group" => {
                self.plan_inventory_source_group(current_state, desired_input)
                    .await
            }
            "location_list" => self.plan_location_list(current_state, desired_input).await,
            "custom_bidding_algorithm" => {
                self.plan_custom_bidding_algorithm(current_state, desired_input)
                    .await
            }
            "site" => self.plan_site(current_state, desired_input).await,
            "ad_group" => self.plan_ad_group(current_state, desired_input).await,
            "targeting_option" => {
                self.plan_targeting_option(current_state, desired_input)
                    .await
            }
            "rule" => self.plan_rule(current_state, desired_input).await,
            "guaranteed_order" => {
                self.plan_guaranteed_order(current_state, desired_input)
                    .await
            }
            "script" => self.plan_script(current_state, desired_input).await,
            "invoice" => self.plan_invoice(current_state, desired_input).await,
            "negative_keyword_list" => {
                self.plan_negative_keyword_list(current_state, desired_input)
                    .await
            }
            "assigned_inventory_source" => {
                self.plan_assigned_inventory_source(current_state, desired_input)
                    .await
            }
            "insertion_order" => {
                self.plan_insertion_order(current_state, desired_input)
                    .await
            }
            "negative_keyword" => {
                self.plan_negative_keyword(current_state, desired_input)
                    .await
            }
            "creative" => self.plan_creative(current_state, desired_input).await,
            "custom_list" => self.plan_custom_list(current_state, desired_input).await,
            "sdfdownloadtask" => {
                self.plan_sdfdownloadtask(current_state, desired_input)
                    .await
            }
            "custom_bidding_algorithm" => {
                self.plan_custom_bidding_algorithm(current_state, desired_input)
                    .await
            }
            "assigned_targeting_option" => {
                self.plan_assigned_targeting_option(current_state, desired_input)
                    .await
            }
            "line_item" => self.plan_line_item(current_state, desired_input).await,
            "floodlight_activitie" => {
                self.plan_floodlight_activitie(current_state, desired_input)
                    .await
            }
            "campaign" => self.plan_campaign(current_state, desired_input).await,
            "inventory_source" => {
                self.plan_inventory_source(current_state, desired_input)
                    .await
            }
            "partner" => self.plan_partner(current_state, desired_input).await,
            "first_and_third_party_audience" => {
                self.plan_first_and_third_party_audience(current_state, desired_input)
                    .await
            }
            "channel" => self.plan_channel(current_state, desired_input).await,
            "targeting_option" => {
                self.plan_targeting_option(current_state, desired_input)
                    .await
            }
            "location_list" => self.plan_location_list(current_state, desired_input).await,
            "assigned_location" => {
                self.plan_assigned_location(current_state, desired_input)
                    .await
            }
            "ad_group" => self.plan_ad_group(current_state, desired_input).await,
            "ad_group_ad" => self.plan_ad_group_ad(current_state, desired_input).await,
            "user" => self.plan_user(current_state, desired_input).await,
            "asset" => self.plan_asset(current_state, desired_input).await,
            "advertiser" => self.plan_advertiser(current_state, desired_input).await,
            "operation" => self.plan_operation(current_state, desired_input).await,
            "floodlight_group" => {
                self.plan_floodlight_group(current_state, desired_input)
                    .await
            }
            "combined_audience" => {
                self.plan_combined_audience(current_state, desired_input)
                    .await
            }
            "google_audience" => {
                self.plan_google_audience(current_state, desired_input)
                    .await
            }
            "site" => self.plan_site(current_state, desired_input).await,
            "inventory_source_group" => {
                self.plan_inventory_source_group(current_state, desired_input)
                    .await
            }
            "media" => self.plan_media(current_state, desired_input).await,
            "media" => self.plan_media(current_state, desired_input).await,
            "operation" => self.plan_operation(current_state, desired_input).await,
            _ => Err(hemmer_core::HemmerError::Provider(format!(
                "Unknown resource type: {}.{}",
                "displayvideo_api", resource_name
            ))),
        }
    }

    /// Create a new resource
    pub async fn create_resource(
        &self,
        resource_name: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        match resource_name {
            "inventory_source_group" => self.create_inventory_source_group(input).await,
            "user" => self.create_user(input).await,
            "operation" => self.create_operation(input).await,
            "asset" => self.create_asset(input).await,
            "media" => self.create_media(input).await,
            "custom_bidding_algorithm" => self.create_custom_bidding_algorithm(input).await,
            "targeting_option" => self.create_targeting_option(input).await,
            "guaranteed_order" => self.create_guaranteed_order(input).await,
            "campaign" => self.create_campaign(input).await,
            "negative_keyword_list" => self.create_negative_keyword_list(input).await,
            "partner" => self.create_partner(input).await,
            "assigned_inventory_source" => self.create_assigned_inventory_source(input).await,
            "combined_audience" => self.create_combined_audience(input).await,
            "creative" => self.create_creative(input).await,
            "sdfdownloadtask" => self.create_sdfdownloadtask(input).await,
            "site" => self.create_site(input).await,
            "first_and_third_party_audience" => {
                self.create_first_and_third_party_audience(input).await
            }
            "insertion_order" => self.create_insertion_order(input).await,
            "invoice" => self.create_invoice(input).await,
            "negative_keyword" => self.create_negative_keyword(input).await,
            "advertiser" => self.create_advertiser(input).await,
            "google_audience" => self.create_google_audience(input).await,
            "assigned_targeting_option" => self.create_assigned_targeting_option(input).await,
            "manual_trigger" => self.create_manual_trigger(input).await,
            "channel" => self.create_channel(input).await,
            "custom_list" => self.create_custom_list(input).await,
            "assigned_location" => self.create_assigned_location(input).await,
            "line_item" => self.create_line_item(input).await,
            "script" => self.create_script(input).await,
            "location_list" => self.create_location_list(input).await,
            "floodlight_group" => self.create_floodlight_group(input).await,
            "inventory_source" => self.create_inventory_source(input).await,
            "media" => self.create_media(input).await,
            "operation" => self.create_operation(input).await,
            "youtube_ad_group_ad" => self.create_youtube_ad_group_ad(input).await,
            "assigned_inventory_source" => self.create_assigned_inventory_source(input).await,
            "inventory_source" => self.create_inventory_source(input).await,
            "negative_keyword_list" => self.create_negative_keyword_list(input).await,
            "media" => self.create_media(input).await,
            "partner" => self.create_partner(input).await,
            "line_item" => self.create_line_item(input).await,
            "advertiser" => self.create_advertiser(input).await,
            "youtube_ad_group" => self.create_youtube_ad_group(input).await,
            "negative_keyword" => self.create_negative_keyword(input).await,
            "assigned_location" => self.create_assigned_location(input).await,
            "user" => self.create_user(input).await,
            "sdfdownloadtask" => self.create_sdfdownloadtask(input).await,
            "google_audience" => self.create_google_audience(input).await,
            "campaign" => self.create_campaign(input).await,
            "site" => self.create_site(input).await,
            "floodlight_activitie" => self.create_floodlight_activitie(input).await,
            "combined_audience" => self.create_combined_audience(input).await,
            "targeting_option" => self.create_targeting_option(input).await,
            "custom_list" => self.create_custom_list(input).await,
            "creative" => self.create_creative(input).await,
            "channel" => self.create_channel(input).await,
            "asset" => self.create_asset(input).await,
            "operation" => self.create_operation(input).await,
            "location_list" => self.create_location_list(input).await,
            "floodlight_group" => self.create_floodlight_group(input).await,
            "custom_bidding_algorithm" => self.create_custom_bidding_algorithm(input).await,
            "guaranteed_order" => self.create_guaranteed_order(input).await,
            "script" => self.create_script(input).await,
            "insertion_order" => self.create_insertion_order(input).await,
            "manual_trigger" => self.create_manual_trigger(input).await,
            "inventory_source_group" => self.create_inventory_source_group(input).await,
            "assigned_targeting_option" => self.create_assigned_targeting_option(input).await,
            "invoice" => self.create_invoice(input).await,
            "media" => self.create_media(input).await,
            "operation" => self.create_operation(input).await,
            "campaign" => self.create_campaign(input).await,
            "advertiser" => self.create_advertiser(input).await,
            "negative_keyword_list" => self.create_negative_keyword_list(input).await,
            "line_item" => self.create_line_item(input).await,
            "assigned_targeting_option" => self.create_assigned_targeting_option(input).await,
            "script" => self.create_script(input).await,
            "youtube_asset_association" => self.create_youtube_asset_association(input).await,
            "media" => self.create_media(input).await,
            "floodlight_group" => self.create_floodlight_group(input).await,
            "assigned_location" => self.create_assigned_location(input).await,
            "negative_keyword" => self.create_negative_keyword(input).await,
            "inventory_source" => self.create_inventory_source(input).await,
            "assigned_inventory_source" => self.create_assigned_inventory_source(input).await,
            "first_party_and_partner_audience" => {
                self.create_first_party_and_partner_audience(input).await
            }
            "insertion_order" => self.create_insertion_order(input).await,
            "sdfdownloadtask" => self.create_sdfdownloadtask(input).await,
            "ad_asset" => self.create_ad_asset(input).await,
            "operation" => self.create_operation(input).await,
            "google_audience" => self.create_google_audience(input).await,
            "user" => self.create_user(input).await,
            "guaranteed_order" => self.create_guaranteed_order(input).await,
            "ad_group_ad" => self.create_ad_group_ad(input).await,
            "partner" => self.create_partner(input).await,
            "floodlight_activitie" => self.create_floodlight_activitie(input).await,
            "creative" => self.create_creative(input).await,
            "custom_list" => self.create_custom_list(input).await,
            "combined_audience" => self.create_combined_audience(input).await,
            "channel" => self.create_channel(input).await,
            "asset" => self.create_asset(input).await,
            "invoice" => self.create_invoice(input).await,
            "rule" => self.create_rule(input).await,
            "inventory_source_group" => self.create_inventory_source_group(input).await,
            "location_list" => self.create_location_list(input).await,
            "custom_bidding_algorithm" => self.create_custom_bidding_algorithm(input).await,
            "site" => self.create_site(input).await,
            "ad_group" => self.create_ad_group(input).await,
            "targeting_option" => self.create_targeting_option(input).await,
            "rule" => self.create_rule(input).await,
            "guaranteed_order" => self.create_guaranteed_order(input).await,
            "script" => self.create_script(input).await,
            "invoice" => self.create_invoice(input).await,
            "negative_keyword_list" => self.create_negative_keyword_list(input).await,
            "assigned_inventory_source" => self.create_assigned_inventory_source(input).await,
            "insertion_order" => self.create_insertion_order(input).await,
            "negative_keyword" => self.create_negative_keyword(input).await,
            "creative" => self.create_creative(input).await,
            "custom_list" => self.create_custom_list(input).await,
            "sdfdownloadtask" => self.create_sdfdownloadtask(input).await,
            "custom_bidding_algorithm" => self.create_custom_bidding_algorithm(input).await,
            "assigned_targeting_option" => self.create_assigned_targeting_option(input).await,
            "line_item" => self.create_line_item(input).await,
            "floodlight_activitie" => self.create_floodlight_activitie(input).await,
            "campaign" => self.create_campaign(input).await,
            "inventory_source" => self.create_inventory_source(input).await,
            "partner" => self.create_partner(input).await,
            "first_and_third_party_audience" => {
                self.create_first_and_third_party_audience(input).await
            }
            "channel" => self.create_channel(input).await,
            "targeting_option" => self.create_targeting_option(input).await,
            "location_list" => self.create_location_list(input).await,
            "assigned_location" => self.create_assigned_location(input).await,
            "ad_group" => self.create_ad_group(input).await,
            "ad_group_ad" => self.create_ad_group_ad(input).await,
            "user" => self.create_user(input).await,
            "asset" => self.create_asset(input).await,
            "advertiser" => self.create_advertiser(input).await,
            "operation" => self.create_operation(input).await,
            "floodlight_group" => self.create_floodlight_group(input).await,
            "combined_audience" => self.create_combined_audience(input).await,
            "google_audience" => self.create_google_audience(input).await,
            "site" => self.create_site(input).await,
            "inventory_source_group" => self.create_inventory_source_group(input).await,
            "media" => self.create_media(input).await,
            "media" => self.create_media(input).await,
            "operation" => self.create_operation(input).await,
            _ => Err(hemmer_core::HemmerError::Provider(format!(
                "Unknown resource type: {}.{}",
                "displayvideo_api", resource_name
            ))),
        }
    }

    /// Read resource state
    pub async fn read_resource(&self, resource_name: &str, id: &str) -> Result<ResourceOutput> {
        match resource_name {
            "inventory_source_group" => self.read_inventory_source_group(id).await,
            "user" => self.read_user(id).await,
            "operation" => self.read_operation(id).await,
            "asset" => self.read_asset(id).await,
            "media" => self.read_media(id).await,
            "custom_bidding_algorithm" => self.read_custom_bidding_algorithm(id).await,
            "targeting_option" => self.read_targeting_option(id).await,
            "guaranteed_order" => self.read_guaranteed_order(id).await,
            "campaign" => self.read_campaign(id).await,
            "negative_keyword_list" => self.read_negative_keyword_list(id).await,
            "partner" => self.read_partner(id).await,
            "assigned_inventory_source" => self.read_assigned_inventory_source(id).await,
            "combined_audience" => self.read_combined_audience(id).await,
            "creative" => self.read_creative(id).await,
            "sdfdownloadtask" => self.read_sdfdownloadtask(id).await,
            "site" => self.read_site(id).await,
            "first_and_third_party_audience" => self.read_first_and_third_party_audience(id).await,
            "insertion_order" => self.read_insertion_order(id).await,
            "invoice" => self.read_invoice(id).await,
            "negative_keyword" => self.read_negative_keyword(id).await,
            "advertiser" => self.read_advertiser(id).await,
            "google_audience" => self.read_google_audience(id).await,
            "assigned_targeting_option" => self.read_assigned_targeting_option(id).await,
            "manual_trigger" => self.read_manual_trigger(id).await,
            "channel" => self.read_channel(id).await,
            "custom_list" => self.read_custom_list(id).await,
            "assigned_location" => self.read_assigned_location(id).await,
            "line_item" => self.read_line_item(id).await,
            "script" => self.read_script(id).await,
            "location_list" => self.read_location_list(id).await,
            "floodlight_group" => self.read_floodlight_group(id).await,
            "inventory_source" => self.read_inventory_source(id).await,
            "media" => self.read_media(id).await,
            "operation" => self.read_operation(id).await,
            "youtube_ad_group_ad" => self.read_youtube_ad_group_ad(id).await,
            "assigned_inventory_source" => self.read_assigned_inventory_source(id).await,
            "inventory_source" => self.read_inventory_source(id).await,
            "negative_keyword_list" => self.read_negative_keyword_list(id).await,
            "media" => self.read_media(id).await,
            "partner" => self.read_partner(id).await,
            "line_item" => self.read_line_item(id).await,
            "advertiser" => self.read_advertiser(id).await,
            "youtube_ad_group" => self.read_youtube_ad_group(id).await,
            "negative_keyword" => self.read_negative_keyword(id).await,
            "assigned_location" => self.read_assigned_location(id).await,
            "user" => self.read_user(id).await,
            "sdfdownloadtask" => self.read_sdfdownloadtask(id).await,
            "google_audience" => self.read_google_audience(id).await,
            "campaign" => self.read_campaign(id).await,
            "site" => self.read_site(id).await,
            "floodlight_activitie" => self.read_floodlight_activitie(id).await,
            "combined_audience" => self.read_combined_audience(id).await,
            "targeting_option" => self.read_targeting_option(id).await,
            "custom_list" => self.read_custom_list(id).await,
            "creative" => self.read_creative(id).await,
            "channel" => self.read_channel(id).await,
            "asset" => self.read_asset(id).await,
            "operation" => self.read_operation(id).await,
            "location_list" => self.read_location_list(id).await,
            "floodlight_group" => self.read_floodlight_group(id).await,
            "custom_bidding_algorithm" => self.read_custom_bidding_algorithm(id).await,
            "guaranteed_order" => self.read_guaranteed_order(id).await,
            "script" => self.read_script(id).await,
            "insertion_order" => self.read_insertion_order(id).await,
            "manual_trigger" => self.read_manual_trigger(id).await,
            "inventory_source_group" => self.read_inventory_source_group(id).await,
            "assigned_targeting_option" => self.read_assigned_targeting_option(id).await,
            "invoice" => self.read_invoice(id).await,
            "media" => self.read_media(id).await,
            "operation" => self.read_operation(id).await,
            "campaign" => self.read_campaign(id).await,
            "advertiser" => self.read_advertiser(id).await,
            "negative_keyword_list" => self.read_negative_keyword_list(id).await,
            "line_item" => self.read_line_item(id).await,
            "assigned_targeting_option" => self.read_assigned_targeting_option(id).await,
            "script" => self.read_script(id).await,
            "youtube_asset_association" => self.read_youtube_asset_association(id).await,
            "media" => self.read_media(id).await,
            "floodlight_group" => self.read_floodlight_group(id).await,
            "assigned_location" => self.read_assigned_location(id).await,
            "negative_keyword" => self.read_negative_keyword(id).await,
            "inventory_source" => self.read_inventory_source(id).await,
            "assigned_inventory_source" => self.read_assigned_inventory_source(id).await,
            "first_party_and_partner_audience" => {
                self.read_first_party_and_partner_audience(id).await
            }
            "insertion_order" => self.read_insertion_order(id).await,
            "sdfdownloadtask" => self.read_sdfdownloadtask(id).await,
            "ad_asset" => self.read_ad_asset(id).await,
            "operation" => self.read_operation(id).await,
            "google_audience" => self.read_google_audience(id).await,
            "user" => self.read_user(id).await,
            "guaranteed_order" => self.read_guaranteed_order(id).await,
            "ad_group_ad" => self.read_ad_group_ad(id).await,
            "partner" => self.read_partner(id).await,
            "floodlight_activitie" => self.read_floodlight_activitie(id).await,
            "creative" => self.read_creative(id).await,
            "custom_list" => self.read_custom_list(id).await,
            "combined_audience" => self.read_combined_audience(id).await,
            "channel" => self.read_channel(id).await,
            "asset" => self.read_asset(id).await,
            "invoice" => self.read_invoice(id).await,
            "rule" => self.read_rule(id).await,
            "inventory_source_group" => self.read_inventory_source_group(id).await,
            "location_list" => self.read_location_list(id).await,
            "custom_bidding_algorithm" => self.read_custom_bidding_algorithm(id).await,
            "site" => self.read_site(id).await,
            "ad_group" => self.read_ad_group(id).await,
            "targeting_option" => self.read_targeting_option(id).await,
            "rule" => self.read_rule(id).await,
            "guaranteed_order" => self.read_guaranteed_order(id).await,
            "script" => self.read_script(id).await,
            "invoice" => self.read_invoice(id).await,
            "negative_keyword_list" => self.read_negative_keyword_list(id).await,
            "assigned_inventory_source" => self.read_assigned_inventory_source(id).await,
            "insertion_order" => self.read_insertion_order(id).await,
            "negative_keyword" => self.read_negative_keyword(id).await,
            "creative" => self.read_creative(id).await,
            "custom_list" => self.read_custom_list(id).await,
            "sdfdownloadtask" => self.read_sdfdownloadtask(id).await,
            "custom_bidding_algorithm" => self.read_custom_bidding_algorithm(id).await,
            "assigned_targeting_option" => self.read_assigned_targeting_option(id).await,
            "line_item" => self.read_line_item(id).await,
            "floodlight_activitie" => self.read_floodlight_activitie(id).await,
            "campaign" => self.read_campaign(id).await,
            "inventory_source" => self.read_inventory_source(id).await,
            "partner" => self.read_partner(id).await,
            "first_and_third_party_audience" => self.read_first_and_third_party_audience(id).await,
            "channel" => self.read_channel(id).await,
            "targeting_option" => self.read_targeting_option(id).await,
            "location_list" => self.read_location_list(id).await,
            "assigned_location" => self.read_assigned_location(id).await,
            "ad_group" => self.read_ad_group(id).await,
            "ad_group_ad" => self.read_ad_group_ad(id).await,
            "user" => self.read_user(id).await,
            "asset" => self.read_asset(id).await,
            "advertiser" => self.read_advertiser(id).await,
            "operation" => self.read_operation(id).await,
            "floodlight_group" => self.read_floodlight_group(id).await,
            "combined_audience" => self.read_combined_audience(id).await,
            "google_audience" => self.read_google_audience(id).await,
            "site" => self.read_site(id).await,
            "inventory_source_group" => self.read_inventory_source_group(id).await,
            "media" => self.read_media(id).await,
            "media" => self.read_media(id).await,
            "operation" => self.read_operation(id).await,
            _ => Err(hemmer_core::HemmerError::Provider(format!(
                "Unknown resource type: {}.{}",
                "displayvideo_api", resource_name
            ))),
        }
    }

    /// Update an existing resource
    pub async fn update_resource(
        &self,
        resource_name: &str,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        match resource_name {
            "inventory_source_group" => self.update_inventory_source_group(id, input).await,
            "user" => self.update_user(id, input).await,
            "operation" => self.update_operation(id, input).await,
            "asset" => self.update_asset(id, input).await,
            "media" => self.update_media(id, input).await,
            "custom_bidding_algorithm" => self.update_custom_bidding_algorithm(id, input).await,
            "targeting_option" => self.update_targeting_option(id, input).await,
            "guaranteed_order" => self.update_guaranteed_order(id, input).await,
            "campaign" => self.update_campaign(id, input).await,
            "negative_keyword_list" => self.update_negative_keyword_list(id, input).await,
            "partner" => self.update_partner(id, input).await,
            "assigned_inventory_source" => self.update_assigned_inventory_source(id, input).await,
            "combined_audience" => self.update_combined_audience(id, input).await,
            "creative" => self.update_creative(id, input).await,
            "sdfdownloadtask" => self.update_sdfdownloadtask(id, input).await,
            "site" => self.update_site(id, input).await,
            "first_and_third_party_audience" => {
                self.update_first_and_third_party_audience(id, input).await
            }
            "insertion_order" => self.update_insertion_order(id, input).await,
            "invoice" => self.update_invoice(id, input).await,
            "negative_keyword" => self.update_negative_keyword(id, input).await,
            "advertiser" => self.update_advertiser(id, input).await,
            "google_audience" => self.update_google_audience(id, input).await,
            "assigned_targeting_option" => self.update_assigned_targeting_option(id, input).await,
            "manual_trigger" => self.update_manual_trigger(id, input).await,
            "channel" => self.update_channel(id, input).await,
            "custom_list" => self.update_custom_list(id, input).await,
            "assigned_location" => self.update_assigned_location(id, input).await,
            "line_item" => self.update_line_item(id, input).await,
            "script" => self.update_script(id, input).await,
            "location_list" => self.update_location_list(id, input).await,
            "floodlight_group" => self.update_floodlight_group(id, input).await,
            "inventory_source" => self.update_inventory_source(id, input).await,
            "media" => self.update_media(id, input).await,
            "operation" => self.update_operation(id, input).await,
            "youtube_ad_group_ad" => self.update_youtube_ad_group_ad(id, input).await,
            "assigned_inventory_source" => self.update_assigned_inventory_source(id, input).await,
            "inventory_source" => self.update_inventory_source(id, input).await,
            "negative_keyword_list" => self.update_negative_keyword_list(id, input).await,
            "media" => self.update_media(id, input).await,
            "partner" => self.update_partner(id, input).await,
            "line_item" => self.update_line_item(id, input).await,
            "advertiser" => self.update_advertiser(id, input).await,
            "youtube_ad_group" => self.update_youtube_ad_group(id, input).await,
            "negative_keyword" => self.update_negative_keyword(id, input).await,
            "assigned_location" => self.update_assigned_location(id, input).await,
            "user" => self.update_user(id, input).await,
            "sdfdownloadtask" => self.update_sdfdownloadtask(id, input).await,
            "google_audience" => self.update_google_audience(id, input).await,
            "campaign" => self.update_campaign(id, input).await,
            "site" => self.update_site(id, input).await,
            "floodlight_activitie" => self.update_floodlight_activitie(id, input).await,
            "combined_audience" => self.update_combined_audience(id, input).await,
            "targeting_option" => self.update_targeting_option(id, input).await,
            "custom_list" => self.update_custom_list(id, input).await,
            "creative" => self.update_creative(id, input).await,
            "channel" => self.update_channel(id, input).await,
            "asset" => self.update_asset(id, input).await,
            "operation" => self.update_operation(id, input).await,
            "location_list" => self.update_location_list(id, input).await,
            "floodlight_group" => self.update_floodlight_group(id, input).await,
            "custom_bidding_algorithm" => self.update_custom_bidding_algorithm(id, input).await,
            "guaranteed_order" => self.update_guaranteed_order(id, input).await,
            "script" => self.update_script(id, input).await,
            "insertion_order" => self.update_insertion_order(id, input).await,
            "manual_trigger" => self.update_manual_trigger(id, input).await,
            "inventory_source_group" => self.update_inventory_source_group(id, input).await,
            "assigned_targeting_option" => self.update_assigned_targeting_option(id, input).await,
            "invoice" => self.update_invoice(id, input).await,
            "media" => self.update_media(id, input).await,
            "operation" => self.update_operation(id, input).await,
            "campaign" => self.update_campaign(id, input).await,
            "advertiser" => self.update_advertiser(id, input).await,
            "negative_keyword_list" => self.update_negative_keyword_list(id, input).await,
            "line_item" => self.update_line_item(id, input).await,
            "assigned_targeting_option" => self.update_assigned_targeting_option(id, input).await,
            "script" => self.update_script(id, input).await,
            "youtube_asset_association" => self.update_youtube_asset_association(id, input).await,
            "media" => self.update_media(id, input).await,
            "floodlight_group" => self.update_floodlight_group(id, input).await,
            "assigned_location" => self.update_assigned_location(id, input).await,
            "negative_keyword" => self.update_negative_keyword(id, input).await,
            "inventory_source" => self.update_inventory_source(id, input).await,
            "assigned_inventory_source" => self.update_assigned_inventory_source(id, input).await,
            "first_party_and_partner_audience" => {
                self.update_first_party_and_partner_audience(id, input)
                    .await
            }
            "insertion_order" => self.update_insertion_order(id, input).await,
            "sdfdownloadtask" => self.update_sdfdownloadtask(id, input).await,
            "ad_asset" => self.update_ad_asset(id, input).await,
            "operation" => self.update_operation(id, input).await,
            "google_audience" => self.update_google_audience(id, input).await,
            "user" => self.update_user(id, input).await,
            "guaranteed_order" => self.update_guaranteed_order(id, input).await,
            "ad_group_ad" => self.update_ad_group_ad(id, input).await,
            "partner" => self.update_partner(id, input).await,
            "floodlight_activitie" => self.update_floodlight_activitie(id, input).await,
            "creative" => self.update_creative(id, input).await,
            "custom_list" => self.update_custom_list(id, input).await,
            "combined_audience" => self.update_combined_audience(id, input).await,
            "channel" => self.update_channel(id, input).await,
            "asset" => self.update_asset(id, input).await,
            "invoice" => self.update_invoice(id, input).await,
            "rule" => self.update_rule(id, input).await,
            "inventory_source_group" => self.update_inventory_source_group(id, input).await,
            "location_list" => self.update_location_list(id, input).await,
            "custom_bidding_algorithm" => self.update_custom_bidding_algorithm(id, input).await,
            "site" => self.update_site(id, input).await,
            "ad_group" => self.update_ad_group(id, input).await,
            "targeting_option" => self.update_targeting_option(id, input).await,
            "rule" => self.update_rule(id, input).await,
            "guaranteed_order" => self.update_guaranteed_order(id, input).await,
            "script" => self.update_script(id, input).await,
            "invoice" => self.update_invoice(id, input).await,
            "negative_keyword_list" => self.update_negative_keyword_list(id, input).await,
            "assigned_inventory_source" => self.update_assigned_inventory_source(id, input).await,
            "insertion_order" => self.update_insertion_order(id, input).await,
            "negative_keyword" => self.update_negative_keyword(id, input).await,
            "creative" => self.update_creative(id, input).await,
            "custom_list" => self.update_custom_list(id, input).await,
            "sdfdownloadtask" => self.update_sdfdownloadtask(id, input).await,
            "custom_bidding_algorithm" => self.update_custom_bidding_algorithm(id, input).await,
            "assigned_targeting_option" => self.update_assigned_targeting_option(id, input).await,
            "line_item" => self.update_line_item(id, input).await,
            "floodlight_activitie" => self.update_floodlight_activitie(id, input).await,
            "campaign" => self.update_campaign(id, input).await,
            "inventory_source" => self.update_inventory_source(id, input).await,
            "partner" => self.update_partner(id, input).await,
            "first_and_third_party_audience" => {
                self.update_first_and_third_party_audience(id, input).await
            }
            "channel" => self.update_channel(id, input).await,
            "targeting_option" => self.update_targeting_option(id, input).await,
            "location_list" => self.update_location_list(id, input).await,
            "assigned_location" => self.update_assigned_location(id, input).await,
            "ad_group" => self.update_ad_group(id, input).await,
            "ad_group_ad" => self.update_ad_group_ad(id, input).await,
            "user" => self.update_user(id, input).await,
            "asset" => self.update_asset(id, input).await,
            "advertiser" => self.update_advertiser(id, input).await,
            "operation" => self.update_operation(id, input).await,
            "floodlight_group" => self.update_floodlight_group(id, input).await,
            "combined_audience" => self.update_combined_audience(id, input).await,
            "google_audience" => self.update_google_audience(id, input).await,
            "site" => self.update_site(id, input).await,
            "inventory_source_group" => self.update_inventory_source_group(id, input).await,
            "media" => self.update_media(id, input).await,
            "media" => self.update_media(id, input).await,
            "operation" => self.update_operation(id, input).await,
            _ => Err(hemmer_core::HemmerError::Provider(format!(
                "Unknown resource type: {}.{}",
                "displayvideo_api", resource_name
            ))),
        }
    }

    /// Delete a resource
    pub async fn delete_resource(&self, resource_name: &str, id: &str) -> Result<()> {
        match resource_name {
            "inventory_source_group" => self.delete_inventory_source_group(id).await,
            "user" => self.delete_user(id).await,
            "operation" => self.delete_operation(id).await,
            "asset" => self.delete_asset(id).await,
            "media" => self.delete_media(id).await,
            "custom_bidding_algorithm" => self.delete_custom_bidding_algorithm(id).await,
            "targeting_option" => self.delete_targeting_option(id).await,
            "guaranteed_order" => self.delete_guaranteed_order(id).await,
            "campaign" => self.delete_campaign(id).await,
            "negative_keyword_list" => self.delete_negative_keyword_list(id).await,
            "partner" => self.delete_partner(id).await,
            "assigned_inventory_source" => self.delete_assigned_inventory_source(id).await,
            "combined_audience" => self.delete_combined_audience(id).await,
            "creative" => self.delete_creative(id).await,
            "sdfdownloadtask" => self.delete_sdfdownloadtask(id).await,
            "site" => self.delete_site(id).await,
            "first_and_third_party_audience" => {
                self.delete_first_and_third_party_audience(id).await
            }
            "insertion_order" => self.delete_insertion_order(id).await,
            "invoice" => self.delete_invoice(id).await,
            "negative_keyword" => self.delete_negative_keyword(id).await,
            "advertiser" => self.delete_advertiser(id).await,
            "google_audience" => self.delete_google_audience(id).await,
            "assigned_targeting_option" => self.delete_assigned_targeting_option(id).await,
            "manual_trigger" => self.delete_manual_trigger(id).await,
            "channel" => self.delete_channel(id).await,
            "custom_list" => self.delete_custom_list(id).await,
            "assigned_location" => self.delete_assigned_location(id).await,
            "line_item" => self.delete_line_item(id).await,
            "script" => self.delete_script(id).await,
            "location_list" => self.delete_location_list(id).await,
            "floodlight_group" => self.delete_floodlight_group(id).await,
            "inventory_source" => self.delete_inventory_source(id).await,
            "media" => self.delete_media(id).await,
            "operation" => self.delete_operation(id).await,
            "youtube_ad_group_ad" => self.delete_youtube_ad_group_ad(id).await,
            "assigned_inventory_source" => self.delete_assigned_inventory_source(id).await,
            "inventory_source" => self.delete_inventory_source(id).await,
            "negative_keyword_list" => self.delete_negative_keyword_list(id).await,
            "media" => self.delete_media(id).await,
            "partner" => self.delete_partner(id).await,
            "line_item" => self.delete_line_item(id).await,
            "advertiser" => self.delete_advertiser(id).await,
            "youtube_ad_group" => self.delete_youtube_ad_group(id).await,
            "negative_keyword" => self.delete_negative_keyword(id).await,
            "assigned_location" => self.delete_assigned_location(id).await,
            "user" => self.delete_user(id).await,
            "sdfdownloadtask" => self.delete_sdfdownloadtask(id).await,
            "google_audience" => self.delete_google_audience(id).await,
            "campaign" => self.delete_campaign(id).await,
            "site" => self.delete_site(id).await,
            "floodlight_activitie" => self.delete_floodlight_activitie(id).await,
            "combined_audience" => self.delete_combined_audience(id).await,
            "targeting_option" => self.delete_targeting_option(id).await,
            "custom_list" => self.delete_custom_list(id).await,
            "creative" => self.delete_creative(id).await,
            "channel" => self.delete_channel(id).await,
            "asset" => self.delete_asset(id).await,
            "operation" => self.delete_operation(id).await,
            "location_list" => self.delete_location_list(id).await,
            "floodlight_group" => self.delete_floodlight_group(id).await,
            "custom_bidding_algorithm" => self.delete_custom_bidding_algorithm(id).await,
            "guaranteed_order" => self.delete_guaranteed_order(id).await,
            "script" => self.delete_script(id).await,
            "insertion_order" => self.delete_insertion_order(id).await,
            "manual_trigger" => self.delete_manual_trigger(id).await,
            "inventory_source_group" => self.delete_inventory_source_group(id).await,
            "assigned_targeting_option" => self.delete_assigned_targeting_option(id).await,
            "invoice" => self.delete_invoice(id).await,
            "media" => self.delete_media(id).await,
            "operation" => self.delete_operation(id).await,
            "campaign" => self.delete_campaign(id).await,
            "advertiser" => self.delete_advertiser(id).await,
            "negative_keyword_list" => self.delete_negative_keyword_list(id).await,
            "line_item" => self.delete_line_item(id).await,
            "assigned_targeting_option" => self.delete_assigned_targeting_option(id).await,
            "script" => self.delete_script(id).await,
            "youtube_asset_association" => self.delete_youtube_asset_association(id).await,
            "media" => self.delete_media(id).await,
            "floodlight_group" => self.delete_floodlight_group(id).await,
            "assigned_location" => self.delete_assigned_location(id).await,
            "negative_keyword" => self.delete_negative_keyword(id).await,
            "inventory_source" => self.delete_inventory_source(id).await,
            "assigned_inventory_source" => self.delete_assigned_inventory_source(id).await,
            "first_party_and_partner_audience" => {
                self.delete_first_party_and_partner_audience(id).await
            }
            "insertion_order" => self.delete_insertion_order(id).await,
            "sdfdownloadtask" => self.delete_sdfdownloadtask(id).await,
            "ad_asset" => self.delete_ad_asset(id).await,
            "operation" => self.delete_operation(id).await,
            "google_audience" => self.delete_google_audience(id).await,
            "user" => self.delete_user(id).await,
            "guaranteed_order" => self.delete_guaranteed_order(id).await,
            "ad_group_ad" => self.delete_ad_group_ad(id).await,
            "partner" => self.delete_partner(id).await,
            "floodlight_activitie" => self.delete_floodlight_activitie(id).await,
            "creative" => self.delete_creative(id).await,
            "custom_list" => self.delete_custom_list(id).await,
            "combined_audience" => self.delete_combined_audience(id).await,
            "channel" => self.delete_channel(id).await,
            "asset" => self.delete_asset(id).await,
            "invoice" => self.delete_invoice(id).await,
            "rule" => self.delete_rule(id).await,
            "inventory_source_group" => self.delete_inventory_source_group(id).await,
            "location_list" => self.delete_location_list(id).await,
            "custom_bidding_algorithm" => self.delete_custom_bidding_algorithm(id).await,
            "site" => self.delete_site(id).await,
            "ad_group" => self.delete_ad_group(id).await,
            "targeting_option" => self.delete_targeting_option(id).await,
            "rule" => self.delete_rule(id).await,
            "guaranteed_order" => self.delete_guaranteed_order(id).await,
            "script" => self.delete_script(id).await,
            "invoice" => self.delete_invoice(id).await,
            "negative_keyword_list" => self.delete_negative_keyword_list(id).await,
            "assigned_inventory_source" => self.delete_assigned_inventory_source(id).await,
            "insertion_order" => self.delete_insertion_order(id).await,
            "negative_keyword" => self.delete_negative_keyword(id).await,
            "creative" => self.delete_creative(id).await,
            "custom_list" => self.delete_custom_list(id).await,
            "sdfdownloadtask" => self.delete_sdfdownloadtask(id).await,
            "custom_bidding_algorithm" => self.delete_custom_bidding_algorithm(id).await,
            "assigned_targeting_option" => self.delete_assigned_targeting_option(id).await,
            "line_item" => self.delete_line_item(id).await,
            "floodlight_activitie" => self.delete_floodlight_activitie(id).await,
            "campaign" => self.delete_campaign(id).await,
            "inventory_source" => self.delete_inventory_source(id).await,
            "partner" => self.delete_partner(id).await,
            "first_and_third_party_audience" => {
                self.delete_first_and_third_party_audience(id).await
            }
            "channel" => self.delete_channel(id).await,
            "targeting_option" => self.delete_targeting_option(id).await,
            "location_list" => self.delete_location_list(id).await,
            "assigned_location" => self.delete_assigned_location(id).await,
            "ad_group" => self.delete_ad_group(id).await,
            "ad_group_ad" => self.delete_ad_group_ad(id).await,
            "user" => self.delete_user(id).await,
            "asset" => self.delete_asset(id).await,
            "advertiser" => self.delete_advertiser(id).await,
            "operation" => self.delete_operation(id).await,
            "floodlight_group" => self.delete_floodlight_group(id).await,
            "combined_audience" => self.delete_combined_audience(id).await,
            "google_audience" => self.delete_google_audience(id).await,
            "site" => self.delete_site(id).await,
            "inventory_source_group" => self.delete_inventory_source_group(id).await,
            "media" => self.delete_media(id).await,
            "media" => self.delete_media(id).await,
            "operation" => self.delete_operation(id).await,
            _ => Err(hemmer_core::HemmerError::Provider(format!(
                "Unknown resource type: {}.{}",
                "displayvideo_api", resource_name
            ))),
        }
    }

    // ========================================================================
    // Resource-specific CRUD implementations
    // ========================================================================

    // ------------------------------------------------------------------------
    // Inventory_source_group resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a inventory_source_group resource
    async fn plan_inventory_source_group(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new inventory_source_group resource
    async fn create_inventory_source_group(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a inventory_source_group resource
    async fn read_inventory_source_group(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a inventory_source_group resource
    async fn update_inventory_source_group(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a inventory_source_group resource
    async fn delete_inventory_source_group(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // User resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a user resource
    async fn plan_user(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new user resource
    async fn create_user(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a user resource
    async fn read_user(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a user resource
    async fn update_user(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a user resource
    async fn delete_user(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Operation resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a operation resource
    async fn plan_operation(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new operation resource
    async fn create_operation(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a operation resource
    async fn read_operation(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a operation resource
    async fn update_operation(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a operation resource
    async fn delete_operation(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Asset resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a asset resource
    async fn plan_asset(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new asset resource
    async fn create_asset(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a asset resource
    async fn read_asset(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a asset resource
    async fn update_asset(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a asset resource
    async fn delete_asset(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Media resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a media resource
    async fn plan_media(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new media resource
    async fn create_media(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a media resource
    async fn read_media(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a media resource
    async fn update_media(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a media resource
    async fn delete_media(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Custom_bidding_algorithm resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a custom_bidding_algorithm resource
    async fn plan_custom_bidding_algorithm(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new custom_bidding_algorithm resource
    async fn create_custom_bidding_algorithm(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a custom_bidding_algorithm resource
    async fn read_custom_bidding_algorithm(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a custom_bidding_algorithm resource
    async fn update_custom_bidding_algorithm(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a custom_bidding_algorithm resource
    async fn delete_custom_bidding_algorithm(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Targeting_option resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a targeting_option resource
    async fn plan_targeting_option(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new targeting_option resource
    async fn create_targeting_option(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a targeting_option resource
    async fn read_targeting_option(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a targeting_option resource
    async fn update_targeting_option(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a targeting_option resource
    async fn delete_targeting_option(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Guaranteed_order resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a guaranteed_order resource
    async fn plan_guaranteed_order(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new guaranteed_order resource
    async fn create_guaranteed_order(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a guaranteed_order resource
    async fn read_guaranteed_order(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a guaranteed_order resource
    async fn update_guaranteed_order(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a guaranteed_order resource
    async fn delete_guaranteed_order(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Campaign resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a campaign resource
    async fn plan_campaign(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new campaign resource
    async fn create_campaign(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a campaign resource
    async fn read_campaign(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a campaign resource
    async fn update_campaign(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a campaign resource
    async fn delete_campaign(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Negative_keyword_list resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a negative_keyword_list resource
    async fn plan_negative_keyword_list(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new negative_keyword_list resource
    async fn create_negative_keyword_list(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a negative_keyword_list resource
    async fn read_negative_keyword_list(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a negative_keyword_list resource
    async fn update_negative_keyword_list(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a negative_keyword_list resource
    async fn delete_negative_keyword_list(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Partner resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a partner resource
    async fn plan_partner(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new partner resource
    async fn create_partner(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a partner resource
    async fn read_partner(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a partner resource
    async fn update_partner(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a partner resource
    async fn delete_partner(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Assigned_inventory_source resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a assigned_inventory_source resource
    async fn plan_assigned_inventory_source(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new assigned_inventory_source resource
    async fn create_assigned_inventory_source(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a assigned_inventory_source resource
    async fn read_assigned_inventory_source(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a assigned_inventory_source resource
    async fn update_assigned_inventory_source(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a assigned_inventory_source resource
    async fn delete_assigned_inventory_source(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Combined_audience resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a combined_audience resource
    async fn plan_combined_audience(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new combined_audience resource
    async fn create_combined_audience(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a combined_audience resource
    async fn read_combined_audience(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a combined_audience resource
    async fn update_combined_audience(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a combined_audience resource
    async fn delete_combined_audience(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Creative resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a creative resource
    async fn plan_creative(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new creative resource
    async fn create_creative(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a creative resource
    async fn read_creative(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a creative resource
    async fn update_creative(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a creative resource
    async fn delete_creative(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Sdfdownloadtask resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a sdfdownloadtask resource
    async fn plan_sdfdownloadtask(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new sdfdownloadtask resource
    async fn create_sdfdownloadtask(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a sdfdownloadtask resource
    async fn read_sdfdownloadtask(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a sdfdownloadtask resource
    async fn update_sdfdownloadtask(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a sdfdownloadtask resource
    async fn delete_sdfdownloadtask(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Site resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a site resource
    async fn plan_site(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new site resource
    async fn create_site(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a site resource
    async fn read_site(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a site resource
    async fn update_site(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a site resource
    async fn delete_site(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // First_and_third_party_audience resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a first_and_third_party_audience resource
    async fn plan_first_and_third_party_audience(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new first_and_third_party_audience resource
    async fn create_first_and_third_party_audience(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a first_and_third_party_audience resource
    async fn read_first_and_third_party_audience(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a first_and_third_party_audience resource
    async fn update_first_and_third_party_audience(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a first_and_third_party_audience resource
    async fn delete_first_and_third_party_audience(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Insertion_order resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a insertion_order resource
    async fn plan_insertion_order(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new insertion_order resource
    async fn create_insertion_order(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a insertion_order resource
    async fn read_insertion_order(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a insertion_order resource
    async fn update_insertion_order(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a insertion_order resource
    async fn delete_insertion_order(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Invoice resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a invoice resource
    async fn plan_invoice(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new invoice resource
    async fn create_invoice(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a invoice resource
    async fn read_invoice(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a invoice resource
    async fn update_invoice(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a invoice resource
    async fn delete_invoice(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Negative_keyword resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a negative_keyword resource
    async fn plan_negative_keyword(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new negative_keyword resource
    async fn create_negative_keyword(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a negative_keyword resource
    async fn read_negative_keyword(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a negative_keyword resource
    async fn update_negative_keyword(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a negative_keyword resource
    async fn delete_negative_keyword(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Advertiser resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a advertiser resource
    async fn plan_advertiser(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new advertiser resource
    async fn create_advertiser(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a advertiser resource
    async fn read_advertiser(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a advertiser resource
    async fn update_advertiser(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a advertiser resource
    async fn delete_advertiser(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Google_audience resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a google_audience resource
    async fn plan_google_audience(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new google_audience resource
    async fn create_google_audience(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a google_audience resource
    async fn read_google_audience(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a google_audience resource
    async fn update_google_audience(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a google_audience resource
    async fn delete_google_audience(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Assigned_targeting_option resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a assigned_targeting_option resource
    async fn plan_assigned_targeting_option(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new assigned_targeting_option resource
    async fn create_assigned_targeting_option(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a assigned_targeting_option resource
    async fn read_assigned_targeting_option(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a assigned_targeting_option resource
    async fn update_assigned_targeting_option(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a assigned_targeting_option resource
    async fn delete_assigned_targeting_option(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Manual_trigger resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a manual_trigger resource
    async fn plan_manual_trigger(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new manual_trigger resource
    async fn create_manual_trigger(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a manual_trigger resource
    async fn read_manual_trigger(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a manual_trigger resource
    async fn update_manual_trigger(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a manual_trigger resource
    async fn delete_manual_trigger(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Channel resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a channel resource
    async fn plan_channel(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new channel resource
    async fn create_channel(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a channel resource
    async fn read_channel(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a channel resource
    async fn update_channel(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a channel resource
    async fn delete_channel(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Custom_list resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a custom_list resource
    async fn plan_custom_list(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new custom_list resource
    async fn create_custom_list(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a custom_list resource
    async fn read_custom_list(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a custom_list resource
    async fn update_custom_list(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a custom_list resource
    async fn delete_custom_list(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Assigned_location resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a assigned_location resource
    async fn plan_assigned_location(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new assigned_location resource
    async fn create_assigned_location(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a assigned_location resource
    async fn read_assigned_location(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a assigned_location resource
    async fn update_assigned_location(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a assigned_location resource
    async fn delete_assigned_location(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Line_item resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a line_item resource
    async fn plan_line_item(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new line_item resource
    async fn create_line_item(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a line_item resource
    async fn read_line_item(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a line_item resource
    async fn update_line_item(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a line_item resource
    async fn delete_line_item(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Script resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a script resource
    async fn plan_script(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new script resource
    async fn create_script(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a script resource
    async fn read_script(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a script resource
    async fn update_script(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a script resource
    async fn delete_script(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Location_list resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a location_list resource
    async fn plan_location_list(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new location_list resource
    async fn create_location_list(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a location_list resource
    async fn read_location_list(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a location_list resource
    async fn update_location_list(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a location_list resource
    async fn delete_location_list(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Floodlight_group resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a floodlight_group resource
    async fn plan_floodlight_group(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new floodlight_group resource
    async fn create_floodlight_group(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a floodlight_group resource
    async fn read_floodlight_group(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a floodlight_group resource
    async fn update_floodlight_group(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a floodlight_group resource
    async fn delete_floodlight_group(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Inventory_source resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a inventory_source resource
    async fn plan_inventory_source(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new inventory_source resource
    async fn create_inventory_source(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a inventory_source resource
    async fn read_inventory_source(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a inventory_source resource
    async fn update_inventory_source(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a inventory_source resource
    async fn delete_inventory_source(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Media resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a media resource
    async fn plan_media(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new media resource
    async fn create_media(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a media resource
    async fn read_media(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a media resource
    async fn update_media(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a media resource
    async fn delete_media(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Operation resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a operation resource
    async fn plan_operation(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new operation resource
    async fn create_operation(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a operation resource
    async fn read_operation(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a operation resource
    async fn update_operation(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a operation resource
    async fn delete_operation(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Youtube_ad_group_ad resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a youtube_ad_group_ad resource
    async fn plan_youtube_ad_group_ad(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new youtube_ad_group_ad resource
    async fn create_youtube_ad_group_ad(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a youtube_ad_group_ad resource
    async fn read_youtube_ad_group_ad(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a youtube_ad_group_ad resource
    async fn update_youtube_ad_group_ad(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a youtube_ad_group_ad resource
    async fn delete_youtube_ad_group_ad(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Assigned_inventory_source resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a assigned_inventory_source resource
    async fn plan_assigned_inventory_source(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new assigned_inventory_source resource
    async fn create_assigned_inventory_source(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a assigned_inventory_source resource
    async fn read_assigned_inventory_source(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a assigned_inventory_source resource
    async fn update_assigned_inventory_source(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a assigned_inventory_source resource
    async fn delete_assigned_inventory_source(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Inventory_source resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a inventory_source resource
    async fn plan_inventory_source(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new inventory_source resource
    async fn create_inventory_source(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a inventory_source resource
    async fn read_inventory_source(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a inventory_source resource
    async fn update_inventory_source(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a inventory_source resource
    async fn delete_inventory_source(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Negative_keyword_list resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a negative_keyword_list resource
    async fn plan_negative_keyword_list(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new negative_keyword_list resource
    async fn create_negative_keyword_list(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a negative_keyword_list resource
    async fn read_negative_keyword_list(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a negative_keyword_list resource
    async fn update_negative_keyword_list(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a negative_keyword_list resource
    async fn delete_negative_keyword_list(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Media resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a media resource
    async fn plan_media(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new media resource
    async fn create_media(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a media resource
    async fn read_media(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a media resource
    async fn update_media(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a media resource
    async fn delete_media(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Partner resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a partner resource
    async fn plan_partner(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new partner resource
    async fn create_partner(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a partner resource
    async fn read_partner(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a partner resource
    async fn update_partner(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a partner resource
    async fn delete_partner(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Line_item resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a line_item resource
    async fn plan_line_item(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new line_item resource
    async fn create_line_item(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a line_item resource
    async fn read_line_item(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a line_item resource
    async fn update_line_item(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a line_item resource
    async fn delete_line_item(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Advertiser resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a advertiser resource
    async fn plan_advertiser(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new advertiser resource
    async fn create_advertiser(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a advertiser resource
    async fn read_advertiser(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a advertiser resource
    async fn update_advertiser(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a advertiser resource
    async fn delete_advertiser(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Youtube_ad_group resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a youtube_ad_group resource
    async fn plan_youtube_ad_group(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new youtube_ad_group resource
    async fn create_youtube_ad_group(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a youtube_ad_group resource
    async fn read_youtube_ad_group(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a youtube_ad_group resource
    async fn update_youtube_ad_group(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a youtube_ad_group resource
    async fn delete_youtube_ad_group(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Negative_keyword resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a negative_keyword resource
    async fn plan_negative_keyword(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new negative_keyword resource
    async fn create_negative_keyword(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a negative_keyword resource
    async fn read_negative_keyword(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a negative_keyword resource
    async fn update_negative_keyword(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a negative_keyword resource
    async fn delete_negative_keyword(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Assigned_location resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a assigned_location resource
    async fn plan_assigned_location(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new assigned_location resource
    async fn create_assigned_location(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a assigned_location resource
    async fn read_assigned_location(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a assigned_location resource
    async fn update_assigned_location(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a assigned_location resource
    async fn delete_assigned_location(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // User resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a user resource
    async fn plan_user(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new user resource
    async fn create_user(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a user resource
    async fn read_user(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a user resource
    async fn update_user(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a user resource
    async fn delete_user(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Sdfdownloadtask resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a sdfdownloadtask resource
    async fn plan_sdfdownloadtask(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new sdfdownloadtask resource
    async fn create_sdfdownloadtask(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a sdfdownloadtask resource
    async fn read_sdfdownloadtask(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a sdfdownloadtask resource
    async fn update_sdfdownloadtask(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a sdfdownloadtask resource
    async fn delete_sdfdownloadtask(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Google_audience resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a google_audience resource
    async fn plan_google_audience(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new google_audience resource
    async fn create_google_audience(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a google_audience resource
    async fn read_google_audience(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a google_audience resource
    async fn update_google_audience(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a google_audience resource
    async fn delete_google_audience(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Campaign resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a campaign resource
    async fn plan_campaign(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new campaign resource
    async fn create_campaign(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a campaign resource
    async fn read_campaign(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a campaign resource
    async fn update_campaign(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a campaign resource
    async fn delete_campaign(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Site resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a site resource
    async fn plan_site(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new site resource
    async fn create_site(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a site resource
    async fn read_site(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a site resource
    async fn update_site(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a site resource
    async fn delete_site(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Floodlight_activitie resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a floodlight_activitie resource
    async fn plan_floodlight_activitie(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new floodlight_activitie resource
    async fn create_floodlight_activitie(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a floodlight_activitie resource
    async fn read_floodlight_activitie(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a floodlight_activitie resource
    async fn update_floodlight_activitie(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a floodlight_activitie resource
    async fn delete_floodlight_activitie(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Combined_audience resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a combined_audience resource
    async fn plan_combined_audience(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new combined_audience resource
    async fn create_combined_audience(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a combined_audience resource
    async fn read_combined_audience(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a combined_audience resource
    async fn update_combined_audience(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a combined_audience resource
    async fn delete_combined_audience(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Targeting_option resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a targeting_option resource
    async fn plan_targeting_option(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new targeting_option resource
    async fn create_targeting_option(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a targeting_option resource
    async fn read_targeting_option(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a targeting_option resource
    async fn update_targeting_option(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a targeting_option resource
    async fn delete_targeting_option(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Custom_list resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a custom_list resource
    async fn plan_custom_list(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new custom_list resource
    async fn create_custom_list(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a custom_list resource
    async fn read_custom_list(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a custom_list resource
    async fn update_custom_list(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a custom_list resource
    async fn delete_custom_list(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Creative resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a creative resource
    async fn plan_creative(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new creative resource
    async fn create_creative(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a creative resource
    async fn read_creative(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a creative resource
    async fn update_creative(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a creative resource
    async fn delete_creative(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Channel resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a channel resource
    async fn plan_channel(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new channel resource
    async fn create_channel(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a channel resource
    async fn read_channel(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a channel resource
    async fn update_channel(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a channel resource
    async fn delete_channel(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Asset resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a asset resource
    async fn plan_asset(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new asset resource
    async fn create_asset(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a asset resource
    async fn read_asset(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a asset resource
    async fn update_asset(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a asset resource
    async fn delete_asset(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Operation resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a operation resource
    async fn plan_operation(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new operation resource
    async fn create_operation(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a operation resource
    async fn read_operation(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a operation resource
    async fn update_operation(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a operation resource
    async fn delete_operation(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Location_list resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a location_list resource
    async fn plan_location_list(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new location_list resource
    async fn create_location_list(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a location_list resource
    async fn read_location_list(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a location_list resource
    async fn update_location_list(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a location_list resource
    async fn delete_location_list(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Floodlight_group resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a floodlight_group resource
    async fn plan_floodlight_group(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new floodlight_group resource
    async fn create_floodlight_group(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a floodlight_group resource
    async fn read_floodlight_group(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a floodlight_group resource
    async fn update_floodlight_group(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a floodlight_group resource
    async fn delete_floodlight_group(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Custom_bidding_algorithm resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a custom_bidding_algorithm resource
    async fn plan_custom_bidding_algorithm(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new custom_bidding_algorithm resource
    async fn create_custom_bidding_algorithm(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a custom_bidding_algorithm resource
    async fn read_custom_bidding_algorithm(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a custom_bidding_algorithm resource
    async fn update_custom_bidding_algorithm(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a custom_bidding_algorithm resource
    async fn delete_custom_bidding_algorithm(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Guaranteed_order resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a guaranteed_order resource
    async fn plan_guaranteed_order(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new guaranteed_order resource
    async fn create_guaranteed_order(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a guaranteed_order resource
    async fn read_guaranteed_order(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a guaranteed_order resource
    async fn update_guaranteed_order(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a guaranteed_order resource
    async fn delete_guaranteed_order(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Script resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a script resource
    async fn plan_script(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new script resource
    async fn create_script(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a script resource
    async fn read_script(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a script resource
    async fn update_script(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a script resource
    async fn delete_script(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Insertion_order resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a insertion_order resource
    async fn plan_insertion_order(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new insertion_order resource
    async fn create_insertion_order(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a insertion_order resource
    async fn read_insertion_order(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a insertion_order resource
    async fn update_insertion_order(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a insertion_order resource
    async fn delete_insertion_order(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Manual_trigger resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a manual_trigger resource
    async fn plan_manual_trigger(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new manual_trigger resource
    async fn create_manual_trigger(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a manual_trigger resource
    async fn read_manual_trigger(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a manual_trigger resource
    async fn update_manual_trigger(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a manual_trigger resource
    async fn delete_manual_trigger(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Inventory_source_group resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a inventory_source_group resource
    async fn plan_inventory_source_group(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new inventory_source_group resource
    async fn create_inventory_source_group(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a inventory_source_group resource
    async fn read_inventory_source_group(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a inventory_source_group resource
    async fn update_inventory_source_group(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a inventory_source_group resource
    async fn delete_inventory_source_group(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Assigned_targeting_option resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a assigned_targeting_option resource
    async fn plan_assigned_targeting_option(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new assigned_targeting_option resource
    async fn create_assigned_targeting_option(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a assigned_targeting_option resource
    async fn read_assigned_targeting_option(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a assigned_targeting_option resource
    async fn update_assigned_targeting_option(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a assigned_targeting_option resource
    async fn delete_assigned_targeting_option(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Invoice resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a invoice resource
    async fn plan_invoice(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new invoice resource
    async fn create_invoice(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a invoice resource
    async fn read_invoice(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a invoice resource
    async fn update_invoice(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a invoice resource
    async fn delete_invoice(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Media resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a media resource
    async fn plan_media(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new media resource
    async fn create_media(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a media resource
    async fn read_media(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a media resource
    async fn update_media(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a media resource
    async fn delete_media(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Operation resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a operation resource
    async fn plan_operation(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new operation resource
    async fn create_operation(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a operation resource
    async fn read_operation(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a operation resource
    async fn update_operation(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a operation resource
    async fn delete_operation(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Campaign resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a campaign resource
    async fn plan_campaign(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new campaign resource
    async fn create_campaign(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a campaign resource
    async fn read_campaign(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a campaign resource
    async fn update_campaign(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a campaign resource
    async fn delete_campaign(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Advertiser resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a advertiser resource
    async fn plan_advertiser(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new advertiser resource
    async fn create_advertiser(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a advertiser resource
    async fn read_advertiser(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a advertiser resource
    async fn update_advertiser(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a advertiser resource
    async fn delete_advertiser(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Negative_keyword_list resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a negative_keyword_list resource
    async fn plan_negative_keyword_list(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new negative_keyword_list resource
    async fn create_negative_keyword_list(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a negative_keyword_list resource
    async fn read_negative_keyword_list(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a negative_keyword_list resource
    async fn update_negative_keyword_list(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a negative_keyword_list resource
    async fn delete_negative_keyword_list(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Line_item resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a line_item resource
    async fn plan_line_item(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new line_item resource
    async fn create_line_item(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a line_item resource
    async fn read_line_item(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a line_item resource
    async fn update_line_item(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a line_item resource
    async fn delete_line_item(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Assigned_targeting_option resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a assigned_targeting_option resource
    async fn plan_assigned_targeting_option(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new assigned_targeting_option resource
    async fn create_assigned_targeting_option(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a assigned_targeting_option resource
    async fn read_assigned_targeting_option(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a assigned_targeting_option resource
    async fn update_assigned_targeting_option(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a assigned_targeting_option resource
    async fn delete_assigned_targeting_option(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Script resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a script resource
    async fn plan_script(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new script resource
    async fn create_script(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a script resource
    async fn read_script(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a script resource
    async fn update_script(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a script resource
    async fn delete_script(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Youtube_asset_association resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a youtube_asset_association resource
    async fn plan_youtube_asset_association(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new youtube_asset_association resource
    async fn create_youtube_asset_association(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a youtube_asset_association resource
    async fn read_youtube_asset_association(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a youtube_asset_association resource
    async fn update_youtube_asset_association(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a youtube_asset_association resource
    async fn delete_youtube_asset_association(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Media resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a media resource
    async fn plan_media(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new media resource
    async fn create_media(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a media resource
    async fn read_media(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a media resource
    async fn update_media(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a media resource
    async fn delete_media(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Floodlight_group resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a floodlight_group resource
    async fn plan_floodlight_group(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new floodlight_group resource
    async fn create_floodlight_group(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a floodlight_group resource
    async fn read_floodlight_group(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a floodlight_group resource
    async fn update_floodlight_group(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a floodlight_group resource
    async fn delete_floodlight_group(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Assigned_location resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a assigned_location resource
    async fn plan_assigned_location(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new assigned_location resource
    async fn create_assigned_location(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a assigned_location resource
    async fn read_assigned_location(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a assigned_location resource
    async fn update_assigned_location(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a assigned_location resource
    async fn delete_assigned_location(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Negative_keyword resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a negative_keyword resource
    async fn plan_negative_keyword(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new negative_keyword resource
    async fn create_negative_keyword(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a negative_keyword resource
    async fn read_negative_keyword(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a negative_keyword resource
    async fn update_negative_keyword(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a negative_keyword resource
    async fn delete_negative_keyword(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Inventory_source resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a inventory_source resource
    async fn plan_inventory_source(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new inventory_source resource
    async fn create_inventory_source(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a inventory_source resource
    async fn read_inventory_source(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a inventory_source resource
    async fn update_inventory_source(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a inventory_source resource
    async fn delete_inventory_source(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Assigned_inventory_source resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a assigned_inventory_source resource
    async fn plan_assigned_inventory_source(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new assigned_inventory_source resource
    async fn create_assigned_inventory_source(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a assigned_inventory_source resource
    async fn read_assigned_inventory_source(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a assigned_inventory_source resource
    async fn update_assigned_inventory_source(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a assigned_inventory_source resource
    async fn delete_assigned_inventory_source(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // First_party_and_partner_audience resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a first_party_and_partner_audience resource
    async fn plan_first_party_and_partner_audience(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new first_party_and_partner_audience resource
    async fn create_first_party_and_partner_audience(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a first_party_and_partner_audience resource
    async fn read_first_party_and_partner_audience(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a first_party_and_partner_audience resource
    async fn update_first_party_and_partner_audience(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a first_party_and_partner_audience resource
    async fn delete_first_party_and_partner_audience(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Insertion_order resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a insertion_order resource
    async fn plan_insertion_order(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new insertion_order resource
    async fn create_insertion_order(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a insertion_order resource
    async fn read_insertion_order(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a insertion_order resource
    async fn update_insertion_order(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a insertion_order resource
    async fn delete_insertion_order(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Sdfdownloadtask resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a sdfdownloadtask resource
    async fn plan_sdfdownloadtask(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new sdfdownloadtask resource
    async fn create_sdfdownloadtask(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a sdfdownloadtask resource
    async fn read_sdfdownloadtask(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a sdfdownloadtask resource
    async fn update_sdfdownloadtask(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a sdfdownloadtask resource
    async fn delete_sdfdownloadtask(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Ad_asset resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a ad_asset resource
    async fn plan_ad_asset(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new ad_asset resource
    async fn create_ad_asset(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a ad_asset resource
    async fn read_ad_asset(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a ad_asset resource
    async fn update_ad_asset(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a ad_asset resource
    async fn delete_ad_asset(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Operation resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a operation resource
    async fn plan_operation(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new operation resource
    async fn create_operation(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a operation resource
    async fn read_operation(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a operation resource
    async fn update_operation(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a operation resource
    async fn delete_operation(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Google_audience resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a google_audience resource
    async fn plan_google_audience(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new google_audience resource
    async fn create_google_audience(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a google_audience resource
    async fn read_google_audience(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a google_audience resource
    async fn update_google_audience(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a google_audience resource
    async fn delete_google_audience(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // User resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a user resource
    async fn plan_user(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new user resource
    async fn create_user(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a user resource
    async fn read_user(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a user resource
    async fn update_user(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a user resource
    async fn delete_user(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Guaranteed_order resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a guaranteed_order resource
    async fn plan_guaranteed_order(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new guaranteed_order resource
    async fn create_guaranteed_order(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a guaranteed_order resource
    async fn read_guaranteed_order(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a guaranteed_order resource
    async fn update_guaranteed_order(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a guaranteed_order resource
    async fn delete_guaranteed_order(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Ad_group_ad resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a ad_group_ad resource
    async fn plan_ad_group_ad(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new ad_group_ad resource
    async fn create_ad_group_ad(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a ad_group_ad resource
    async fn read_ad_group_ad(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a ad_group_ad resource
    async fn update_ad_group_ad(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a ad_group_ad resource
    async fn delete_ad_group_ad(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Partner resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a partner resource
    async fn plan_partner(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new partner resource
    async fn create_partner(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a partner resource
    async fn read_partner(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a partner resource
    async fn update_partner(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a partner resource
    async fn delete_partner(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Floodlight_activitie resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a floodlight_activitie resource
    async fn plan_floodlight_activitie(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new floodlight_activitie resource
    async fn create_floodlight_activitie(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a floodlight_activitie resource
    async fn read_floodlight_activitie(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a floodlight_activitie resource
    async fn update_floodlight_activitie(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a floodlight_activitie resource
    async fn delete_floodlight_activitie(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Creative resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a creative resource
    async fn plan_creative(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new creative resource
    async fn create_creative(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a creative resource
    async fn read_creative(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a creative resource
    async fn update_creative(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a creative resource
    async fn delete_creative(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Custom_list resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a custom_list resource
    async fn plan_custom_list(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new custom_list resource
    async fn create_custom_list(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a custom_list resource
    async fn read_custom_list(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a custom_list resource
    async fn update_custom_list(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a custom_list resource
    async fn delete_custom_list(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Combined_audience resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a combined_audience resource
    async fn plan_combined_audience(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new combined_audience resource
    async fn create_combined_audience(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a combined_audience resource
    async fn read_combined_audience(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a combined_audience resource
    async fn update_combined_audience(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a combined_audience resource
    async fn delete_combined_audience(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Channel resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a channel resource
    async fn plan_channel(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new channel resource
    async fn create_channel(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a channel resource
    async fn read_channel(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a channel resource
    async fn update_channel(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a channel resource
    async fn delete_channel(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Asset resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a asset resource
    async fn plan_asset(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new asset resource
    async fn create_asset(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a asset resource
    async fn read_asset(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a asset resource
    async fn update_asset(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a asset resource
    async fn delete_asset(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Invoice resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a invoice resource
    async fn plan_invoice(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new invoice resource
    async fn create_invoice(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a invoice resource
    async fn read_invoice(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a invoice resource
    async fn update_invoice(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a invoice resource
    async fn delete_invoice(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Rule resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a rule resource
    async fn plan_rule(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new rule resource
    async fn create_rule(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a rule resource
    async fn read_rule(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a rule resource
    async fn update_rule(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a rule resource
    async fn delete_rule(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Inventory_source_group resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a inventory_source_group resource
    async fn plan_inventory_source_group(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new inventory_source_group resource
    async fn create_inventory_source_group(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a inventory_source_group resource
    async fn read_inventory_source_group(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a inventory_source_group resource
    async fn update_inventory_source_group(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a inventory_source_group resource
    async fn delete_inventory_source_group(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Location_list resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a location_list resource
    async fn plan_location_list(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new location_list resource
    async fn create_location_list(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a location_list resource
    async fn read_location_list(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a location_list resource
    async fn update_location_list(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a location_list resource
    async fn delete_location_list(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Custom_bidding_algorithm resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a custom_bidding_algorithm resource
    async fn plan_custom_bidding_algorithm(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new custom_bidding_algorithm resource
    async fn create_custom_bidding_algorithm(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a custom_bidding_algorithm resource
    async fn read_custom_bidding_algorithm(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a custom_bidding_algorithm resource
    async fn update_custom_bidding_algorithm(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a custom_bidding_algorithm resource
    async fn delete_custom_bidding_algorithm(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Site resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a site resource
    async fn plan_site(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new site resource
    async fn create_site(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a site resource
    async fn read_site(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a site resource
    async fn update_site(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a site resource
    async fn delete_site(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Ad_group resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a ad_group resource
    async fn plan_ad_group(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new ad_group resource
    async fn create_ad_group(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a ad_group resource
    async fn read_ad_group(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a ad_group resource
    async fn update_ad_group(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a ad_group resource
    async fn delete_ad_group(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Targeting_option resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a targeting_option resource
    async fn plan_targeting_option(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new targeting_option resource
    async fn create_targeting_option(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a targeting_option resource
    async fn read_targeting_option(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a targeting_option resource
    async fn update_targeting_option(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a targeting_option resource
    async fn delete_targeting_option(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Rule resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a rule resource
    async fn plan_rule(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new rule resource
    async fn create_rule(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a rule resource
    async fn read_rule(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a rule resource
    async fn update_rule(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a rule resource
    async fn delete_rule(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Guaranteed_order resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a guaranteed_order resource
    async fn plan_guaranteed_order(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new guaranteed_order resource
    async fn create_guaranteed_order(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a guaranteed_order resource
    async fn read_guaranteed_order(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a guaranteed_order resource
    async fn update_guaranteed_order(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a guaranteed_order resource
    async fn delete_guaranteed_order(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Script resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a script resource
    async fn plan_script(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new script resource
    async fn create_script(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a script resource
    async fn read_script(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a script resource
    async fn update_script(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a script resource
    async fn delete_script(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Invoice resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a invoice resource
    async fn plan_invoice(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new invoice resource
    async fn create_invoice(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a invoice resource
    async fn read_invoice(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a invoice resource
    async fn update_invoice(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a invoice resource
    async fn delete_invoice(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Negative_keyword_list resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a negative_keyword_list resource
    async fn plan_negative_keyword_list(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new negative_keyword_list resource
    async fn create_negative_keyword_list(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a negative_keyword_list resource
    async fn read_negative_keyword_list(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a negative_keyword_list resource
    async fn update_negative_keyword_list(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a negative_keyword_list resource
    async fn delete_negative_keyword_list(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Assigned_inventory_source resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a assigned_inventory_source resource
    async fn plan_assigned_inventory_source(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new assigned_inventory_source resource
    async fn create_assigned_inventory_source(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a assigned_inventory_source resource
    async fn read_assigned_inventory_source(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a assigned_inventory_source resource
    async fn update_assigned_inventory_source(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a assigned_inventory_source resource
    async fn delete_assigned_inventory_source(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Insertion_order resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a insertion_order resource
    async fn plan_insertion_order(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new insertion_order resource
    async fn create_insertion_order(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a insertion_order resource
    async fn read_insertion_order(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a insertion_order resource
    async fn update_insertion_order(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a insertion_order resource
    async fn delete_insertion_order(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Negative_keyword resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a negative_keyword resource
    async fn plan_negative_keyword(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new negative_keyword resource
    async fn create_negative_keyword(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a negative_keyword resource
    async fn read_negative_keyword(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a negative_keyword resource
    async fn update_negative_keyword(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a negative_keyword resource
    async fn delete_negative_keyword(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Creative resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a creative resource
    async fn plan_creative(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new creative resource
    async fn create_creative(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a creative resource
    async fn read_creative(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a creative resource
    async fn update_creative(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a creative resource
    async fn delete_creative(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Custom_list resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a custom_list resource
    async fn plan_custom_list(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new custom_list resource
    async fn create_custom_list(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a custom_list resource
    async fn read_custom_list(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a custom_list resource
    async fn update_custom_list(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a custom_list resource
    async fn delete_custom_list(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Sdfdownloadtask resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a sdfdownloadtask resource
    async fn plan_sdfdownloadtask(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new sdfdownloadtask resource
    async fn create_sdfdownloadtask(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a sdfdownloadtask resource
    async fn read_sdfdownloadtask(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a sdfdownloadtask resource
    async fn update_sdfdownloadtask(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a sdfdownloadtask resource
    async fn delete_sdfdownloadtask(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Custom_bidding_algorithm resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a custom_bidding_algorithm resource
    async fn plan_custom_bidding_algorithm(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new custom_bidding_algorithm resource
    async fn create_custom_bidding_algorithm(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a custom_bidding_algorithm resource
    async fn read_custom_bidding_algorithm(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a custom_bidding_algorithm resource
    async fn update_custom_bidding_algorithm(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a custom_bidding_algorithm resource
    async fn delete_custom_bidding_algorithm(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Assigned_targeting_option resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a assigned_targeting_option resource
    async fn plan_assigned_targeting_option(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new assigned_targeting_option resource
    async fn create_assigned_targeting_option(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a assigned_targeting_option resource
    async fn read_assigned_targeting_option(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a assigned_targeting_option resource
    async fn update_assigned_targeting_option(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a assigned_targeting_option resource
    async fn delete_assigned_targeting_option(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Line_item resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a line_item resource
    async fn plan_line_item(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new line_item resource
    async fn create_line_item(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a line_item resource
    async fn read_line_item(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a line_item resource
    async fn update_line_item(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a line_item resource
    async fn delete_line_item(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Floodlight_activitie resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a floodlight_activitie resource
    async fn plan_floodlight_activitie(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new floodlight_activitie resource
    async fn create_floodlight_activitie(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a floodlight_activitie resource
    async fn read_floodlight_activitie(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a floodlight_activitie resource
    async fn update_floodlight_activitie(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a floodlight_activitie resource
    async fn delete_floodlight_activitie(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Campaign resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a campaign resource
    async fn plan_campaign(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new campaign resource
    async fn create_campaign(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a campaign resource
    async fn read_campaign(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a campaign resource
    async fn update_campaign(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a campaign resource
    async fn delete_campaign(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Inventory_source resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a inventory_source resource
    async fn plan_inventory_source(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new inventory_source resource
    async fn create_inventory_source(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a inventory_source resource
    async fn read_inventory_source(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a inventory_source resource
    async fn update_inventory_source(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a inventory_source resource
    async fn delete_inventory_source(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Partner resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a partner resource
    async fn plan_partner(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new partner resource
    async fn create_partner(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a partner resource
    async fn read_partner(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a partner resource
    async fn update_partner(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a partner resource
    async fn delete_partner(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // First_and_third_party_audience resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a first_and_third_party_audience resource
    async fn plan_first_and_third_party_audience(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new first_and_third_party_audience resource
    async fn create_first_and_third_party_audience(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a first_and_third_party_audience resource
    async fn read_first_and_third_party_audience(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a first_and_third_party_audience resource
    async fn update_first_and_third_party_audience(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a first_and_third_party_audience resource
    async fn delete_first_and_third_party_audience(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Channel resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a channel resource
    async fn plan_channel(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new channel resource
    async fn create_channel(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a channel resource
    async fn read_channel(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a channel resource
    async fn update_channel(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a channel resource
    async fn delete_channel(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Targeting_option resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a targeting_option resource
    async fn plan_targeting_option(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new targeting_option resource
    async fn create_targeting_option(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a targeting_option resource
    async fn read_targeting_option(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a targeting_option resource
    async fn update_targeting_option(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a targeting_option resource
    async fn delete_targeting_option(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Location_list resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a location_list resource
    async fn plan_location_list(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new location_list resource
    async fn create_location_list(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a location_list resource
    async fn read_location_list(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a location_list resource
    async fn update_location_list(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a location_list resource
    async fn delete_location_list(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Assigned_location resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a assigned_location resource
    async fn plan_assigned_location(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new assigned_location resource
    async fn create_assigned_location(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a assigned_location resource
    async fn read_assigned_location(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a assigned_location resource
    async fn update_assigned_location(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a assigned_location resource
    async fn delete_assigned_location(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Ad_group resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a ad_group resource
    async fn plan_ad_group(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new ad_group resource
    async fn create_ad_group(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a ad_group resource
    async fn read_ad_group(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a ad_group resource
    async fn update_ad_group(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a ad_group resource
    async fn delete_ad_group(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Ad_group_ad resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a ad_group_ad resource
    async fn plan_ad_group_ad(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new ad_group_ad resource
    async fn create_ad_group_ad(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a ad_group_ad resource
    async fn read_ad_group_ad(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a ad_group_ad resource
    async fn update_ad_group_ad(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a ad_group_ad resource
    async fn delete_ad_group_ad(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // User resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a user resource
    async fn plan_user(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new user resource
    async fn create_user(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a user resource
    async fn read_user(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a user resource
    async fn update_user(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a user resource
    async fn delete_user(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Asset resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a asset resource
    async fn plan_asset(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new asset resource
    async fn create_asset(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a asset resource
    async fn read_asset(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a asset resource
    async fn update_asset(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a asset resource
    async fn delete_asset(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Advertiser resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a advertiser resource
    async fn plan_advertiser(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new advertiser resource
    async fn create_advertiser(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a advertiser resource
    async fn read_advertiser(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a advertiser resource
    async fn update_advertiser(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a advertiser resource
    async fn delete_advertiser(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Operation resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a operation resource
    async fn plan_operation(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new operation resource
    async fn create_operation(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a operation resource
    async fn read_operation(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a operation resource
    async fn update_operation(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a operation resource
    async fn delete_operation(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Floodlight_group resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a floodlight_group resource
    async fn plan_floodlight_group(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new floodlight_group resource
    async fn create_floodlight_group(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a floodlight_group resource
    async fn read_floodlight_group(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a floodlight_group resource
    async fn update_floodlight_group(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a floodlight_group resource
    async fn delete_floodlight_group(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Combined_audience resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a combined_audience resource
    async fn plan_combined_audience(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new combined_audience resource
    async fn create_combined_audience(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a combined_audience resource
    async fn read_combined_audience(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a combined_audience resource
    async fn update_combined_audience(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a combined_audience resource
    async fn delete_combined_audience(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Google_audience resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a google_audience resource
    async fn plan_google_audience(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new google_audience resource
    async fn create_google_audience(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a google_audience resource
    async fn read_google_audience(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a google_audience resource
    async fn update_google_audience(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a google_audience resource
    async fn delete_google_audience(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Site resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a site resource
    async fn plan_site(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new site resource
    async fn create_site(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a site resource
    async fn read_site(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a site resource
    async fn update_site(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a site resource
    async fn delete_site(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Inventory_source_group resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a inventory_source_group resource
    async fn plan_inventory_source_group(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new inventory_source_group resource
    async fn create_inventory_source_group(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a inventory_source_group resource
    async fn read_inventory_source_group(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a inventory_source_group resource
    async fn update_inventory_source_group(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a inventory_source_group resource
    async fn delete_inventory_source_group(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Media resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a media resource
    async fn plan_media(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new media resource
    async fn create_media(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a media resource
    async fn read_media(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a media resource
    async fn update_media(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a media resource
    async fn delete_media(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Media resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a media resource
    async fn plan_media(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new media resource
    async fn create_media(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a media resource
    async fn read_media(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a media resource
    async fn update_media(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a media resource
    async fn delete_media(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Operation resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a operation resource
    async fn plan_operation(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new operation resource
    async fn create_operation(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a operation resource
    async fn read_operation(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a operation resource
    async fn update_operation(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a operation resource
    async fn delete_operation(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }
}
