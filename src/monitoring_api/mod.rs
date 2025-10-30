//! Monitoring_api Service
//!
//! Auto-generated service module for monitoring_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for monitoring_api
pub struct Monitoring_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Monitoring_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get metric_descriptor resource handler
    pub fn metric_descriptor(&self) -> resources::Metric_descriptor<'_> {
        resources::Metric_descriptor::new(self.provider)
    }
    /// Get snooze resource handler
    pub fn snooze(&self) -> resources::Snooze<'_> {
        resources::Snooze::new(self.provider)
    }
    /// Get group resource handler
    pub fn group(&self) -> resources::Group<'_> {
        resources::Group::new(self.provider)
    }
    /// Get time_serie resource handler
    pub fn time_serie(&self) -> resources::Time_serie<'_> {
        resources::Time_serie::new(self.provider)
    }
    /// Get member resource handler
    pub fn member(&self) -> resources::Member<'_> {
        resources::Member::new(self.provider)
    }
    /// Get uptime_check_config resource handler
    pub fn uptime_check_config(&self) -> resources::Uptime_check_config<'_> {
        resources::Uptime_check_config::new(self.provider)
    }
    /// Get monitored_resource_descriptor resource handler
    pub fn monitored_resource_descriptor(&self) -> resources::Monitored_resource_descriptor<'_> {
        resources::Monitored_resource_descriptor::new(self.provider)
    }
    /// Get uptime_check_ip resource handler
    pub fn uptime_check_ip(&self) -> resources::Uptime_check_ip<'_> {
        resources::Uptime_check_ip::new(self.provider)
    }
    /// Get notification_channel_descriptor resource handler
    pub fn notification_channel_descriptor(&self) -> resources::Notification_channel_descriptor<'_> {
        resources::Notification_channel_descriptor::new(self.provider)
    }
    /// Get service_level_objective resource handler
    pub fn service_level_objective(&self) -> resources::Service_level_objective<'_> {
        resources::Service_level_objective::new(self.provider)
    }
    /// Get alert resource handler
    pub fn alert(&self) -> resources::Alert<'_> {
        resources::Alert::new(self.provider)
    }
    /// Get alert_policie resource handler
    pub fn alert_policie(&self) -> resources::Alert_policie<'_> {
        resources::Alert_policie::new(self.provider)
    }
    /// Get collectd_time_serie resource handler
    pub fn collectd_time_serie(&self) -> resources::Collectd_time_serie<'_> {
        resources::Collectd_time_serie::new(self.provider)
    }
    /// Get notification_channel resource handler
    pub fn notification_channel(&self) -> resources::Notification_channel<'_> {
        resources::Notification_channel::new(self.provider)
    }
    /// Get service resource handler
    pub fn service(&self) -> resources::Service<'_> {
        resources::Service::new(self.provider)
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
