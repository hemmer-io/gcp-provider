//! Osconfig_api service for Gcp provider
//!
//! This module handles all osconfig_api resources and their CRUD operations.

use hemmer_core::Result;
use hemmer_provider::{ResourceInput, ResourceOutput, ResourcePlan};

/// Osconfig_api service handler
pub struct Osconfig_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Osconfig_apiService<'a> {
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
            "operation" => self.plan_operation(current_state, desired_input).await,
            "inventorie" => self.plan_inventorie(current_state, desired_input).await,
            "vulnerability_report" => {
                self.plan_vulnerability_report(current_state, desired_input)
                    .await
            }
            "patch_deployment" => {
                self.plan_patch_deployment(current_state, desired_input)
                    .await
            }
            "report" => self.plan_report(current_state, desired_input).await,
            "os_policy_assignment" => {
                self.plan_os_policy_assignment(current_state, desired_input)
                    .await
            }
            "instance_detail" => {
                self.plan_instance_detail(current_state, desired_input)
                    .await
            }
            "global" => self.plan_global(current_state, desired_input).await,
            "patch_job" => self.plan_patch_job(current_state, desired_input).await,
            "report" => self.plan_report(current_state, desired_input).await,
            "operation" => self.plan_operation(current_state, desired_input).await,
            "instance_os_policies_compliance" => {
                self.plan_instance_os_policies_compliance(current_state, desired_input)
                    .await
            }
            "vulnerability_report" => {
                self.plan_vulnerability_report(current_state, desired_input)
                    .await
            }
            "os_policy_assignment" => {
                self.plan_os_policy_assignment(current_state, desired_input)
                    .await
            }
            "inventorie" => self.plan_inventorie(current_state, desired_input).await,
            "policy_orchestrator" => {
                self.plan_policy_orchestrator(current_state, desired_input)
                    .await
            }
            "operation" => self.plan_operation(current_state, desired_input).await,
            "operation" => self.plan_operation(current_state, desired_input).await,
            "policy_orchestrator" => {
                self.plan_policy_orchestrator(current_state, desired_input)
                    .await
            }
            "patch_job" => self.plan_patch_job(current_state, desired_input).await,
            "instance_detail" => {
                self.plan_instance_detail(current_state, desired_input)
                    .await
            }
            "patch_deployment" => {
                self.plan_patch_deployment(current_state, desired_input)
                    .await
            }
            "instance" => self.plan_instance(current_state, desired_input).await,
            "guest_policie" => self.plan_guest_policie(current_state, desired_input).await,
            _ => Err(hemmer_core::HemmerError::Provider(format!(
                "Unknown resource type: {}.{}",
                "osconfig_api", resource_name
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
            "operation" => self.create_operation(input).await,
            "inventorie" => self.create_inventorie(input).await,
            "vulnerability_report" => self.create_vulnerability_report(input).await,
            "patch_deployment" => self.create_patch_deployment(input).await,
            "report" => self.create_report(input).await,
            "os_policy_assignment" => self.create_os_policy_assignment(input).await,
            "instance_detail" => self.create_instance_detail(input).await,
            "global" => self.create_global(input).await,
            "patch_job" => self.create_patch_job(input).await,
            "report" => self.create_report(input).await,
            "operation" => self.create_operation(input).await,
            "instance_os_policies_compliance" => {
                self.create_instance_os_policies_compliance(input).await
            }
            "vulnerability_report" => self.create_vulnerability_report(input).await,
            "os_policy_assignment" => self.create_os_policy_assignment(input).await,
            "inventorie" => self.create_inventorie(input).await,
            "policy_orchestrator" => self.create_policy_orchestrator(input).await,
            "operation" => self.create_operation(input).await,
            "operation" => self.create_operation(input).await,
            "policy_orchestrator" => self.create_policy_orchestrator(input).await,
            "patch_job" => self.create_patch_job(input).await,
            "instance_detail" => self.create_instance_detail(input).await,
            "patch_deployment" => self.create_patch_deployment(input).await,
            "instance" => self.create_instance(input).await,
            "guest_policie" => self.create_guest_policie(input).await,
            _ => Err(hemmer_core::HemmerError::Provider(format!(
                "Unknown resource type: {}.{}",
                "osconfig_api", resource_name
            ))),
        }
    }

    /// Read resource state
    pub async fn read_resource(&self, resource_name: &str, id: &str) -> Result<ResourceOutput> {
        match resource_name {
            "operation" => self.read_operation(id).await,
            "inventorie" => self.read_inventorie(id).await,
            "vulnerability_report" => self.read_vulnerability_report(id).await,
            "patch_deployment" => self.read_patch_deployment(id).await,
            "report" => self.read_report(id).await,
            "os_policy_assignment" => self.read_os_policy_assignment(id).await,
            "instance_detail" => self.read_instance_detail(id).await,
            "global" => self.read_global(id).await,
            "patch_job" => self.read_patch_job(id).await,
            "report" => self.read_report(id).await,
            "operation" => self.read_operation(id).await,
            "instance_os_policies_compliance" => {
                self.read_instance_os_policies_compliance(id).await
            }
            "vulnerability_report" => self.read_vulnerability_report(id).await,
            "os_policy_assignment" => self.read_os_policy_assignment(id).await,
            "inventorie" => self.read_inventorie(id).await,
            "policy_orchestrator" => self.read_policy_orchestrator(id).await,
            "operation" => self.read_operation(id).await,
            "operation" => self.read_operation(id).await,
            "policy_orchestrator" => self.read_policy_orchestrator(id).await,
            "patch_job" => self.read_patch_job(id).await,
            "instance_detail" => self.read_instance_detail(id).await,
            "patch_deployment" => self.read_patch_deployment(id).await,
            "instance" => self.read_instance(id).await,
            "guest_policie" => self.read_guest_policie(id).await,
            _ => Err(hemmer_core::HemmerError::Provider(format!(
                "Unknown resource type: {}.{}",
                "osconfig_api", resource_name
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
            "operation" => self.update_operation(id, input).await,
            "inventorie" => self.update_inventorie(id, input).await,
            "vulnerability_report" => self.update_vulnerability_report(id, input).await,
            "patch_deployment" => self.update_patch_deployment(id, input).await,
            "report" => self.update_report(id, input).await,
            "os_policy_assignment" => self.update_os_policy_assignment(id, input).await,
            "instance_detail" => self.update_instance_detail(id, input).await,
            "global" => self.update_global(id, input).await,
            "patch_job" => self.update_patch_job(id, input).await,
            "report" => self.update_report(id, input).await,
            "operation" => self.update_operation(id, input).await,
            "instance_os_policies_compliance" => {
                self.update_instance_os_policies_compliance(id, input).await
            }
            "vulnerability_report" => self.update_vulnerability_report(id, input).await,
            "os_policy_assignment" => self.update_os_policy_assignment(id, input).await,
            "inventorie" => self.update_inventorie(id, input).await,
            "policy_orchestrator" => self.update_policy_orchestrator(id, input).await,
            "operation" => self.update_operation(id, input).await,
            "operation" => self.update_operation(id, input).await,
            "policy_orchestrator" => self.update_policy_orchestrator(id, input).await,
            "patch_job" => self.update_patch_job(id, input).await,
            "instance_detail" => self.update_instance_detail(id, input).await,
            "patch_deployment" => self.update_patch_deployment(id, input).await,
            "instance" => self.update_instance(id, input).await,
            "guest_policie" => self.update_guest_policie(id, input).await,
            _ => Err(hemmer_core::HemmerError::Provider(format!(
                "Unknown resource type: {}.{}",
                "osconfig_api", resource_name
            ))),
        }
    }

    /// Delete a resource
    pub async fn delete_resource(&self, resource_name: &str, id: &str) -> Result<()> {
        match resource_name {
            "operation" => self.delete_operation(id).await,
            "inventorie" => self.delete_inventorie(id).await,
            "vulnerability_report" => self.delete_vulnerability_report(id).await,
            "patch_deployment" => self.delete_patch_deployment(id).await,
            "report" => self.delete_report(id).await,
            "os_policy_assignment" => self.delete_os_policy_assignment(id).await,
            "instance_detail" => self.delete_instance_detail(id).await,
            "global" => self.delete_global(id).await,
            "patch_job" => self.delete_patch_job(id).await,
            "report" => self.delete_report(id).await,
            "operation" => self.delete_operation(id).await,
            "instance_os_policies_compliance" => {
                self.delete_instance_os_policies_compliance(id).await
            }
            "vulnerability_report" => self.delete_vulnerability_report(id).await,
            "os_policy_assignment" => self.delete_os_policy_assignment(id).await,
            "inventorie" => self.delete_inventorie(id).await,
            "policy_orchestrator" => self.delete_policy_orchestrator(id).await,
            "operation" => self.delete_operation(id).await,
            "operation" => self.delete_operation(id).await,
            "policy_orchestrator" => self.delete_policy_orchestrator(id).await,
            "patch_job" => self.delete_patch_job(id).await,
            "instance_detail" => self.delete_instance_detail(id).await,
            "patch_deployment" => self.delete_patch_deployment(id).await,
            "instance" => self.delete_instance(id).await,
            "guest_policie" => self.delete_guest_policie(id).await,
            _ => Err(hemmer_core::HemmerError::Provider(format!(
                "Unknown resource type: {}.{}",
                "osconfig_api", resource_name
            ))),
        }
    }

    // ========================================================================
    // Resource-specific CRUD implementations
    // ========================================================================

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
    // Inventorie resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a inventorie resource
    async fn plan_inventorie(
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

    /// Create a new inventorie resource
    async fn create_inventorie(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a inventorie resource
    async fn read_inventorie(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a inventorie resource
    async fn update_inventorie(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a inventorie resource
    async fn delete_inventorie(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Vulnerability_report resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a vulnerability_report resource
    async fn plan_vulnerability_report(
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

    /// Create a new vulnerability_report resource
    async fn create_vulnerability_report(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a vulnerability_report resource
    async fn read_vulnerability_report(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a vulnerability_report resource
    async fn update_vulnerability_report(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a vulnerability_report resource
    async fn delete_vulnerability_report(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Patch_deployment resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a patch_deployment resource
    async fn plan_patch_deployment(
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

    /// Create a new patch_deployment resource
    async fn create_patch_deployment(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a patch_deployment resource
    async fn read_patch_deployment(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a patch_deployment resource
    async fn update_patch_deployment(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a patch_deployment resource
    async fn delete_patch_deployment(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Report resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a report resource
    async fn plan_report(
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

    /// Create a new report resource
    async fn create_report(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a report resource
    async fn read_report(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a report resource
    async fn update_report(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a report resource
    async fn delete_report(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Os_policy_assignment resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a os_policy_assignment resource
    async fn plan_os_policy_assignment(
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

    /// Create a new os_policy_assignment resource
    async fn create_os_policy_assignment(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a os_policy_assignment resource
    async fn read_os_policy_assignment(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a os_policy_assignment resource
    async fn update_os_policy_assignment(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a os_policy_assignment resource
    async fn delete_os_policy_assignment(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Instance_detail resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a instance_detail resource
    async fn plan_instance_detail(
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

    /// Create a new instance_detail resource
    async fn create_instance_detail(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a instance_detail resource
    async fn read_instance_detail(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a instance_detail resource
    async fn update_instance_detail(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a instance_detail resource
    async fn delete_instance_detail(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Global resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a global resource
    async fn plan_global(
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

    /// Create a new global resource
    async fn create_global(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a global resource
    async fn read_global(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a global resource
    async fn update_global(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a global resource
    async fn delete_global(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Patch_job resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a patch_job resource
    async fn plan_patch_job(
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

    /// Create a new patch_job resource
    async fn create_patch_job(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a patch_job resource
    async fn read_patch_job(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a patch_job resource
    async fn update_patch_job(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a patch_job resource
    async fn delete_patch_job(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Report resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a report resource
    async fn plan_report(
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

    /// Create a new report resource
    async fn create_report(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a report resource
    async fn read_report(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a report resource
    async fn update_report(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a report resource
    async fn delete_report(&self, id: &str) -> Result<()> {
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
    // Instance_os_policies_compliance resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a instance_os_policies_compliance resource
    async fn plan_instance_os_policies_compliance(
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

    /// Create a new instance_os_policies_compliance resource
    async fn create_instance_os_policies_compliance(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a instance_os_policies_compliance resource
    async fn read_instance_os_policies_compliance(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a instance_os_policies_compliance resource
    async fn update_instance_os_policies_compliance(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a instance_os_policies_compliance resource
    async fn delete_instance_os_policies_compliance(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Vulnerability_report resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a vulnerability_report resource
    async fn plan_vulnerability_report(
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

    /// Create a new vulnerability_report resource
    async fn create_vulnerability_report(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a vulnerability_report resource
    async fn read_vulnerability_report(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a vulnerability_report resource
    async fn update_vulnerability_report(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a vulnerability_report resource
    async fn delete_vulnerability_report(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Os_policy_assignment resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a os_policy_assignment resource
    async fn plan_os_policy_assignment(
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

    /// Create a new os_policy_assignment resource
    async fn create_os_policy_assignment(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a os_policy_assignment resource
    async fn read_os_policy_assignment(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a os_policy_assignment resource
    async fn update_os_policy_assignment(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a os_policy_assignment resource
    async fn delete_os_policy_assignment(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Inventorie resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a inventorie resource
    async fn plan_inventorie(
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

    /// Create a new inventorie resource
    async fn create_inventorie(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a inventorie resource
    async fn read_inventorie(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a inventorie resource
    async fn update_inventorie(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a inventorie resource
    async fn delete_inventorie(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Policy_orchestrator resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a policy_orchestrator resource
    async fn plan_policy_orchestrator(
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

    /// Create a new policy_orchestrator resource
    async fn create_policy_orchestrator(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a policy_orchestrator resource
    async fn read_policy_orchestrator(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a policy_orchestrator resource
    async fn update_policy_orchestrator(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a policy_orchestrator resource
    async fn delete_policy_orchestrator(&self, id: &str) -> Result<()> {
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
    // Policy_orchestrator resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a policy_orchestrator resource
    async fn plan_policy_orchestrator(
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

    /// Create a new policy_orchestrator resource
    async fn create_policy_orchestrator(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a policy_orchestrator resource
    async fn read_policy_orchestrator(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a policy_orchestrator resource
    async fn update_policy_orchestrator(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a policy_orchestrator resource
    async fn delete_policy_orchestrator(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Patch_job resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a patch_job resource
    async fn plan_patch_job(
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

    /// Create a new patch_job resource
    async fn create_patch_job(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a patch_job resource
    async fn read_patch_job(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a patch_job resource
    async fn update_patch_job(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a patch_job resource
    async fn delete_patch_job(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Instance_detail resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a instance_detail resource
    async fn plan_instance_detail(
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

    /// Create a new instance_detail resource
    async fn create_instance_detail(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a instance_detail resource
    async fn read_instance_detail(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a instance_detail resource
    async fn update_instance_detail(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a instance_detail resource
    async fn delete_instance_detail(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Patch_deployment resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a patch_deployment resource
    async fn plan_patch_deployment(
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

    /// Create a new patch_deployment resource
    async fn create_patch_deployment(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a patch_deployment resource
    async fn read_patch_deployment(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a patch_deployment resource
    async fn update_patch_deployment(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a patch_deployment resource
    async fn delete_patch_deployment(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Instance resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a instance resource
    async fn plan_instance(
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

    /// Create a new instance resource
    async fn create_instance(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a instance resource
    async fn read_instance(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a instance resource
    async fn update_instance(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a instance resource
    async fn delete_instance(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Guest_policie resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a guest_policie resource
    async fn plan_guest_policie(
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

    /// Create a new guest_policie resource
    async fn create_guest_policie(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a guest_policie resource
    async fn read_guest_policie(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a guest_policie resource
    async fn update_guest_policie(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a guest_policie resource
    async fn delete_guest_policie(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }
}
