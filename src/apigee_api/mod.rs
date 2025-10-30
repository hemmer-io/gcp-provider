//! Apigee_api Service
//!
//! Auto-generated service module for apigee_api

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for apigee_api
pub struct Apigee_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Apigee_apiService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get organization resource handler
    pub fn organization(&self) -> resources::Organization<'_> {
        resources::Organization::new(self.provider)
    }
    /// Get developer resource handler
    pub fn developer(&self) -> resources::Developer<'_> {
        resources::Developer::new(self.provider)
    }
    /// Get security_profile resource handler
    pub fn security_profile(&self) -> resources::Security_profile<'_> {
        resources::Security_profile::new(self.provider)
    }
    /// Get appgroup resource handler
    pub fn appgroup(&self) -> resources::Appgroup<'_> {
        resources::Appgroup::new(self.provider)
    }
    /// Get admin resource handler
    pub fn admin(&self) -> resources::Admin<'_> {
        resources::Admin::new(self.provider)
    }
    /// Get canaryevaluation resource handler
    pub fn canaryevaluation(&self) -> resources::Canaryevaluation<'_> {
        resources::Canaryevaluation::new(self.provider)
    }
    /// Get revision resource handler
    pub fn revision(&self) -> resources::Revision<'_> {
        resources::Revision::new(self.provider)
    }
    /// Get app resource handler
    pub fn app(&self) -> resources::App<'_> {
        resources::App::new(self.provider)
    }
    /// Get security_report resource handler
    pub fn security_report(&self) -> resources::Security_report<'_> {
        resources::Security_report::new(self.provider)
    }
    /// Get deployment resource handler
    pub fn deployment(&self) -> resources::Deployment<'_> {
        resources::Deployment::new(self.provider)
    }
    /// Get entrie resource handler
    pub fn entrie(&self) -> resources::Entrie<'_> {
        resources::Entrie::new(self.provider)
    }
    /// Get cache resource handler
    pub fn cache(&self) -> resources::Cache<'_> {
        resources::Cache::new(self.provider)
    }
    /// Get attachment resource handler
    pub fn attachment(&self) -> resources::Attachment<'_> {
        resources::Attachment::new(self.provider)
    }
    /// Get override resource handler
    pub fn override(&self) -> resources::Override<'_> {
        resources::Override::new(self.provider)
    }
    /// Get issuer resource handler
    pub fn issuer(&self) -> resources::Issuer<'_> {
        resources::Issuer::new(self.provider)
    }
    /// Get addons_config resource handler
    pub fn addons_config(&self) -> resources::Addons_config<'_> {
        resources::Addons_config::new(self.provider)
    }
    /// Get keystore resource handler
    pub fn keystore(&self) -> resources::Keystore<'_> {
        resources::Keystore::new(self.provider)
    }
    /// Get create resource handler
    pub fn create(&self) -> resources::Create<'_> {
        resources::Create::new(self.provider)
    }
    /// Get apidoc resource handler
    pub fn apidoc(&self) -> resources::Apidoc<'_> {
        resources::Apidoc::new(self.provider)
    }
    /// Get security_assessment_result resource handler
    pub fn security_assessment_result(&self) -> resources::Security_assessment_result<'_> {
        resources::Security_assessment_result::new(self.provider)
    }
    /// Get security_incident resource handler
    pub fn security_incident(&self) -> resources::Security_incident<'_> {
        resources::Security_incident::new(self.provider)
    }
    /// Get balance resource handler
    pub fn balance(&self) -> resources::Balance<'_> {
        resources::Balance::new(self.provider)
    }
    /// Get aliase resource handler
    pub fn aliase(&self) -> resources::Aliase<'_> {
        resources::Aliase::new(self.provider)
    }
    /// Get apiproduct resource handler
    pub fn apiproduct(&self) -> resources::Apiproduct<'_> {
        resources::Apiproduct::new(self.provider)
    }
    /// Get security_profiles_v2 resource handler
    pub fn security_profiles_v2(&self) -> resources::Security_profiles_v2<'_> {
        resources::Security_profiles_v2::new(self.provider)
    }
    /// Get envgroup resource handler
    pub fn envgroup(&self) -> resources::Envgroup<'_> {
        resources::Envgroup::new(self.provider)
    }
    /// Get project resource handler
    pub fn project(&self) -> resources::Project<'_> {
        resources::Project::new(self.provider)
    }
    /// Get datacollector resource handler
    pub fn datacollector(&self) -> resources::Datacollector<'_> {
        resources::Datacollector::new(self.provider)
    }
    /// Get data resource handler
    pub fn data(&self) -> resources::Data<'_> {
        resources::Data::new(self.provider)
    }
    /// Get environment resource handler
    pub fn environment(&self) -> resources::Environment<'_> {
        resources::Environment::new(self.provider)
    }
    /// Get host_querie resource handler
    pub fn host_querie(&self) -> resources::Host_querie<'_> {
        resources::Host_querie::new(self.provider)
    }
    /// Get attribute resource handler
    pub fn attribute(&self) -> resources::Attribute<'_> {
        resources::Attribute::new(self.provider)
    }
    /// Get host_stat resource handler
    pub fn host_stat(&self) -> resources::Host_stat<'_> {
        resources::Host_stat::new(self.provider)
    }
    /// Get security_feedback resource handler
    pub fn security_feedback(&self) -> resources::Security_feedback<'_> {
        resources::Security_feedback::new(self.provider)
    }
    /// Get api resource handler
    pub fn api(&self) -> resources::Api<'_> {
        resources::Api::new(self.provider)
    }
    /// Get dns_zone resource handler
    pub fn dns_zone(&self) -> resources::Dns_zone<'_> {
        resources::Dns_zone::new(self.provider)
    }
    /// Get security_action resource handler
    pub fn security_action(&self) -> resources::Security_action<'_> {
        resources::Security_action::new(self.provider)
    }
    /// Get optimized_stat resource handler
    pub fn optimized_stat(&self) -> resources::Optimized_stat<'_> {
        resources::Optimized_stat::new(self.provider)
    }
    /// Get optimized_host_stat resource handler
    pub fn optimized_host_stat(&self) -> resources::Optimized_host_stat<'_> {
        resources::Optimized_host_stat::new(self.provider)
    }
    /// Get subscription resource handler
    pub fn subscription(&self) -> resources::Subscription<'_> {
        resources::Subscription::new(self.provider)
    }
    /// Get host_security_report resource handler
    pub fn host_security_report(&self) -> resources::Host_security_report<'_> {
        resources::Host_security_report::new(self.provider)
    }
    /// Get operation resource handler
    pub fn operation(&self) -> resources::Operation<'_> {
        resources::Operation::new(self.provider)
    }
    /// Get space resource handler
    pub fn space(&self) -> resources::Space<'_> {
        resources::Space::new(self.provider)
    }
    /// Get instance resource handler
    pub fn instance(&self) -> resources::Instance<'_> {
        resources::Instance::new(self.provider)
    }
    /// Get sharedflow resource handler
    pub fn sharedflow(&self) -> resources::Sharedflow<'_> {
        resources::Sharedflow::new(self.provider)
    }
    /// Get resourcefile resource handler
    pub fn resourcefile(&self) -> resources::Resourcefile<'_> {
        resources::Resourcefile::new(self.provider)
    }
    /// Get debugsession resource handler
    pub fn debugsession(&self) -> resources::Debugsession<'_> {
        resources::Debugsession::new(self.provider)
    }
    /// Get security_monitoring_condition resource handler
    pub fn security_monitoring_condition(&self) -> resources::Security_monitoring_condition<'_> {
        resources::Security_monitoring_condition::new(self.provider)
    }
    /// Get keyvaluemap resource handler
    pub fn keyvaluemap(&self) -> resources::Keyvaluemap<'_> {
        resources::Keyvaluemap::new(self.provider)
    }
    /// Get datastore resource handler
    pub fn datastore(&self) -> resources::Datastore<'_> {
        resources::Datastore::new(self.provider)
    }
    /// Get nat_addresse resource handler
    pub fn nat_addresse(&self) -> resources::Nat_addresse<'_> {
        resources::Nat_addresse::new(self.provider)
    }
    /// Get endpoint_attachment resource handler
    pub fn endpoint_attachment(&self) -> resources::Endpoint_attachment<'_> {
        resources::Endpoint_attachment::new(self.provider)
    }
    /// Get flowhook resource handler
    pub fn flowhook(&self) -> resources::Flowhook<'_> {
        resources::Flowhook::new(self.provider)
    }
    /// Get key resource handler
    pub fn key(&self) -> resources::Key<'_> {
        resources::Key::new(self.provider)
    }
    /// Get rateplan resource handler
    pub fn rateplan(&self) -> resources::Rateplan<'_> {
        resources::Rateplan::new(self.provider)
    }
    /// Get archive_deployment resource handler
    pub fn archive_deployment(&self) -> resources::Archive_deployment<'_> {
        resources::Archive_deployment::new(self.provider)
    }
    /// Get targetserver resource handler
    pub fn targetserver(&self) -> resources::Targetserver<'_> {
        resources::Targetserver::new(self.provider)
    }
    /// Get stat resource handler
    pub fn stat(&self) -> resources::Stat<'_> {
        resources::Stat::new(self.provider)
    }
    /// Get querie resource handler
    pub fn querie(&self) -> resources::Querie<'_> {
        resources::Querie::new(self.provider)
    }
    /// Get security_stat resource handler
    pub fn security_stat(&self) -> resources::Security_stat<'_> {
        resources::Security_stat::new(self.provider)
    }
    /// Get export resource handler
    pub fn export(&self) -> resources::Export<'_> {
        resources::Export::new(self.provider)
    }
    /// Get apicategorie resource handler
    pub fn apicategorie(&self) -> resources::Apicategorie<'_> {
        resources::Apicategorie::new(self.provider)
    }
    /// Get report resource handler
    pub fn report(&self) -> resources::Report<'_> {
        resources::Report::new(self.provider)
    }
    /// Get reference resource handler
    pub fn reference(&self) -> resources::Reference<'_> {
        resources::Reference::new(self.provider)
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
