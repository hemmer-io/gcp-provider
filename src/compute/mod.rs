//! Compute Service
//!
//! Auto-generated service module for compute

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for compute
pub struct ComputeService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> ComputeService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get target_http_proxie resource handler
    pub fn target_http_proxie(&self) -> resources::Target_http_proxie<'_> {
        resources::Target_http_proxie::new(self.provider)
    }
    /// Get ssl_certificate resource handler
    pub fn ssl_certificate(&self) -> resources::Ssl_certificate<'_> {
        resources::Ssl_certificate::new(self.provider)
    }
    /// Get global_addresse resource handler
    pub fn global_addresse(&self) -> resources::Global_addresse<'_> {
        resources::Global_addresse::new(self.provider)
    }
    /// Get region_url_map resource handler
    pub fn region_url_map(&self) -> resources::Region_url_map<'_> {
        resources::Region_url_map::new(self.provider)
    }
    /// Get machine_image resource handler
    pub fn machine_image(&self) -> resources::Machine_image<'_> {
        resources::Machine_image::new(self.provider)
    }
    /// Get interconnect_remote_location resource handler
    pub fn interconnect_remote_location(&self) -> resources::Interconnect_remote_location<'_> {
        resources::Interconnect_remote_location::new(self.provider)
    }
    /// Get instant_snapshot resource handler
    pub fn instant_snapshot(&self) -> resources::Instant_snapshot<'_> {
        resources::Instant_snapshot::new(self.provider)
    }
    /// Get global_public_delegated_prefixe resource handler
    pub fn global_public_delegated_prefixe(&self) -> resources::Global_public_delegated_prefixe<'_> {
        resources::Global_public_delegated_prefixe::new(self.provider)
    }
    /// Get addresse resource handler
    pub fn addresse(&self) -> resources::Addresse<'_> {
        resources::Addresse::new(self.provider)
    }
    /// Get image resource handler
    pub fn image(&self) -> resources::Image<'_> {
        resources::Image::new(self.provider)
    }
    /// Get vpn_gateway resource handler
    pub fn vpn_gateway(&self) -> resources::Vpn_gateway<'_> {
        resources::Vpn_gateway::new(self.provider)
    }
    /// Get interconnect_location resource handler
    pub fn interconnect_location(&self) -> resources::Interconnect_location<'_> {
        resources::Interconnect_location::new(self.provider)
    }
    /// Get region_network_firewall_policie resource handler
    pub fn region_network_firewall_policie(&self) -> resources::Region_network_firewall_policie<'_> {
        resources::Region_network_firewall_policie::new(self.provider)
    }
    /// Get cross_site_network resource handler
    pub fn cross_site_network(&self) -> resources::Cross_site_network<'_> {
        resources::Cross_site_network::new(self.provider)
    }
    /// Get target_instance resource handler
    pub fn target_instance(&self) -> resources::Target_instance<'_> {
        resources::Target_instance::new(self.provider)
    }
    /// Get target_https_proxie resource handler
    pub fn target_https_proxie(&self) -> resources::Target_https_proxie<'_> {
        resources::Target_https_proxie::new(self.provider)
    }
    /// Get public_advertised_prefixe resource handler
    pub fn public_advertised_prefixe(&self) -> resources::Public_advertised_prefixe<'_> {
        resources::Public_advertised_prefixe::new(self.provider)
    }
    /// Get region resource handler
    pub fn region(&self) -> resources::Region<'_> {
        resources::Region::new(self.provider)
    }
    /// Get region_target_tcp_proxie resource handler
    pub fn region_target_tcp_proxie(&self) -> resources::Region_target_tcp_proxie<'_> {
        resources::Region_target_tcp_proxie::new(self.provider)
    }
    /// Get firewall resource handler
    pub fn firewall(&self) -> resources::Firewall<'_> {
        resources::Firewall::new(self.provider)
    }
    /// Get target_ssl_proxie resource handler
    pub fn target_ssl_proxie(&self) -> resources::Target_ssl_proxie<'_> {
        resources::Target_ssl_proxie::new(self.provider)
    }
    /// Get resource_policie resource handler
    pub fn resource_policie(&self) -> resources::Resource_policie<'_> {
        resources::Resource_policie::new(self.provider)
    }
    /// Get region_autoscaler resource handler
    pub fn region_autoscaler(&self) -> resources::Region_autoscaler<'_> {
        resources::Region_autoscaler::new(self.provider)
    }
    /// Get region_ssl_certificate resource handler
    pub fn region_ssl_certificate(&self) -> resources::Region_ssl_certificate<'_> {
        resources::Region_ssl_certificate::new(self.provider)
    }
    /// Get region_health_aggregation_policie resource handler
    pub fn region_health_aggregation_policie(&self) -> resources::Region_health_aggregation_policie<'_> {
        resources::Region_health_aggregation_policie::new(self.provider)
    }
    /// Get region_instance_group resource handler
    pub fn region_instance_group(&self) -> resources::Region_instance_group<'_> {
        resources::Region_instance_group::new(self.provider)
    }
    /// Get region_ssl_policie resource handler
    pub fn region_ssl_policie(&self) -> resources::Region_ssl_policie<'_> {
        resources::Region_ssl_policie::new(self.provider)
    }
    /// Get region_notification_endpoint resource handler
    pub fn region_notification_endpoint(&self) -> resources::Region_notification_endpoint<'_> {
        resources::Region_notification_endpoint::new(self.provider)
    }
    /// Get autoscaler resource handler
    pub fn autoscaler(&self) -> resources::Autoscaler<'_> {
        resources::Autoscaler::new(self.provider)
    }
    /// Get region_target_https_proxie resource handler
    pub fn region_target_https_proxie(&self) -> resources::Region_target_https_proxie<'_> {
        resources::Region_target_https_proxie::new(self.provider)
    }
    /// Get backend_service resource handler
    pub fn backend_service(&self) -> resources::Backend_service<'_> {
        resources::Backend_service::new(self.provider)
    }
    /// Get region_snapshot_setting resource handler
    pub fn region_snapshot_setting(&self) -> resources::Region_snapshot_setting<'_> {
        resources::Region_snapshot_setting::new(self.provider)
    }
    /// Get router resource handler
    pub fn router(&self) -> resources::Router<'_> {
        resources::Router::new(self.provider)
    }
    /// Get zone resource handler
    pub fn zone(&self) -> resources::Zone<'_> {
        resources::Zone::new(self.provider)
    }
    /// Get region_health_check_service resource handler
    pub fn region_health_check_service(&self) -> resources::Region_health_check_service<'_> {
        resources::Region_health_check_service::new(self.provider)
    }
    /// Get subnetwork resource handler
    pub fn subnetwork(&self) -> resources::Subnetwork<'_> {
        resources::Subnetwork::new(self.provider)
    }
    /// Get external_vpn_gateway resource handler
    pub fn external_vpn_gateway(&self) -> resources::External_vpn_gateway<'_> {
        resources::External_vpn_gateway::new(self.provider)
    }
    /// Get zone_vm_extension_policie resource handler
    pub fn zone_vm_extension_policie(&self) -> resources::Zone_vm_extension_policie<'_> {
        resources::Zone_vm_extension_policie::new(self.provider)
    }
    /// Get ssl_policie resource handler
    pub fn ssl_policie(&self) -> resources::Ssl_policie<'_> {
        resources::Ssl_policie::new(self.provider)
    }
    /// Get node_type resource handler
    pub fn node_type(&self) -> resources::Node_type<'_> {
        resources::Node_type::new(self.provider)
    }
    /// Get storage_pool resource handler
    pub fn storage_pool(&self) -> resources::Storage_pool<'_> {
        resources::Storage_pool::new(self.provider)
    }
    /// Get security_policie resource handler
    pub fn security_policie(&self) -> resources::Security_policie<'_> {
        resources::Security_policie::new(self.provider)
    }
    /// Get region_instance_group_manager_resize_request resource handler
    pub fn region_instance_group_manager_resize_request(&self) -> resources::Region_instance_group_manager_resize_request<'_> {
        resources::Region_instance_group_manager_resize_request::new(self.provider)
    }
    /// Get public_delegated_prefixe resource handler
    pub fn public_delegated_prefixe(&self) -> resources::Public_delegated_prefixe<'_> {
        resources::Public_delegated_prefixe::new(self.provider)
    }
    /// Get license resource handler
    pub fn license(&self) -> resources::License<'_> {
        resources::License::new(self.provider)
    }
    /// Get network_edge_security_service resource handler
    pub fn network_edge_security_service(&self) -> resources::Network_edge_security_service<'_> {
        resources::Network_edge_security_service::new(self.provider)
    }
    /// Get target_vpn_gateway resource handler
    pub fn target_vpn_gateway(&self) -> resources::Target_vpn_gateway<'_> {
        resources::Target_vpn_gateway::new(self.provider)
    }
    /// Get snapshot_setting resource handler
    pub fn snapshot_setting(&self) -> resources::Snapshot_setting<'_> {
        resources::Snapshot_setting::new(self.provider)
    }
    /// Get health_check resource handler
    pub fn health_check(&self) -> resources::Health_check<'_> {
        resources::Health_check::new(self.provider)
    }
    /// Get reservation_sub_block resource handler
    pub fn reservation_sub_block(&self) -> resources::Reservation_sub_block<'_> {
        resources::Reservation_sub_block::new(self.provider)
    }
    /// Get disk_type resource handler
    pub fn disk_type(&self) -> resources::Disk_type<'_> {
        resources::Disk_type::new(self.provider)
    }
    /// Get region_disk resource handler
    pub fn region_disk(&self) -> resources::Region_disk<'_> {
        resources::Region_disk::new(self.provider)
    }
    /// Get region_network_policie resource handler
    pub fn region_network_policie(&self) -> resources::Region_network_policie<'_> {
        resources::Region_network_policie::new(self.provider)
    }
    /// Get zone_operation resource handler
    pub fn zone_operation(&self) -> resources::Zone_operation<'_> {
        resources::Zone_operation::new(self.provider)
    }
    /// Get region_operation resource handler
    pub fn region_operation(&self) -> resources::Region_operation<'_> {
        resources::Region_operation::new(self.provider)
    }
    /// Get image_family_view resource handler
    pub fn image_family_view(&self) -> resources::Image_family_view<'_> {
        resources::Image_family_view::new(self.provider)
    }
    /// Get region_instance_group_manager resource handler
    pub fn region_instance_group_manager(&self) -> resources::Region_instance_group_manager<'_> {
        resources::Region_instance_group_manager::new(self.provider)
    }
    /// Get region_instance resource handler
    pub fn region_instance(&self) -> resources::Region_instance<'_> {
        resources::Region_instance::new(self.provider)
    }
    /// Get interconnect_group resource handler
    pub fn interconnect_group(&self) -> resources::Interconnect_group<'_> {
        resources::Interconnect_group::new(self.provider)
    }
    /// Get target_pool resource handler
    pub fn target_pool(&self) -> resources::Target_pool<'_> {
        resources::Target_pool::new(self.provider)
    }
    /// Get forwarding_rule resource handler
    pub fn forwarding_rule(&self) -> resources::Forwarding_rule<'_> {
        resources::Forwarding_rule::new(self.provider)
    }
    /// Get region_target_http_proxie resource handler
    pub fn region_target_http_proxie(&self) -> resources::Region_target_http_proxie<'_> {
        resources::Region_target_http_proxie::new(self.provider)
    }
    /// Get backend_bucket resource handler
    pub fn backend_bucket(&self) -> resources::Backend_bucket<'_> {
        resources::Backend_bucket::new(self.provider)
    }
    /// Get route resource handler
    pub fn route(&self) -> resources::Route<'_> {
        resources::Route::new(self.provider)
    }
    /// Get network_attachment resource handler
    pub fn network_attachment(&self) -> resources::Network_attachment<'_> {
        resources::Network_attachment::new(self.provider)
    }
    /// Get machine_type resource handler
    pub fn machine_type(&self) -> resources::Machine_type<'_> {
        resources::Machine_type::new(self.provider)
    }
    /// Get region_multi_mig resource handler
    pub fn region_multi_mig(&self) -> resources::Region_multi_mig<'_> {
        resources::Region_multi_mig::new(self.provider)
    }
    /// Get region_snapshot resource handler
    pub fn region_snapshot(&self) -> resources::Region_snapshot<'_> {
        resources::Region_snapshot::new(self.provider)
    }
    /// Get node_template resource handler
    pub fn node_template(&self) -> resources::Node_template<'_> {
        resources::Node_template::new(self.provider)
    }
    /// Get http_health_check resource handler
    pub fn http_health_check(&self) -> resources::Http_health_check<'_> {
        resources::Http_health_check::new(self.provider)
    }
    /// Get region_health_check resource handler
    pub fn region_health_check(&self) -> resources::Region_health_check<'_> {
        resources::Region_health_check::new(self.provider)
    }
    /// Get accelerator_type resource handler
    pub fn accelerator_type(&self) -> resources::Accelerator_type<'_> {
        resources::Accelerator_type::new(self.provider)
    }
    /// Get global_operation resource handler
    pub fn global_operation(&self) -> resources::Global_operation<'_> {
        resources::Global_operation::new(self.provider)
    }
    /// Get region_backend_bucket resource handler
    pub fn region_backend_bucket(&self) -> resources::Region_backend_bucket<'_> {
        resources::Region_backend_bucket::new(self.provider)
    }
    /// Get region_security_policie resource handler
    pub fn region_security_policie(&self) -> resources::Region_security_policie<'_> {
        resources::Region_security_policie::new(self.provider)
    }
    /// Get storage_pool_type resource handler
    pub fn storage_pool_type(&self) -> resources::Storage_pool_type<'_> {
        resources::Storage_pool_type::new(self.provider)
    }
    /// Get reservation_block resource handler
    pub fn reservation_block(&self) -> resources::Reservation_block<'_> {
        resources::Reservation_block::new(self.provider)
    }
    /// Get global_forwarding_rule resource handler
    pub fn global_forwarding_rule(&self) -> resources::Global_forwarding_rule<'_> {
        resources::Global_forwarding_rule::new(self.provider)
    }
    /// Get license_code resource handler
    pub fn license_code(&self) -> resources::License_code<'_> {
        resources::License_code::new(self.provider)
    }
    /// Get disk_setting resource handler
    pub fn disk_setting(&self) -> resources::Disk_setting<'_> {
        resources::Disk_setting::new(self.provider)
    }
    /// Get project resource handler
    pub fn project(&self) -> resources::Project<'_> {
        resources::Project::new(self.provider)
    }
    /// Get firewall_policie resource handler
    pub fn firewall_policie(&self) -> resources::Firewall_policie<'_> {
        resources::Firewall_policie::new(self.provider)
    }
    /// Get snapshot resource handler
    pub fn snapshot(&self) -> resources::Snapshot<'_> {
        resources::Snapshot::new(self.provider)
    }
    /// Get region_network_endpoint_group resource handler
    pub fn region_network_endpoint_group(&self) -> resources::Region_network_endpoint_group<'_> {
        resources::Region_network_endpoint_group::new(self.provider)
    }
    /// Get target_tcp_proxie resource handler
    pub fn target_tcp_proxie(&self) -> resources::Target_tcp_proxie<'_> {
        resources::Target_tcp_proxie::new(self.provider)
    }
    /// Get future_reservation resource handler
    pub fn future_reservation(&self) -> resources::Future_reservation<'_> {
        resources::Future_reservation::new(self.provider)
    }
    /// Get region_instant_snapshot resource handler
    pub fn region_instant_snapshot(&self) -> resources::Region_instant_snapshot<'_> {
        resources::Region_instant_snapshot::new(self.provider)
    }
    /// Get region_disk_setting resource handler
    pub fn region_disk_setting(&self) -> resources::Region_disk_setting<'_> {
        resources::Region_disk_setting::new(self.provider)
    }
    /// Get preview_feature resource handler
    pub fn preview_feature(&self) -> resources::Preview_feature<'_> {
        resources::Preview_feature::new(self.provider)
    }
    /// Get region_instance_template resource handler
    pub fn region_instance_template(&self) -> resources::Region_instance_template<'_> {
        resources::Region_instance_template::new(self.provider)
    }
    /// Get network_profile resource handler
    pub fn network_profile(&self) -> resources::Network_profile<'_> {
        resources::Network_profile::new(self.provider)
    }
    /// Get region_backend_service resource handler
    pub fn region_backend_service(&self) -> resources::Region_backend_service<'_> {
        resources::Region_backend_service::new(self.provider)
    }
    /// Get disk resource handler
    pub fn disk(&self) -> resources::Disk<'_> {
        resources::Disk::new(self.provider)
    }
    /// Get instance_group_manager_resize_request resource handler
    pub fn instance_group_manager_resize_request(&self) -> resources::Instance_group_manager_resize_request<'_> {
        resources::Instance_group_manager_resize_request::new(self.provider)
    }
    /// Get advice resource handler
    pub fn advice(&self) -> resources::Advice<'_> {
        resources::Advice::new(self.provider)
    }
    /// Get node_group resource handler
    pub fn node_group(&self) -> resources::Node_group<'_> {
        resources::Node_group::new(self.provider)
    }
    /// Get instance_group_manager resource handler
    pub fn instance_group_manager(&self) -> resources::Instance_group_manager<'_> {
        resources::Instance_group_manager::new(self.provider)
    }
    /// Get region_disk_type resource handler
    pub fn region_disk_type(&self) -> resources::Region_disk_type<'_> {
        resources::Region_disk_type::new(self.provider)
    }
    /// Get https_health_check resource handler
    pub fn https_health_check(&self) -> resources::Https_health_check<'_> {
        resources::Https_health_check::new(self.provider)
    }
    /// Get interconnect resource handler
    pub fn interconnect(&self) -> resources::Interconnect<'_> {
        resources::Interconnect::new(self.provider)
    }
    /// Get instance_setting resource handler
    pub fn instance_setting(&self) -> resources::Instance_setting<'_> {
        resources::Instance_setting::new(self.provider)
    }
    /// Get network_firewall_policie resource handler
    pub fn network_firewall_policie(&self) -> resources::Network_firewall_policie<'_> {
        resources::Network_firewall_policie::new(self.provider)
    }
    /// Get global_vm_extension_policie resource handler
    pub fn global_vm_extension_policie(&self) -> resources::Global_vm_extension_policie<'_> {
        resources::Global_vm_extension_policie::new(self.provider)
    }
    /// Get organization_security_policie resource handler
    pub fn organization_security_policie(&self) -> resources::Organization_security_policie<'_> {
        resources::Organization_security_policie::new(self.provider)
    }
    /// Get reservation resource handler
    pub fn reservation(&self) -> resources::Reservation<'_> {
        resources::Reservation::new(self.provider)
    }
    /// Get region_health_source resource handler
    pub fn region_health_source(&self) -> resources::Region_health_source<'_> {
        resources::Region_health_source::new(self.provider)
    }
    /// Get packet_mirroring resource handler
    pub fn packet_mirroring(&self) -> resources::Packet_mirroring<'_> {
        resources::Packet_mirroring::new(self.provider)
    }
    /// Get region_commitment resource handler
    pub fn region_commitment(&self) -> resources::Region_commitment<'_> {
        resources::Region_commitment::new(self.provider)
    }
    /// Get global_network_endpoint_group resource handler
    pub fn global_network_endpoint_group(&self) -> resources::Global_network_endpoint_group<'_> {
        resources::Global_network_endpoint_group::new(self.provider)
    }
    /// Get wire_group resource handler
    pub fn wire_group(&self) -> resources::Wire_group<'_> {
        resources::Wire_group::new(self.provider)
    }
    /// Get region_composite_health_check resource handler
    pub fn region_composite_health_check(&self) -> resources::Region_composite_health_check<'_> {
        resources::Region_composite_health_check::new(self.provider)
    }
    /// Get instance_group resource handler
    pub fn instance_group(&self) -> resources::Instance_group<'_> {
        resources::Instance_group::new(self.provider)
    }
    /// Get instance_template resource handler
    pub fn instance_template(&self) -> resources::Instance_template<'_> {
        resources::Instance_template::new(self.provider)
    }
    /// Get region_zone resource handler
    pub fn region_zone(&self) -> resources::Region_zone<'_> {
        resources::Region_zone::new(self.provider)
    }
    /// Get service_attachment resource handler
    pub fn service_attachment(&self) -> resources::Service_attachment<'_> {
        resources::Service_attachment::new(self.provider)
    }
    /// Get interconnect_attachment resource handler
    pub fn interconnect_attachment(&self) -> resources::Interconnect_attachment<'_> {
        resources::Interconnect_attachment::new(self.provider)
    }
    /// Get vpn_tunnel resource handler
    pub fn vpn_tunnel(&self) -> resources::Vpn_tunnel<'_> {
        resources::Vpn_tunnel::new(self.provider)
    }
    /// Get target_grpc_proxie resource handler
    pub fn target_grpc_proxie(&self) -> resources::Target_grpc_proxie<'_> {
        resources::Target_grpc_proxie::new(self.provider)
    }
    /// Get instance resource handler
    pub fn instance(&self) -> resources::Instance<'_> {
        resources::Instance::new(self.provider)
    }
    /// Get global_organization_operation resource handler
    pub fn global_organization_operation(&self) -> resources::Global_organization_operation<'_> {
        resources::Global_organization_operation::new(self.provider)
    }
    /// Get network_endpoint_group resource handler
    pub fn network_endpoint_group(&self) -> resources::Network_endpoint_group<'_> {
        resources::Network_endpoint_group::new(self.provider)
    }
    /// Get url_map resource handler
    pub fn url_map(&self) -> resources::Url_map<'_> {
        resources::Url_map::new(self.provider)
    }
    /// Get interconnect_attachment_group resource handler
    pub fn interconnect_attachment_group(&self) -> resources::Interconnect_attachment_group<'_> {
        resources::Interconnect_attachment_group::new(self.provider)
    }
    /// Get network resource handler
    pub fn network(&self) -> resources::Network<'_> {
        resources::Network::new(self.provider)
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
